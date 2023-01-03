use engine::{Engine, Game};

struct MyGame {
}

impl Game for MyGame {
    fn update(&mut self, ctx:&mut engine::Context) {
        ctx.draw();
    }
} 

fn main() {
    pollster::block_on(async {
        let g = MyGame {
        };
        let engine = Engine::new(g).await;
        engine.run().await; 
    }); 
}