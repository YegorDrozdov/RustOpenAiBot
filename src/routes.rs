use teloxide::{prelude::*, RequestError};
use teloxide::types::{ChatAction, InputFile, Message};
use crate::config::Config;
use crate::keyboards::{extract_command_from_label, get_main_keyboard, Command};
use crate::openai::OpenAIRecognizer;
use crate::pexels::get_photo_url;
use std::sync::Arc;
use reqwest::{get, Url};
use log::{info, warn, error};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use image::{GenericImageView, ImageFormat};

pub async fn handle_message(
    bot: Bot,
    message: Message,
    config: Arc<Config>,
    commands: Arc<Vec<Command>>,
    openai_recognizer: Arc<tokio::sync::Mutex<OpenAIRecognizer>>,
) -> ResponseResult<()> {
    if let Some(text) = message.text().map(|t| t.to_string()) {
        let username = message.from().and_then(|u| u.username.clone()).unwrap_or_else(|| "unknown".to_string());
        let user_id = message.from().map_or(0, |u| u.id.0);
        let chat_id = message.chat.id;
        let message_id = message.id;
        info!("Received message: {} from {} ({})", &text, user_id, username);

        if &text == "/start" {
            info!("User {} ({}) started the bot.", user_id, username);

            let keyboard = get_main_keyboard(&commands);

            let welcome_msg = bot.send_message(
                chat_id,
                "Hi! I am a bot that can send photos on various themes: \
                dogs, cats, vehicles, underwater scenes, space, nature, and more. \
                Please choose an option from the keyboard below or \
                type your own command starting with /. \
                To refresh the page, enter /start."
            )
            .reply_markup(keyboard)
            .await?;

            bot.pin_chat_message(message.chat.id, welcome_msg.id).await?;
            return Ok(());
        }

        let query: String = if text.starts_with('/') {
            extract_command_from_label(&text)
        } else {
            let command = commands.iter().find(|cmd| cmd.name == text);
            if let Some(cmd) = command {
                cmd.action.clone()
            } else {
                "".to_string()
            }
        };

        if query.is_empty() {
            bot.send_chat_action(chat_id, ChatAction::Typing).await?;
            let openai_recognizer = Arc::clone(&openai_recognizer);
            let text_clone = text.to_string();
            let bot_clone = bot.clone();
            let chat_id_clone = chat_id;

            tokio::spawn(async move {
                let mut recognizer = openai_recognizer.lock().await;
                match recognizer.get_response(&text_clone).await {
                    Ok(response) => {
                        info!("OpenAI response body: {}", &response);
                        if let Err(e) = bot_clone.send_message(chat_id_clone, response).await {
                            warn!("Failed to send OpenAI response: {}", e);
                        }
                    },
                    Err(e) => {
                        warn!("Failed to get a response from OpenAI: {}", e);
                        if let Err(e) = bot_clone.send_message(chat_id_clone, "Failed to get a response from OpenAI.").await {
                            warn!("Failed to send error message: {}", e);
                        }
                    }
                }
            });

            return Ok(());
        }

        tokio::spawn(async move {
            match get_photo_url(&query, Arc::clone(&config)).await {
                Ok(Some(photo_url)) => {
                    let _ = &bot.send_chat_action(chat_id, ChatAction::UploadPhoto).await.unwrap_or_default();
                    info!("Sending photo URL: {} to user {} ({})", &photo_url, user_id, username);

                    match Url::parse(&photo_url) {
                        Ok(url) => {
                            match &bot.send_photo(chat_id, InputFile::url(url.clone()))
                                .caption(format!("Here is {} photo for you!", &text))
                                .await
                            {
                                Ok(_) => info!("Photo sent successfully"),
                                Err(err) => {
                                    warn !("Failed to send photo: {:?}", err);

                                    // Check for PHOTO_INVALID_DIMENSIONS error
                                    // if let RequestError::Api(api_error) = &err {
                                    //     if api_error.to_string().contains("PHOTO_INVALID_DIMENSIONS") || api_error.to_string().contains("FailedToGetUrlContent"){
                                    //         info!("Photo has invalid dimensions or content, proceeding to resize.");

                                    match get(&photo_url).await {
                                        Ok(response) => {
                                            if response.status().is_success() {
                                                // "Your request will take a little more time. We apologize for the inconvenience."
                                                let _ = &bot.send_message(
                                                    chat_id, 
                                                    "Your request will require a bit more time. We apologize for any inconvenience this may cause."
                                                ).reply_to_message_id(message_id).await;
                                                
                                                let bytes = response.bytes().await.unwrap();
                                                let img = image::load_from_memory(&bytes).unwrap();
                                                let (width, height) = img.dimensions();

                                                if width > 2560 || height > 2560 {
                                                    let scale = 2560.0 / width.max(height) as f32;
                                                    let new_width = (width as f32 * scale) as u32;
                                                    let new_height = (height as f32 * scale) as u32;
                                                    let resized_img = img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3);
                                                    
                                                    let file_path = format!("/tmp/{}.jpg", Uuid::new_v4());
                                                    resized_img.save_with_format(&file_path, ImageFormat::Jpeg).unwrap();

                                                    if let Err(err) = &bot.send_photo(chat_id, InputFile::file(file_path.clone()))
                                                            .caption(format!("Here is {} delayed photo. We apologize for the inconvenience.", &text))
                                                            .reply_to_message_id(message_id)
                                                            .await 
                                                            {
                                                        error!("Failed to send udated photo: {:?}", err);
                                                    } else {info!("Updated photo sent successfully")}

                                                    tokio::fs::remove_file(file_path).await.unwrap();
                                                }
                                            } else {
                                                error!("Failed to access URL: {:?}", response.status());
                                                let _ = &bot.send_message(chat_id, "Failed to access the photo URL.").await;
                                            }
                                        },
                                        Err(err) => {
                                            error!("Error accessing URL: {:?}", err);
                                            let _ = &bot.send_message(chat_id, "Failed to access the photo URL.").await;
                                        }
                                    }
                                }
                            }
                        },
                        Err(e) => { error!("Failed to parse URL: {:?}", e) }
                    }
                },
                Ok(None) => {
                    if let Err(e) = &bot.send_message(chat_id, "Sorry, no photos found.").await 
                    {warn!("Failed to send error message: {}", e)}
                },
                Err(err) => {
                    error!("Error getting photo URL: {:?}", err);
                    if let Err(e) = &bot.send_message(chat_id, "Failed to get a photo after 5 attempts. Please try again later.").await 
                    {warn!("Failed to send error message: {}", e)}
                }
            }
        });
    }
    Ok(())
}
