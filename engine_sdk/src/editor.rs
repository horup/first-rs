use serde::{Serialize, Deserialize};
use crate::Pic;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Def {
    pub pic:Pic,
    pub class:String
}

impl Def {
    pub fn new(atlas:u32, index:u16, class:&str) -> Self {
        Self {
            pic:Pic::new(atlas, index),
            class:class.into()
        }
    }
}

pub trait Editor {
    fn def_wall(&mut self, param:Def);
    fn def_entity(&mut self, param:Def);
}