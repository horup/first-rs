use crate::{Engine, Event};

pub trait Game : 'static {
    fn init(&mut self, engine:&mut dyn Engine);
    fn update(&mut self, engine:&mut dyn Engine);
    fn serialize(&self) -> Vec<u8> {
        Vec::new()
    }
    fn deserialize(&mut self, _bytes:&Vec<u8>) {
    }

    fn on_event(&mut self, _engine:&mut dyn Engine, _event:&Event) {
    }
}