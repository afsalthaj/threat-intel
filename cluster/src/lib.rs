use crate::bindings::exports::rag::cluster_exports::api::{ClusterInput, Guest};

mod bindings;

struct Component;

impl Guest for Component {
    fn get_alert_messages() -> Result<Vec<String>, String> {
        todo!()
    }

    fn process_cluster_input(log: ClusterInput) -> Result<String, String> {
        todo!()
    }
}

bindings::export!(Component with_types_in bindings);
