
use engine_sdk::{Game, Event as EngineEvent, VirtualKeyCode, registry::{Registry}, Sprite, Tilemap};
use crate::{systems::{self, DiagnosticsSystem}, components::{Player, Door, Mob, Activator, Health, Item, Effector, EmitSound, Event}, singletons::GameState, Campaign, Signal, Start, listeners, sounds};

pub struct Piggy {
    pub registry:Registry,
    pub start_signals:Signal<Start>,
    pub campaign:Campaign,
    pub diagnostics_system:DiagnosticsSystem
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
        registry.register_singleton::<Tilemap>();
        registry.register_singleton::<GameState>();
        registry.register_component::<EmitSound>();
        registry.register_component::<Event>();
        Self { registry, 
            campaign:Campaign::new(), 
            start_signals:Signal::new(),
            diagnostics_system:DiagnosticsSystem::default()
        }
    }
}


impl Game for Piggy {
    fn init(&mut self, engine:&mut dyn engine_sdk::Engine) {
        systems::init_system(&mut self.registry, engine, &mut self.start_signals);
    }

    fn update(&mut self, engine:&mut dyn engine_sdk::Engine) {
        if engine.key_just_pressed(engine_sdk::VirtualKeyCode::Escape) {
            engine.set_cursor_grabbed(true);
        }
        if engine.mouse_down(0) {
            engine.set_cursor_grabbed(false);
        }

        if engine.key_just_pressed(VirtualKeyCode::Key1) {
            engine.play_music(sounds::PICKUP_KEY);
        }

        if engine.key_just_pressed(VirtualKeyCode::Key2) {
            engine.stop_music();
        }

        
        systems::player_system(&mut self.registry, engine, &mut self.start_signals);
        systems::mob_system(&mut self.registry, engine);
        systems::physics_system(&mut self.registry, engine);
        systems::item_pickup(&mut self.registry);
        systems::activator_system(&mut self.registry, engine);
        systems::door_system(&mut self.registry, engine);
        systems::effector_system(&mut self.registry, engine);
        systems::render_world_system(&mut self.registry, engine);
        systems::render_flash_system(&mut self.registry, engine);
        systems::ui_system(&mut self.registry, engine);
        systems::sound_playback(&mut self.registry, engine);

        for start_signal in self.start_signals.drain() {
            listeners::on_start(&mut self.registry, &self.campaign, &start_signal, engine);
        }

        systems::events_process(&mut self.registry, engine);
        systems::events_cleanup(&mut self.registry);

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

        self.diagnostics_system.calculate_fps(engine);
        self.diagnostics_system.render(engine);
    }

    fn on_event(&mut self, engine:&mut dyn engine_sdk::Engine, event:&EngineEvent) {
        match event {
            EngineEvent::Map { map } => {
                self.start_signals.push(Start { override_map: Some(map.clone()), level:0 });
            }
            EngineEvent::Focused(focused) => {
                engine.set_cursor_grabbed(!*focused);
            },
        }
    }
}