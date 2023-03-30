use engine_sdk::registry::{Registry, Commands};

use crate::components::Event;

pub fn cleanup(registry:&mut Registry) {
    let events = registry.components::<Event>();
    let mut commands = Commands::default();
    for (e, _) in events.iter() {
        commands.push(move |registry|{
            registry.despawn(e);
        });
    }
    commands.execute(registry);
}