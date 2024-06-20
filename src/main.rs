#[macro_use]
extern crate diesel;
extern crate dotenv;

mod config;
mod logging;
mod routes;
mod keyboards;
mod pexels;
mod openai;
mod error;
mod types;
mod models;
mod schema;
mod database;
mod user;
mod custom_json;

use openai::OpenAIRecognizer;
use teloxide::prelude::*;
use config::Config;
use logging::init_logger;
use std::sync::Arc;
use database::establish_connection_pool;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let _ = init_logger();
    let pool = Arc::new(establish_connection_pool());
    let config = Arc::new(Config::from_env().expect("Failed to load configuration"));
    let commands = Arc::new(keyboards::load_commands_with_emoji("commands.yaml"));
    let openai_recognizer = Arc::new(
        tokio::sync::Mutex::new(OpenAIRecognizer::new(config.api_key.clone(), config.api_model.clone()))
    );

    let bot = Bot::new(config.bot_token.clone());

    teloxide::repl(bot.clone(), move |message: Message| {
        let bot = bot.clone();
        let config = Arc::clone(&config);
        let commands = Arc::clone(&commands);
        let openai_recognizer = Arc::clone(&openai_recognizer);
        let pool = Arc::clone(&pool);

        async move {
            let _ = routes::handle_message(bot, message, config, commands, openai_recognizer, pool).await;
            respond(())
        }
    })
    .await;
}
