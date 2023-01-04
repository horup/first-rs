use crate::Engine;

pub trait Game : 'static {
    fn update(&mut self, engine:&mut dyn Engine);
}