use engine_sdk::{Engine, registry::Registry};

pub fn activator_system(_registry:&mut Registry, _engine:&mut dyn Engine) {
   /* for id in state.entities.iter() {
        if let Some(activatee_thing) = state.activatee_entity(id) {
            let player_pos = activatee_thing.sprite.pos;
            let mut near = Vec::new();
            let mut registry = state.as_registry();
            let radius = 1.0;
            registry.query_around(player_pos.truncate(), radius, &mut near);
            for id in near.drain(..) {
                if let Some(activator_thing) = state.activator_entity(id) {
                    match activator_thing.activator {
                        crate::components::Activator::Door { key } => {
                            let can_open = match key {
                                Some(key) => {
                                    let mut can_open = false;
                                    if activatee_thing.player_thing.player.inventory.has(*key) {
                                        can_open = true;
                                    }

                                    can_open
                                },
                                None => true,
                            };
                            if can_open {
                                if let Some(door) = state.doors.get_mut2(id) {
                                    door.open();
                                }
                            }
                        },
                    }
                }
            }
        }
    }*/
}