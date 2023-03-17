mod piggy;
use engine_sdk::Game;
pub use piggy::*;

mod defs;
pub use defs::*;

mod state;
pub use state::*;

pub mod systems;
pub mod components;

#[no_mangle]
pub fn create() -> Box<dyn Game> {
    Box::new(Piggy::default())
}