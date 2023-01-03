use crate::Context;

pub trait Game : 'static {
    fn update(ctx:&mut Context);
}