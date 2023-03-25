mod piggy;
use engine_sdk::Game;
pub use piggy::*;

mod defs;
pub use defs::*;

mod global;
pub use global::*;

mod world_ext;
pub use world_ext::*;

pub mod systems;
pub mod components;

#[no_mangle]
pub fn create() -> Box<dyn Game> {
    Box::<piggy::Piggy>::default()
}