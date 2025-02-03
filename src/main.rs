use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
struct DeepSeekRequest {
    prompt: String,
    max_tokens: u32,
}

#[derive(Deserialize, Debug)]
struct DeepSeekResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = "Insert your DeepSeek API here";
    let api_url = "API endpoint";

    let request_body = DeepSeekRequest {
        prompt: "Your Prompt Here".to_string(),
        max_tokens: 50,
    };

    let client = Client::new();

    let response = client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    if response.status().is_success() {
        let response_body: DeepSeekResponse = response.json().await?;
        println!("Response: {:?}", response_body);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}
