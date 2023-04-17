use engine_sdk::{registry::{Component, uuid::{Uuid, uuid}}, Pic};
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Activator {
    Door {
        key:Option<Pic>
    }
}

impl Component for Activator {
    fn type_id() -> Uuid{
        uuid!("2ca7d324-0665-4afd-a98f-199e6d529aab")
    }
}

impl Default for Activator {
    fn default() -> Self {
        Self::Door { key: None }
    }
}