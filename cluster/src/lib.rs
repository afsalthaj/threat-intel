use std::cell::RefCell;
use std::collections::HashMap;
use crate::bindings::exports::rag::cluster_exports::api::{ClusterInput, Guest};

mod bindings;

struct Component;

impl Guest for Component {
    fn get_alert_messages() -> Result<Vec<String>, String> {
        STATE.with_borrow_mut(|state| {
            let mut messages = vec![];

            for (_, v) in state.alert_messages.clone() {
                messages.push(v);
            }

            Ok(messages)
        })
    }

    fn process_cluster_input(log: ClusterInput) -> Result<String, String> {
        STATE.with_borrow_mut(|state| {
            let existing_logs = state.log_messages.clone();
            let new_embedding = log.embedding.clone();

            let mut found_similarity = false;
            let least_similarity = 0.2;

            for (_, existing_log_embedding) in existing_logs {
                let similarity = State::cosine_similarity(&new_embedding, &existing_log_embedding);
                if similarity > least_similarity {
                   found_similarity = true;
                   break;
                }
            }

            if found_similarity {
                state.log_messages.insert(
                    log.log_line.clone(),
                    log.embedding.clone()
                );
            };

            if state.log_messages.len() > 2 {
                todo!()
            }

            Ok("Found".to_string())
        })
    }
}

struct State {
    log_messages: HashMap<String, Vec<f32>>, // Each log message and their embedding (to find cosine similarity)
    alert_messages: HashMap<Vec<String>, String>
}

impl State {
    // The primary need of this is to shift the pipeline from develop model to
    // stagnate mode, where models settle down and there after
    // the new logs can be compared to cosine similarity across all clusters (which can be in 1000s)
    // and then push there and ask LLM to analyse, but this part isn't done as it is just a detail
    fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let magnitude_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let magnitude_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        dot_product / (magnitude_a * magnitude_b)
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
       log_messages: HashMap::new(),
        alert_messages: HashMap::new()
    });
}

bindings::export!(Component with_types_in bindings);
