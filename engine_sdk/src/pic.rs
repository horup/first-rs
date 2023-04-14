use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Pic {
    pub atlas:u32,
    pub index:f32
}

impl Pic {
    pub fn new(atlas:u32, index:f32) -> Self {
        Self {
            atlas,
            index
        }
    }
}