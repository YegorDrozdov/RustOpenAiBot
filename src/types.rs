use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct OpenAIRequest {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Clone)]
pub struct OpenAIResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize, Clone)]
pub struct Choice {
    pub message: MessageContent,
}

#[derive(Deserialize, Clone)]
pub struct MessageContent {
    pub content: String,
}

#[derive(Deserialize, Clone)]
pub struct OpenAIErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Deserialize, Clone)]
pub struct ErrorDetail {
    pub message: String,
}
