use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
pub struct Item {
    pub amount:f32
}

impl Item {
    pub fn new(amount:f32) -> Self {
        Self {
            amount
        }
    }
}