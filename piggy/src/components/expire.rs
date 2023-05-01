use engine_sdk::{registry::{Component, uuid::uuid}, Timer};
use serde::{Serialize, Deserialize};

/// marks an entity such that it expires after a certain amount of time
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Expire {
    pub timer:Timer
}

impl Component for Expire {
    fn type_id() -> engine_sdk::registry::uuid::Uuid {
        uuid!("18343023-615d-4d20-9aa3-68da4465a95f")
    }
}