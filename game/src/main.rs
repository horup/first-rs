use engine::Engine;
fn main() {
    pollster::block_on(async {
        let engine = Engine::new().await;
        engine.run().await;
    });
}
