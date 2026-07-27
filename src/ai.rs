use serde::{Deserialize, Serialize};
use crate::errors::AppResult;

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

pub async fn categorize(description: &str) -> AppResult<String> {
    let api_key = std::env::var("OPENROUTER_API_KEY")
        .map_err(|_| crate::errors::AppError::Internal("Missing OPENROUTER_API_KEY".to_string()))?;

    let prompt = format!(
        "Categorize the transaction '{}' into one of these labels: Food, Transport, Education, Income, Other. Return only the label name.",
        description
    );

    let client = reqwest::Client::new();
    let request = OpenAiRequest {
        model: "openrouter/free".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: 0.0,
    };

    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| crate::errors::AppError::Internal(format!("Request failed: {}", e)))?;

    let body: OpenAiResponse = response
        .json()
        .await
        .map_err(|e| crate::errors::AppError::Internal(format!("Parse failed: {}", e)))?;

    let category = body
        .choices
        .first()
        .ok_or_else(|| crate::errors::AppError::Internal("No response".to_string()))?
        .message
        .content
        .trim()
        .to_string();

    let valid = ["Food", "Transport", "Education", "Income", "Other"];
    if valid.contains(&category.as_str()) {
        Ok(category)
    } else {
        Ok("Other".to_string())
    }
}