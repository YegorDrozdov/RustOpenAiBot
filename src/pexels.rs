use reqwest::Client;
use reqwest::Url;
use serde_json::Value;
use std::sync::Arc;
use rand::seq::SliceRandom;
use log::{debug, error, warn};
use crate::config::Config;

pub async fn get_photo_url(query: &str, config: Arc<Config>) -> Result<Option<String>, reqwest::Error> {
    let cleared_query = query.replace("_", "");
    let url = format!("https://api.pexels.com/v1/search?query={}&per_page={}", cleared_query, config.total_pics);
    debug!("Requesting URL: {}", url);
    let client = Client::new();

    for attempt in 0..5 {
        match client.get(Url::parse(&url).expect("Invalid URL"))
            .headers(config.headers.clone())
            .send()
            .await {
            Ok(res) => {
                let json: Value = res.json().await?;
                if let Some(photos) = json["photos"].as_array() {
                    if let Some(photo) = photos.choose(&mut rand::thread_rng()) {
                        if let Some(photo_url) = photo["src"]["original"].as_str() {
                            return Ok(Some(photo_url.to_string()));
                        } else {
                            warn!("No photo URL found in response.");
                            return Ok(None);
                        }
                    } else {
                        warn!("No photos found in response.");
                        return Ok(None);
                    }
                } else {
                    error!("Failed to parse 'photos' array from API response.");
                    return Ok(None);
                }
            }
            Err(e) => {
                error!("Attempt {}/5 - Request to Pexels API failed: {}", attempt + 1, e);
                if attempt >= 4 {
                    return Err(e);
                }
            }
        }
    }
    Ok(None)
}
