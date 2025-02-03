use crate::bindings::exports::rag::llm_exports::api::{Context, Guest, LlmResponse, Prompt};
use reqwest::*;
use std::collections::HashMap;

mod bindings;

struct Component;

impl Guest for Component {
    fn ask_model(prompt: Prompt, context: Context) -> std::result::Result<LlmResponse, String> {
        let open_api_key = std::env::var("OPENAI_API_KEY").expect("Failed to find open API key");

        let bearer_token = format!("Bearer {}", open_api_key);

        let prompt = format!(
            "Query: {}\nContext: {}\nResponse:",
            prompt.description, context.value
        );

        let client = Client::new();

        let body = serde_json::json!({
            "model": "gpt-4",
            "messages": [{"role": "user", "content": prompt}],
        });

        let response: Response = client
            .post(&format!("https://api.openai.com/v1/chat/completions"))
            .json(&body)
            .header("Authorization", bearer_token)
            .send()
            .expect("Request failed");

        let result: HashMap<String, serde_json::Value> =
            response.json().map_err(|err| err.to_string())?;

        let llm_response = result["choices"][0]["message"]["content"]
            .as_str()
            .unwrap()
            .to_string();

        Ok(LlmResponse {
            value: llm_response
        })
    }
}

bindings::export!(Component with_types_in bindings);
