use crate::State;
use engine_sdk::Engine;

pub fn effector_system(state: &mut State, engine: &mut dyn Engine) {
    if let Some(player_id) = state.player_id {
        if let Some(player) = state.sprites.get(player_id) {
            let mut near = Vec::new();
            let player_pos = player.pos;
            let world = state.as_world();
            let radius = 1.0;
            world.query_around(player_pos.truncate(), radius, &mut near);
            for id in near.drain(..) {
                if let Some(effector) = state.effectors.get(id) {
                    match effector {
                        crate::components::Effector::ExitMarker => {
                            panic!("you won!");
                        }
                    }
                }
            }
        }
    }
}
