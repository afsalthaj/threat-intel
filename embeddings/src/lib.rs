use crate::bindings::exports::rag::embeddings_exports::api::{AddThreatResponse, Guest, ThreatIntel};

mod bindings;

struct Component;

impl Guest for Component {
    fn add_threat(threat_intel: ThreatIntel) -> Result<AddThreatResponse, String> {
        todo!()
    }
}

bindings::export!(Component with_types_in bindings);
