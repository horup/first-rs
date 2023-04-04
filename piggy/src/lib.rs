mod piggy;
use engine_sdk::Game;
pub use piggy::*;

mod defs;
pub use defs::*;

mod facade;
pub use facade::*;

mod campaign;
pub use campaign::*;

mod signal;
pub use signal::*;

mod signals;
pub use signals::*;

pub mod systems;
pub mod components;
pub mod singletons;

#[no_mangle]
pub fn create() -> Box<dyn Game> {
    Box::<piggy::Piggy>::default()
}