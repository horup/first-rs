
use engine_sdk::{Game, Event as EngineEvent, VirtualKeyCode, registry::{Registry}, Sprite, Tilemap, SoundEmitter};
use crate::{systems::{self, DiagnosticsSystem}, components::{Player, Door, Mob, Activator, Health, Item, Effector, Event, PlayerCompletedFinalLevelEvent, StartEvent, Decoration, Trap, Modifiers, Expire}, singletons::{Global, Local, Campaign, Timemachine}};

pub struct Piggy {
    pub registry:Registry,
    pub diagnostics_system:DiagnosticsSystem
}

impl Default for Piggy {
    fn default() -> Self {
        let mut registry = Registry::new();

        registry.register_singleton::<Tilemap>();
        registry.register_singleton::<Global>();
        registry.register_singleton::<Campaign>();
        registry.register_singleton::<Local>();
        registry.register_singleton::<Timemachine>();

        registry.register_component::<Decoration>();
        registry.register_component::<Sprite>();
        registry.register_component::<SoundEmitter>();
        registry.register_component::<Player>();
        registry.register_component::<Door>();
        registry.register_component::<Mob>();
        registry.register_component::<Activator>();
        registry.register_component::<Health>();
        registry.register_component::<Item>();
        registry.register_component::<Effector>();
        registry.register_component::<Event>();
        registry.register_component::<Trap>();
        registry.register_component::<Modifiers>();
        registry.register_component::<Expire>();
        Self { registry, 
            diagnostics_system:DiagnosticsSystem::default()
        }
    }
}


impl Game for Piggy {
    fn init(&mut self, engine:&mut dyn engine_sdk::Engine) {
        systems::init_system(&mut self.registry, engine);
    }

    fn update(&mut self, engine:&mut dyn engine_sdk::Engine) {
        systems::time_machine_tick(&mut self.registry, engine);
        
        if engine.key_just_pressed(engine_sdk::VirtualKeyCode::Escape) {
            engine.set_cursor_grabbed(true);
        }
        if engine.mouse_down(0) {
            engine.set_cursor_grabbed(false);
        }

        if engine.key_just_pressed(VirtualKeyCode::Key1) {
            self.registry.spawn().attach(Event::PlayerCompletedFinalLevel(PlayerCompletedFinalLevelEvent {}));
        }

        systems::player_system(&mut self.registry, engine);
        systems::mob_system(&mut self.registry, engine);
        systems::physics_system(&mut self.registry, engine);
        systems::trapping(&mut self.registry);
        systems::item_pickup(&mut self.registry);
        systems::activator_system(&mut self.registry, engine);
        systems::door_system(&mut self.registry, engine);
        systems::effector_system(&mut self.registry, engine);
        systems::render_world_system(&mut self.registry, engine);
        systems::render_flash_system(&mut self.registry, engine);
        systems::ui_system(&mut self.registry, engine);
        systems::events_process(&mut self.registry, engine);
        systems::modifiers_update(&mut self.registry, engine.dt());
        systems::expiring(&mut self.registry, engine.dt());

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

        engine.play_sounds(&mut self.registry);
        self.diagnostics_system.calculate_fps(engine);
        self.diagnostics_system.render(engine);
    }

    fn on_event(&mut self, engine:&mut dyn engine_sdk::Engine, event:&EngineEvent) {
        match event {
            EngineEvent::Map { map } => {
                systems::start(&mut self.registry, &StartEvent { override_map: Some(map.clone()), level:0 }, engine);
            }
            EngineEvent::Focused(focused) => {
                engine.set_cursor_grabbed(!*focused);
            },
        }
    }
}