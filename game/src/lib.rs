use engine_sdk::Game;

#[derive(Default)]
pub struct State {
    
}

#[derive(Default)]
pub struct MyGame {
    pub first_run:Option<State>
}

impl Game for MyGame {
    fn update(&mut self, ctx:&mut dyn engine_sdk::Engine) {
        ctx.draw();
    }
} 