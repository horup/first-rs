use engine_sdk::Engine;
use crate::State;

pub fn activator_system(state:&mut State, engine:&mut dyn Engine) {
    // check proximity to door
   /*  if let Some(player_id) = state.player_id {
        if let Some(player_sprite) = state.sprites.get(player_id) {
            
            let player_pos = player_sprite.pos;
            let mut near = Vec::new();
            let mut world = state.as_world();
            let radius = 1.0;
            world.query_around(player_pos.truncate(), radius, &mut near);
            for id in near.drain(..) {
                if let Some(activator) = state.activators.get_mut(id) {
                    match activator {
                        crate::components::Activator::Door { key } => {
                            let can_open = match key {
                                Some(key) => {
                                    let mut can_open = false;
                                    if let Some(player) = state.players.get(player_id) {
                                        if player.inventory.has(*key) {
                                            can_open = true;
                                        }
                                    } 

                                    can_open
                                },
                                None => true,
                            };
                            if can_open {
                                if let Some(door) = state.doors.get_mut(id) {
                                    door.open();
                                }
                            }
                        },
                    }
                }
            }
        }
    }*/

    for (id, _) in state.sprites.iter() {
        if let Some(activatee_thing) = state.activatee_thing(id) {
            let player_pos = activatee_thing.sprite.pos;
            let mut near = Vec::new();
            let mut world = state.as_world();
            let radius = 1.0;
            world.query_around(player_pos.truncate(), radius, &mut near);
            for id in near.drain(..) {
                if let Some(activator_thing) = state.activator_thing(id) {
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
                                if let Some(door) = state.doors.get_mut(id) {
                                    door.open();
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}