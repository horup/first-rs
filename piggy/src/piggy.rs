
use engine_sdk::{Map, Game, Event};
use serde::{Deserialize, Serialize};
use crate::{State, systems};

#[derive(Default, Serialize, Deserialize)]
pub struct Piggy {
    pub current_map: Map,
    pub state:State
}

impl Game for Piggy {
    fn init(&mut self, engine:&mut dyn engine_sdk::Engine) {
        systems::init_system(&mut self.state, engine);
    }

    fn update(&mut self, engine:&mut dyn engine_sdk::Engine) {
        if engine.key_just_pressed(engine_sdk::VirtualKeyCode::Escape) {
            dbg!(8);
            engine.set_cursor_visible(true);
        }
        engine.set_cursor_visible(false);
        systems::player_system(&mut self.state, engine);
        systems::mob_system(&mut self.state, engine);
        systems::physics_system(&mut self.state, engine);
        systems::item_system(&mut self.state, engine);
        systems::activator_system(&mut self.state, engine);
        systems::door_system(&mut self.state, engine);
        systems::effector_system(&mut self.state, engine);
        systems::render_world_system(&mut self.state, engine);
        systems::render_flash_system(&mut self.state, engine);
        systems::ui_system(&mut self.state, engine);
    }

    fn on_event(&mut self, engine:&mut dyn engine_sdk::Engine, event:&Event) {
        match event {
            Event::Map { map } => {
                systems::start_system(&mut self.state, engine, map);
            }
        }
    }
}