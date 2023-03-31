
use engine_sdk::{Map, Game, Event as EngineEvent, VirtualKeyCode, registry::{Registry, Facade}, Sprite, Tilemap};
use crate::{systems, components::{Player, Door, Mob, Activator, Health, Item, Effector, Event}, singletons::GameState};

pub struct Piggy {
    pub registry:Registry
}

impl Default for Piggy {
    fn default() -> Self {
        let mut registry = Registry::new();
        registry.register_component::<Sprite>();
        registry.register_component::<Player>();
        registry.register_component::<Door>();
        registry.register_component::<Mob>();
        registry.register_component::<Activator>();
        registry.register_component::<Health>();
        registry.register_component::<Item>();
        registry.register_component::<Effector>();
        registry.register_component::<Event>();

        registry.register_singleton::<Tilemap>();
        registry.register_singleton::<GameState>();
        Self { registry }
    }
}


impl Game for Piggy {
    fn init(&mut self, engine:&mut dyn engine_sdk::Engine) {
        systems::init_system(&mut self.registry, engine);
    }

    fn update(&mut self, engine:&mut dyn engine_sdk::Engine) {
        if engine.key_just_pressed(engine_sdk::VirtualKeyCode::Escape) {
            engine.set_cursor_grabbed(true);
        }
        if engine.mouse_down(0) {
            engine.set_cursor_grabbed(false);
        }
        
        systems::player_system(&mut self.registry, engine);
        systems::mob_system(&mut self.registry, engine);
        systems::physics_system(&mut self.registry, engine);
        systems::item_system(&mut self.registry, engine);
        systems::activator_system(&mut self.registry, engine);
        systems::door_system(&mut self.registry, engine);
        systems::effector_system(&mut self.registry, engine);
        systems::render_world_system(&mut self.registry, engine);
        systems::render_flash_system(&mut self.registry, engine);
        systems::ui_system(&mut self.registry, engine);
        systems::start_system(&mut self.registry, engine);
        systems::cleanup(&mut self.registry);

        #[cfg(not(target_arch = "wasm32"))]
        {
            let autosave = "autosave.sav";
            if engine.key_just_pressed(VirtualKeyCode::F5) {
                let mut bytes = Vec::new();
                self.registry.serialize(&mut bytes);
                std::fs::write(autosave, bytes).unwrap();
            }
            if engine.key_just_pressed(VirtualKeyCode::F6) {
                if let Ok(serialized) = std::fs::read(autosave) {
                    self.registry.deserialize(&serialized);
                }
            }
        }
    }

    fn on_event(&mut self, engine:&mut dyn engine_sdk::Engine, event:&EngineEvent) {
        match event {
            EngineEvent::Map { map } => {
                //systems::start_system(&mut self.registry, engine, map);
                self.registry.singleton_mut::<GameState>().unwrap().current_map = map.clone();
                self.registry.spawn().attach(Event::Respawn {  });
            }
            EngineEvent::Focused(focused) => {
                engine.set_cursor_grabbed(!*focused);
            },
        }
    }
}