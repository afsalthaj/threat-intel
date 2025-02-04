use crate::bindings::exports::rag::embeddings_exports::api::{Guest, LogEmbedding, LogInput as WitLogInput};
use reqwest::*;
use serde::Serialize;

mod bindings;

struct Component;

impl Guest for Component {
    fn get_log_embedding(log: WitLogInput) -> std::result::Result<LogEmbedding, String> {
        println!("Starting to embed the log lines");

        let client = Client::new();

        // Currently hard coded to get something working
        let response: Response = client
            .post(&format!("http://127.0.0.1:8089/get_log_embedding"))
            .json(&LogInput::from(log))
            .send()
            .expect("Request failed");

        println!("Successfully embedded");

        let embedding = response.json::<Vec<f32>>().map_err(|_| "Invalid response from embedder server")?;

        Ok(LogEmbedding { value: embedding })
    }
}

#[derive(Serialize)]
struct LogInput {
    log: String
}

impl From<WitLogInput> for LogInput {
    fn from(value: WitLogInput) -> Self {
        LogInput {
            log: value.log
        }
    }
}

bindings::export!(Component with_types_in bindings);
