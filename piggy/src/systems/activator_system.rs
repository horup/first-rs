use engine_sdk::Engine;
use crate::State;

pub fn activator_system(state:&mut State, engine:&mut dyn Engine) {
    // check proximity to door
    if let Some(player_id) = state.player_id {
        if let Some(player) = state.sprites.get(player_id) {
            let player_pos = player.pos;
            let mut near = Vec::new();
            let mut world = state.as_world();
            let radius = 1.0;
            world.query_around(player_pos.truncate(), radius, &mut near);
            for id in near.drain(..) {
                if let Some(activator) = state.activators.get_mut(id) {
                    match activator {
                        crate::components::Activator::Door { key } => {
                            if let Some(door) = state.doors.get_mut(id) {
                                door.open();
                            }
                        },
                    }
                }
            }
        }
    }
}