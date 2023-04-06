use engine_sdk::registry::{Component, uuid::{Uuid, uuid}};
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
#[derive(Default)]
pub enum Effector {
    #[default]
    ExitMarker
}



impl Component for Effector {
    fn type_id() -> Uuid {
        uuid!("4a72cb0f-2338-40df-8cef-528192a9cd86")
    }
}