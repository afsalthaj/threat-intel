use rust_bert::pipelines::sentence_embeddings::{Embedding, SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType};
use crate::bindings::exports::rag::embeddings_exports::api::{Guest, LogEmbedding};

mod bindings;

struct Component;

impl Guest for Component {
    fn get_log_embedding(log: String) -> Result<LogEmbedding, String> {
        let text_owned = log.clone();
        let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL6V2)
            .create_model()
            .expect("Failed to load embedding model");

        let embeddings: Vec<Embedding> =
            model.encode(&[text_owned]).expect("Failed to encode text");

        let embedding = LogEmbedding {
            value: embeddings
        };

        embeddings[0].clone()

    }
}

bindings::export!(Component with_types_in bindings);
