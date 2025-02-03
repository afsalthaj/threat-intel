use std::cell::RefCell;
use std::collections::HashMap;
use linfa::DatasetBase;
use linfa_clustering::KMeans;
use linfa_nn::distance::L2Dist;
use linfa::prelude::*;
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use crate::bindings::exports::rag::raw_exports::api::{Guest, LogEvent, Response};
use crate::bindings::golem::rpc::types::Uri;
use crate::bindings::rag::llm_client::llm_client::Api;

mod bindings;

struct Component;

impl Guest for Component {
    fn process_log_event(log: LogEvent) -> Result<Response, String> {
        STATE.with_borrow_mut(|state| {
            let token_index = &mut state.token_index;
            let mut index_counter = state.index_counter;
            let token_counts = &mut state.token_counts;

            let batch_size = state.batch_size;

            let tokens: Vec<&str> = log.message.split_whitespace().collect();
            let mut feature_vector = vec![0.0; token_index.len()];

            state.batch_logs.push(log.clone());

            for token in tokens {
                let token = token.to_lowercase();
                if !token_index.contains_key(&token) {
                    token_index.insert(token.clone(), index_counter);
                    feature_vector.push(1.0);
                    index_counter += 1;
                } else {
                    let idx = token_index[&token];
                    if idx >= feature_vector.len() {
                        feature_vector.resize(idx + 1, 0.0);
                    }
                    feature_vector[idx] += 1.0;
                }
            }

            token_counts.push(feature_vector);

            if token_counts.len() == batch_size {
                state.local_model = Some(streaming_kmeans(&token_counts, &state.batch_logs, state.local_model.clone()));

                token_counts.clear();
                state.batch_logs.clear();
            }
        });

        let worker = "well_well";

        let component_id = "llm";
        let uri = Uri { value: format!("urn:worker:{component_id}/{}", &worker) };

        let api = Api::new(&uri);
        Err("hello".to_string())
    }
}

fn streaming_kmeans(
    token_counts: &Vec<Vec<f64>>,
    logs: &Vec<LogEvent>,
    prev_model: Option<KMeans<f64, L2Dist>>
) -> KMeans<f64, L2Dist> {
    let max_len = token_counts.iter().map(|v| v.len()).max().unwrap_or(0);
    let mut padded_vectors = token_counts.clone();

    for vec in &mut padded_vectors {
        vec.resize(max_len, 0.0);
    }

    let data: Array2<f64> = Array2::from_shape_vec(
        (padded_vectors.len(), max_len),
        padded_vectors.concat(),
    )
        .unwrap();

    let dataset = DatasetBase::from(data);
    let n_clusters = 3;

    let params = KMeans::params(n_clusters).max_n_iterations(100);

    let updated_model = params
        .fit_with(prev_model, &dataset)
        .expect("KMeans updating failed");

    let memberships = updated_model.predict(&dataset);

    for (i, cluster) in memberships.iter().enumerate() {
        println!("Log: {}\nBelongs to cluster: {}\n", logs[i].message, cluster);
    }

    println!("--- End of Batch ---\n");

    updated_model
}

struct State {
    token_index: HashMap<String, usize>,
    index_counter: usize,
    token_counts: Vec<Vec<f64>>,
    local_model: Option<KMeans<f64, L2Dist>>,
    batch_size: usize,
    batch_logs: Vec<LogEvent>
}

thread_local! {
    /**
     * This holds the state of our application, which is always bound to
     * a given user.
     */
    static STATE: RefCell<State> = RefCell::new(State {
        token_counts: vec![],
        token_index: HashMap::new(),
        local_model: None,
        index_counter: 0,
        batch_size: 0,
        batch_logs: vec![]
    });
}


bindings::export!(Component with_types_in bindings);
