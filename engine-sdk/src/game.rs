use std::task::Context;

pub trait Game : 'static {
    fn update(&mut self, ctx:&mut Context);
}