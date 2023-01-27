use crate::Map;

#[derive(Clone)]
pub enum Event {
    Map {
        map:Map
    }
}