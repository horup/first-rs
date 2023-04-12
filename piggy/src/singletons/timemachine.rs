use std::collections::VecDeque;

use engine_sdk::registry::{Component, uuid::uuid, Registry};
use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Timemachine {
    pub last_snapshot_time:f64,
    snapshots:VecDeque<Vec<u8>>
}

impl Timemachine {
    pub fn push(&mut self, r:&mut Registry) {
        let mut bytes = Vec::with_capacity(1024);
        r.serialize(&mut bytes);
        self.snapshots.push_front(bytes);
    }

    pub fn pop(&mut self, r:&mut Registry) {
        if let Some(bytes) = self.snapshots.pop_front() {
            r.deserialize(&bytes);
        }
    }
}

impl Component for Timemachine {
    fn type_id() -> engine_sdk::registry::uuid::Uuid {
        uuid!("39b04d95-6ed1-45ff-ba55-0c7bad8a8256")
    }
}
