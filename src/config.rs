use std::env;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

pub struct Config {
    pub bot_token: String,
    pub pexels_api_key: String,
    pub total_pics: usize,
    pub headers: HeaderMap,
    pub api_key: String,
    pub api_model: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        let bot_token: String = env::var("RustOpenAiBot_TOKEN")?;
        let pexels_api_key: String = env::var("PEXELS_API_KEY")?;
        let api_key = env::var("OPENAI_API_KEY_").expect("OPENAI_API_KEY not set");
        let api_model = env::var("OPENAI_API_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string());
        
        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&pexels_api_key).unwrap());

        let total_pics: usize = env::var("TOTAL_PICS")?.parse().unwrap_or(1);

        Ok(Self {
            bot_token,
            pexels_api_key,
            total_pics,
            headers,
            api_key,
            api_model,
        })
    }
}
