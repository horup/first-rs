
use engine_sdk::{Map, Game, Event, VirtualKeyCode, world::World, Sprite, Tile, Grid, Tilemap};
use crate::{systems, components::{Player, Door, Mob, Activator, Health, Item, Effector}, Global};

pub struct Piggy {
    pub current_map: Map,
    pub world:World
}

impl Default for Piggy {
    fn default() -> Self {
        let mut world = World::new();
        world.register_component::<Sprite>();
        world.register_component::<Player>();
        world.register_component::<Door>();
        world.register_component::<Mob>();
        world.register_component::<Activator>();
        world.register_component::<Health>();
        world.register_component::<Item>();
        world.register_component::<Effector>();

        world.register_singleton::<Tilemap>();
        world.register_singleton::<Global>();
        Self { current_map: Default::default(), world }
    }
}

impl Game for Piggy {
    fn init(&mut self, engine:&mut dyn engine_sdk::Engine) {
        systems::init_system(&mut self.world, engine);
    }

    fn update(&mut self, engine:&mut dyn engine_sdk::Engine) {
        if engine.key_just_pressed(engine_sdk::VirtualKeyCode::Escape) {
            engine.set_cursor_grabbed(true);
        }
        if engine.mouse_down(0) {
            engine.set_cursor_grabbed(false);
        }
        
        systems::player_system(&mut self.world, engine);
        systems::mob_system(&mut self.world, engine);
        systems::physics_system(&mut self.world, engine);
        systems::item_system(&mut self.world, engine);
        systems::activator_system(&mut self.world, engine);
        systems::door_system(&mut self.world, engine);
        systems::effector_system(&mut self.world, engine);
        systems::render_world_system(&mut self.world, engine);
        systems::render_flash_system(&mut self.world, engine);
        systems::ui_system(&mut self.world, engine);

        #[cfg(not(target_arch = "wasm32"))]
        {
            let autosave = "autosave.sav";
            if engine.key_just_pressed(VirtualKeyCode::F5) {
                let mut bytes = Vec::new();
                self.world.serialize(&mut bytes);
                std::fs::write(autosave, bytes).unwrap();
            }
            if engine.key_just_pressed(VirtualKeyCode::F6) {
                if let Ok(serialized) = std::fs::read(autosave) {
                    self.world.deserialize(&serialized);
                }
            }
        }
    }

    fn on_event(&mut self, engine:&mut dyn engine_sdk::Engine, event:&Event) {
        match event {
            Event::Map { map } => {
                systems::start_system(&mut self.world, engine, map);
            }
            Event::Focused(focused) => {
                engine.set_cursor_grabbed(!*focused);
            },
        }
    }
}