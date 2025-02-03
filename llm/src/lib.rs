use crate::bindings::exports::rag::llm_exports::api::{Context, Guest, LlmResponse, Prompt};

mod bindings;

struct Component;

impl Guest for Component {
    fn ask_model(prompt: Prompt, context: Context) -> Result<LlmResponse, String> {
        todo!()
    }
}

bindings::export!(Component with_types_in bindings);
