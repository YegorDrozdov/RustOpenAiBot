use std::fmt;

#[derive(Debug)]
pub struct OpenAIError {
    pub message: String,
}

impl fmt::Display for OpenAIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OpenAIError: {}", self.message)
    }
}

impl std::error::Error for OpenAIError {}
