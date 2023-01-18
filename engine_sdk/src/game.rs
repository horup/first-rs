use crate::Engine;

pub trait Game : 'static {
    fn init(&mut self, engine:&mut dyn Engine);
    fn update(&mut self, engine:&mut dyn Engine);
}