use serde::{Serialize, Deserialize};
use crate::Pic;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Def {
    pub pic:Pic,
    pub tags:Vec<String>
}

pub trait Editor {
    fn def_wall(&mut self, param:Def);
    fn def_entity(&mut self, param:Def);
}