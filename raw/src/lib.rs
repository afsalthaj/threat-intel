use linfa::prelude::*;
use linfa::DatasetBase;
use linfa_clustering::KMeans;
use linfa_nn::distance::L2Dist;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::bindings::exports::rag::raw_exports::api::{Guest, LogEvent, Response};
use crate::bindings::golem::rpc::types::Uri;
use crate::bindings::rag::centroid_client::centroid_client::{Api as CentroidApi, LocalModel};
use crate::bindings::rag::cluster_client::cluster_client::{Api as ClusterApi, ClusterInput};
use crate::bindings::rag::embeddings_client::embeddings_client::Api as EmbedderApi;
use ndarray::{Array2, AssignElem};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod bindings;

struct Component;

impl Guest for Component {
    // Once the log event collection reaches a 1000, we will send the local model to the centroid worker,
    // and get an updated model back.
    fn process_log_event(log: LogEvent) -> Result<Response, String> {
        STATE.with_borrow_mut(|state| {
            let token_index = &mut state.token_index;
            let mut index_counter = state.index_counter;
            let token_counts = &mut state.token_counts;
            let batch_size = state.batch_size;
            state.batch_logs.push(log.clone());

            let tokens: Vec<&str> = log.message.split_whitespace().collect();
            let mut feature_vector = vec![0.0; token_index.len()];

            // Computing sort of a term frequency
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

            token_counts.push((feature_vector, log));

            if token_counts.len() == batch_size {
                state.local_model = Some(streaming_kmeans(
                    &token_counts,
                    &state.batch_logs,
                    state.local_model.clone(),
                ));
                token_counts.clear();
                state.batch_logs.clear();
            }

            // Currently we don't redistribute further - but we can make more secondary reducers
            // for kmeans if things are working
            let centroid_worker_id = "centroid".to_string();

            // To be replaced
            let component_id = "centroid_component_id".to_string();

            let centroid_uri = Uri {
                value: format!("urn:worker:{component_id}/{}", &centroid_worker_id),
            };

            let api = CentroidApi::new(&centroid_uri);

            let local_model = state.local_model.clone();

            // If local model is ready, try and update the centroid
            // the general model may not be fully ready yet and in this case, you will get a None
            if let Some(local_model) = local_model {
                let serialized = serde_json::to_value(local_model)
                    .expect("Unable to serialize")
                    .to_string();

                let generic_model_opt =
                    api.blocking_process_local_model(&LocalModel { value: serialized })?;

                match generic_model_opt {
                    Some(generic_model) => {
                        // Possibly all the logs in the current worker can now be assigned to specific clusters
                        let generic_model: KMeans<f64, L2Dist> =
                            serde_json::from_str(&generic_model.value)
                                .map_err(|err| err.to_string())?;

                        // Prepare the token counts as feature vectors
                        let max_len = token_counts.iter().map(|v| v.0.len()).max().unwrap_or(0);
                        let mut padded_vectors = token_counts.clone();

                        // Pad vectors to ensure they're all the same length
                        for feature_vector_and_log in &mut padded_vectors {
                            feature_vector_and_log.0.resize(max_len, 0.0);
                            let single_log_dataset = DatasetBase::from(
                                Array2::from_shape_vec(
                                    (1, max_len),
                                    feature_vector_and_log.0.clone(),
                                )
                                .unwrap(),
                            );

                            let cluster = generic_model.predict(&single_log_dataset)[0];

                            // This can be any worker. It is kept as a separate worker
                            // since I am not entirely sure the best way to embed
                            let embedder_worker = Uuid::new_v4().to_string();
                            let component_id_of_embedder = "embedder_component_id".to_string();

                            let embedder_uri = Uri {
                                value: format!(
                                    "urn:worker:{component_id_of_embedder}/{}",
                                    &embedder_worker
                                ),
                            };

                            let embedder_api = EmbedderApi::new(&embedder_uri);

                            let embed_result = embedder_api.blocking_get_log_embedding(
                                &feature_vector_and_log.1.clone().message,
                            )?;

                            let component_id_of_cluster = "cluster".to_string();
                            let cluster_worker_name = format!("cluster_{}", cluster);

                            let cluster_uri = Uri {
                                value: format!(
                                    "urn:worker:{component_id_of_cluster}/{}",
                                    &cluster_worker_name
                                ),
                            };

                            let api = ClusterApi::new(&cluster_uri);

                            let cluster_input = ClusterInput {
                                log_line: feature_vector_and_log.1.clone().message,
                                embedding: embed_result.value,
                            };

                            let _ = api.blocking_process_cluster_input(&cluster_input);
                            println!("The log has been sent to a threat cluster")
                        }
                    }

                    None => {}
                }

                let response = Response {
                    detail: "processed and found new category of log".to_string(),
                };

                Ok(response)
            } else {
                let response = Response {
                    detail: "processed the log and waiting for new logs for analysis".to_string(),
                };

                Ok(response)
            }
        })
    }
}

fn streaming_kmeans(
    token_counts: &Vec<(Vec<f64>, LogEvent)>,
    logs: &Vec<LogEvent>,
    prev_model: Option<KMeans<f64, L2Dist>>,
) -> KMeans<f64, L2Dist> {
    let max_len = token_counts.iter().map(|v| v.0.len()).max().unwrap_or(0);
    let mut padded_vectors = token_counts.clone();

    for vec in &mut padded_vectors {
        vec.0.resize(max_len, 0.0);
    }

    let data: Array2<f64> = Array2::from_shape_vec(
        (padded_vectors.len(), max_len),
        padded_vectors
            .iter()
            .map(|x| x.0.clone())
            .collect::<Vec<_>>()
            .concat(),
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
        println!(
            "Log: {}\nBelongs to cluster: {}\n",
            logs[i].message, cluster
        );
    }

    println!("--- End of Batch ---\n");

    updated_model
}

struct State {
    token_index: HashMap<String, usize>,
    index_counter: usize,
    token_counts: Vec<(Vec<f64>, LogEvent)>, // token counts and log messages are stored together
    local_model: Option<KMeans<f64, L2Dist>>,
    batch_size: usize,
    batch_logs: Vec<LogEvent>, // A subset of the original logs that are going to be used for updating local model
}

thread_local! {
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
