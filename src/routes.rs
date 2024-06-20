use teloxide::prelude::*;
use teloxide::types::{ChatAction, InputFile, Message};
use crate::config::Config;
use crate::keyboards::{extract_command_from_label, get_main_keyboard, Command};
use crate::openai::OpenAIRecognizer;
use crate::pexels::get_photo_url;
// use crate::user::handle_user;
use std::sync::Arc;
use reqwest::Url;
use log::{info, warn};
use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn handle_message(
    bot: Bot,
    message: Message,
    config: Arc<Config>,
    commands: Arc<Vec<Command>>,
    openai_recognizer: Arc<tokio::sync::Mutex<OpenAIRecognizer>>,
    pool: Arc<DbPool>,
) -> ResponseResult<()> {
    if let Some(text) = message.text() {
        let username: String = message.from().and_then(|u| u.username.clone()).unwrap_or_else(|| "unknown".to_string());
        let user_id = message.from().map_or(0, |u| u.id.0);
        let chat_id = message.chat.id;
        info!("Received message: {} from {} ({})", text, user_id, username);

        // match handle_user(Arc::clone(&pool), &message).await {
        //     Ok((pk)) => info!("User saved with pk: {}, user_id: {}", pk, user_id),
        //     Err(e) => warn!("Error saving user: {}", e),
        // }

        // Handle /start command
        if text == "/start" {
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

        // Handle text messages starting with '/'
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
            let mut recognizer = openai_recognizer.lock().await;
            match recognizer.get_response(text).await {
                Ok(response) => {
                    info!("OpenAI response body: {}", &response);
                    bot.send_message(message.chat.id, response).await?;
                },
                Err(e) => {
                    warn!("Failed to get a response from OpenAI: {}", e);
                    bot.send_message(message.chat.id, "Failed to get a response from OpenAI.").await?;
                }
            }

            return Ok(());
        }

        // Fetch photo URL from Pexels API
        match get_photo_url(&query, Arc::clone(&config)).await {
            Ok(Some(photo_url)) => {
                bot.send_chat_action(chat_id, ChatAction::UploadPhoto).await?;
                let clean_query = query.trim_start_matches('/').replace("_", "").to_string();
                let msg = remove_extra_spaces(&format!("Here is the {} photo for you!", clean_query));
                info!("Sending photo URL: {} to user {} ({})", photo_url, user_id, username);
                bot.send_photo(message.chat.id, InputFile::url(Url::parse(&photo_url).expect("Invalid URL")))
                    .caption(msg)
                    .await?;
            }
            Ok(None) => {
                bot.send_message(message.chat.id, "Sorry, no photos found.")
                    .await?;
            }
            Err(_) => {
                bot.send_message(message.chat.id, "Failed to get a photo after 5 attempts. Please try again later.")
                    .await?;
            }
        }
    }
    Ok(())
}

fn remove_extra_spaces(s: &str) -> String {
    s.split_whitespace().collect::<Vec<&str>>().join(" ")
}
