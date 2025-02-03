use crate::bindings::exports::rag::centroid_exports::api::LocalModel;
use linfa_clustering::KMeans;
use linfa_nn::distance::L2Dist;

pub struct LocalModelDeserialized {
    pub value: KMeans<f64, L2Dist>,
}

impl LocalModelDeserialized {
    pub fn from_local_model(local_model: &LocalModel) -> Result<LocalModelDeserialized, String> {
        let loaded_model: KMeans<f64, L2Dist> =
            serde_json::from_str(&local_model.value).map_err(|err| err.to_string())?;

        Ok(LocalModelDeserialized {
            value: loaded_model,
        })
    }
}
