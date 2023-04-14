use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct AtlasDef {
    pub atlas:u32,
    pub atlas_index:u16,
    pub tags:Vec<String>
}

pub trait Editor {
    fn def_wall(&mut self, param:AtlasDef);
    fn def_entity(&mut self, param:AtlasDef);
}