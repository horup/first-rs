use engine::{Engine};
use game::MyGame;

fn main() {
    pollster::block_on(async {
        let g = MyGame {
        };
        let engine = Engine::new(Box::new(g)).await;
        engine.run().await;  
    }); 
} 