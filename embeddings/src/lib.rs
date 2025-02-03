use reqwest::*;
use crate::bindings::exports::rag::embeddings_exports::api::{Guest, LogEmbedding};

mod bindings;

struct Component;

impl Guest for Component {
    fn get_log_embedding(log: String) -> std::result::Result<LogEmbedding, String> {
        let client = Client::new();

        // Currently hard coded to get something working
        let response: Response = client
            .post(&format!("http://127.0.0.1:8089/post-example"))
            .json(&log)
            .send()
            .expect("Request failed");

        let embedding = response
            .json::<Vec<f32>>()
            .expect("Invalid response");

        Ok(LogEmbedding {
            value: embedding
        })

    }
}

bindings::export!(Component with_types_in bindings);
