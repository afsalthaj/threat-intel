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
use crate::bindings::rag::embeddings_client::embeddings_client::{Api as EmbedderApi, LogInput};
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
            let feature_vector_and_logs = &mut state.feature_vector_and_logs;
            let batch_size: usize = std::env::var("LOG_BATCH_SIZE_FOR_LOCAL_MODEL_UPDATE").expect(
                "LOG_BATCH_SIZE_FOR_LOCAL_MODEL_UPDATE not in the env"
            ).parse().expect("Batch size needs to be u64");

            let centroid_worker_name =  std::env::var("CENTROID_WORKER_NAME").expect(
                "CENTROID_WORKER_NAME not in the env"
            );

            state.batch_logs.push(log.clone());

            let tokens: Vec<&str> = log.message.split_whitespace().collect();
            let mut feature_vector = vec![0.0; token_index.len()];

            // No expert here: but the idea is
            // Token-specific indexing: Each unique word gets its own index, meaning "Afsal"
            // always maps to the same position in the vector.
            // This ensures that user names and specific terms are consistently represented.
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

            feature_vector_and_logs.push((feature_vector, log));

            if feature_vector_and_logs.len() == batch_size {
                state.local_model = Some(streaming_kmeans(
                    &feature_vector_and_logs,
                    &state.batch_logs,
                    state.local_model.clone(),
                ));

                state.batch_logs.clear();
            }


            let local_model = state.local_model.clone();

            // If local model is ready, try and update the centroid
            // the general model may not be fully ready yet and in this case, you will get a None
            if let Some(local_model) = local_model {
                let serialized = serde_json::to_value(local_model)
                    .expect("Unable to serialize")
                    .to_string();

                let component_id_of_centroid = std::env::var("CENTROID_COMPONENT_ID").expect(
                    "CENTROID_COMPONENT_ID not in the env"
                );

                let centroid_uri = Uri {
                    value: format!("urn:worker:{}/{}", &component_id_of_centroid, &centroid_worker_name),
                };

                let centroid_api = CentroidApi::new(&centroid_uri);

                let generic_model_opt =
                    centroid_api.blocking_process_local_model(&LocalModel { value: serialized })?;

                let mut any_immediate_alert = None;
                match generic_model_opt {
                    Some(generic_model) => {
                        // Possibly all the logs in the current worker can now be assigned to specific clusters
                        let generic_model: KMeans<f64, L2Dist> =
                            serde_json::from_str(&generic_model.value)
                                .map_err(|err| err.to_string())?;

                        // Prepare the token counts as feature vectors
                        let max_len = feature_vector_and_logs.iter().map(|v| v.0.len()).max().unwrap_or(0);
                        let mut padded_vectors = feature_vector_and_logs.clone();

                        println!("--- End of Batch ---\n");

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
                                let component_id_of_embedder = std::env::var("EMBEDDER_COMPONENT_ID").expect(
                                "EMBEDDER_COMPONENT_ID not in the env"
                            );

                            let embedder_uri = Uri {
                                value: format!(
                                    "urn:worker:{component_id_of_embedder}/{}",
                                    &embedder_worker
                                ),
                            };

                            let embedder_api = EmbedderApi::new(&embedder_uri);

                            let embed_result = embedder_api.blocking_get_log_embedding(
                                &LogInput {
                                    log: feature_vector_and_log.1.message.clone(),
                                }
                            )?;

                            let component_id_of_cluster = std::env::var("CLUSTER_COMPONENT_ID").expect(
                                "CLUSTER_COMPONENT_ID not in the env"
                            );

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

                            let msg = api.blocking_process_cluster_input(&cluster_input)?;
                            if let Some(msg)  = msg {
                                any_immediate_alert = Some(msg)
                            };

                            state.batch_logs.clear();
                        }

                        let response = if let Some(immediate_alert) = any_immediate_alert {
                            format!(  "The log is sent to a specific cluster. There is an immediate alert: {}", immediate_alert.value)
                        } else {
                            "The log is sent to a specific cluster".to_string()
                        };

                        let response = Response {
                            detail: response
                        };

                        if feature_vector_and_logs.len() == batch_size {
                            state.feature_vector_and_logs.clear()
                        }

                        Ok(response)
                    }

                    None => {
                        let response = Response {
                            detail: "processed and found new category of log".to_string(),
                        };

                        if feature_vector_and_logs.len() == batch_size {
                            state.feature_vector_and_logs.clear()
                        }

                        Ok(response)
                    }
                }

            } else {
                let response = Response {
                    detail: "processed the log and waiting for new logs for analysis".to_string(),
                };

                if feature_vector_and_logs.len() == batch_size {
                    state.feature_vector_and_logs.clear()
                }

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
    .expect("Failed to get the data");

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
    feature_vector_and_logs: Vec<(Vec<f64>, LogEvent)>, // token counts and log messages are stored together
    local_model: Option<KMeans<f64, L2Dist>>,
    batch_logs: Vec<LogEvent>, // A subset of the original logs that are going to be used for updating local model
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        feature_vector_and_logs: vec![],
        token_index: HashMap::new(),
        local_model: None,
        index_counter: 0,
        batch_logs: vec![]
    });
}

bindings::export!(Component with_types_in bindings);
