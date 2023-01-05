pub trait Engine {
    fn draw(&mut self);
    fn define_texture(&mut self, id:u32, texture:String);
}
