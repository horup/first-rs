
use engine_sdk::{Map, Game, Event, VirtualKeyCode};
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
            engine.set_cursor_grabbed(true);
        }
        if engine.mouse_down(0) {
            engine.set_cursor_grabbed(false);
        }
        
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

        #[cfg(not(target_arch = "wasm32"))]
        {
            let autosave = "autosave.sav";
            if engine.key_just_pressed(VirtualKeyCode::F5) {
                let serialized = bincode::serialize(&self.state).unwrap();
                std::fs::write(autosave, serialized).unwrap();
            }
            if engine.key_just_pressed(VirtualKeyCode::F6) {
                if let Ok(serialized) = std::fs::read(autosave) {
                    let state:State = bincode::deserialize(&serialized).unwrap();
                    self.state = state;
                }
            }
        }
    }

    fn on_event(&mut self, engine:&mut dyn engine_sdk::Engine, event:&Event) {
        match event {
            Event::Map { map } => {
                systems::start_system(&mut self.state, engine, map);
            }
            Event::Focused(focused) => {
                engine.set_cursor_grabbed(!*focused);
            },
        }
    }
}