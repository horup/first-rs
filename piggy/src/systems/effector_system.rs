use engine_sdk::{Engine, world::World};

use crate::PlayerEntity;

pub fn effector_system(world: &mut World, _engine: &mut dyn Engine) {
    for player_entity in world.query::<PlayerEntity>() {
        // mut near = Vec::new();
        let _player_pos = player_entity.sprite.pos;
        let _radius = 1.0;
       /* world.query_around(player_pos.truncate(), radius, &mut near);
        for id in near.drain(..) {
            if let Some(effector) = world.effectors.get(id) {
                match effector {
                    crate::components::Effector::ExitMarker => {
                        panic!("you won!");
                    }
                }
            }
        }*/
    }
}
