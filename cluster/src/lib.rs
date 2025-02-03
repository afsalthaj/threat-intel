use std::cell::RefCell;
use std::collections::HashMap;
use uuid::Uuid;
use crate::bindings::exports::rag::cluster_exports::api::{AlertMessage, ClusterInput, Guest};
use crate::bindings::golem::rpc::types::Uri;
use crate::bindings::rag::llm_client::llm_client::{Api as LlmApi, Context, Prompt};

mod bindings;

struct Component;

impl Guest for Component {
    fn get_alert_messages() -> Result<Vec<AlertMessage>, String> {
        STATE.with_borrow_mut(|state| {
            let mut messages = vec![];

            for (_, v) in state.alert_messages.clone() {
                messages.push(AlertMessage {
                    value: v
                });
            }

            Ok(messages)
        })
    }

    fn process_cluster_input(log: ClusterInput) -> Result<Option<AlertMessage>, String> {
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

            // Not sure if this is a good idea - this requires domain level testing
            // Waiting for more messages in the cluster
            // may be ok, as we don't filter "risk" vs "safe" messages
            if state.log_messages.len() > 10 {
                let llm_worker_id = Uuid::new_v4();

                // To be replaced
                let component_id = std::env::var("LLM_COMPONENT_ID").expect(
                    "LLM_COMPONENT_ID not in the env"
                );

                let llm_component_id = Uri {
                    value: format!("urn:worker:{component_id}/{}", &llm_worker_id),
                };

                let api = LlmApi::new(&llm_component_id);

                let prompt = Prompt {
                    description: "Find security attack behaviours".to_string()
                };

                let log_messages = state.log_messages.clone().into_iter().map(|x| x.0).collect::<Vec<_>>();

                let context = Context {
                    value: log_messages.join(", ")
                };

                let result = api.blocking_ask_model(&prompt, &context)?;

                state.alert_messages.insert(
                    log_messages,
                    result.value.clone()
                );

                Ok(Some(AlertMessage {
                    value: result.value
                }))

            } else {
                Ok(None)
            }
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
