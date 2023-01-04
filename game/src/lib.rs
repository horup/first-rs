use engine_sdk::Game;

pub struct MyGame {

}

impl Game for MyGame {
    fn update(&mut self, ctx:&mut dyn engine_sdk::Context) {
        ctx.draw();
    }
} 