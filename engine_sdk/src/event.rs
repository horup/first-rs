use crate::Map;

#[derive(Clone)]
pub enum Event {
    LoadMap {
        map:Map
    }
}