use engine::{Engine, Game};

struct MyGame {
}

impl Game for MyGame {
    fn update(ctx:&mut engine::Context) {
        todo!()
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
