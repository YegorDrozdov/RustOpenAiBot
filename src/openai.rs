use reqwest::Client;
use log::{error, debug};
use crate::types::{OpenAIRequest, OpenAIResponse, OpenAIErrorResponse, Message};
use crate::error::OpenAIError;

#[derive(Clone)]
pub struct OpenAIRecognizer {
    client: Client,
    api_key: String,
    model: String,
    msg_history: Vec<Message>,
}

impl OpenAIRecognizer {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model,
            msg_history: vec![Message {
                role: "system".to_string(),
                content: "You are a helpful assistant.".to_string(),
            }],
        }
    }

    pub async fn get_response(&mut self, prompt: &str) -> Result<String, OpenAIError> {
        self.msg_history.push(Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        });

        let request_body = OpenAIRequest {
            model: self.model.clone(),
            messages: self.msg_history.clone(),
        };

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await
            .map_err(|e| OpenAIError { message: format!("Request error: {}", e) })?;

        let response_text = response.text().await.map_err(|e| OpenAIError { message: format!("Error reading response text: {}", e) })?;
        debug!("OpenAI response body: {}", response_text);

        // Try to parse the response as an error first
        if let Ok(error_response) = serde_json::from_str::<OpenAIErrorResponse>(&response_text) {
            error!("OpenAI API error: {}", error_response.error.message);
            return Err(OpenAIError { message: error_response.error.message });
        }

        // If not an error, try to parse it as a valid response
        let response_body: OpenAIResponse = serde_json::from_str(&response_text)
            .map_err(|e| OpenAIError { message: format!("Error decoding response body: {}", e) })?;

        let response_content = response_body.choices.get(0).map_or(
            "I'm sorry, I didn't understand that.".to_string(),
            |choice| choice.message.content.clone(),
        );

        self.msg_history.push(Message {
            role: "assistant".to_string(),
            content: response_content.clone(),
        });

        Ok(response_content)
    }
}
