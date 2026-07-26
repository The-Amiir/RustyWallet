use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OpenAiRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
}

#[derive(Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct OpenAiResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize)]
pub struct Choice {
    pub message: ResponseMessage,
}

#[derive(Deserialize)]
pub struct ResponseMessage {
    pub content: String,
}