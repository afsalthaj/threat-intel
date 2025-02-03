use std::cell::RefCell;
use linfa::prelude::*;
use linfa_clustering::KMeans;
use linfa_nn::distance::L2Dist;
use ndarray::{concatenate, Array2, Axis};
use crate::bindings::exports::rag::centroid_exports::api::{Guest, LocalModel, NewModel};
use crate::local_model::LocalModelDeserialized;

mod bindings;
mod local_model;

struct Component;

impl Guest for Component {
    fn process_local_model(log: LocalModel) -> Result<Option<NewModel>, String> {
        let deserialized_model =
            LocalModelDeserialized::from_local_model(&log)?;

        STATE.with_borrow_mut(|state| {
            state.local_model.push(deserialized_model);

            // We will make this a constant for now, but we can make it configurable later.
            // Once the local nodes are done processing the logs, we will update the centroid
            if state.local_model.len() >= 10 {
                state.update_global_model()?;
            }

            Ok(state.model.clone().map(|x| NewModel {
                value: serde_json::to_string(&x).expect("Failed to serialize the model.")
            }))
        })
    }
}

struct State {
    local_model: Vec<LocalModelDeserialized>,
    model: Option<KMeans<f64, L2Dist>>
}

impl State {
    pub fn update_global_model(&mut self) -> Result<(), String> {
        if self.local_model.is_empty() {
            return Err("No local models to update the global model.".to_string())
        }

        let mut combined_centroids: Vec<Array2<f64>> = Vec::new();

        for local_model in &self.local_model {
            let centroids = local_model.value.centroids().clone();
            combined_centroids.push(centroids);
        }

        let synthetic_dataset = concatenate(Axis(0), &combined_centroids.iter().map(|x| x.view()).collect::<Vec<_>>())
            .expect("Failed to concatenate centroids.");

        let num_clusters = self.local_model[0].value.centroids().nrows();

        let data = DatasetBase::from(synthetic_dataset);

        let global_model = KMeans::params(num_clusters)
            .fit(&data).map_err(|err| err.to_string())?;

        self.model = Some(global_model);
        println!("Global model updated successfully!");
        Ok(())
    }
}

thread_local! {
    /**
     * This holds the state of our application, which is always bound to
     * a given user.
     */
    static STATE: RefCell<State> = RefCell::new(State {
        local_model: vec![],
        model: None,
    });
}

bindings::export!(Component with_types_in bindings);
