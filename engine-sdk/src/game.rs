use crate::Context;

pub trait Game : 'static {
    fn update(&mut self, ctx:&mut dyn Context);
}