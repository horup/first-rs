use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pic {
    pub atlas:u32,
    pub index:u16
}

impl Pic {
    pub const fn new(atlas:u32, index:u16) -> Self {
        Self {
            atlas,
            index
        }
    }
}