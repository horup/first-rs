use engine_sdk::{Engine, registry::{Registry, Facade}};
use crate::{PlayerEntity, PiggyFacade};

pub fn effector_system(registry: &mut Registry, _engine: &mut dyn Engine) {
    for player_entity in registry.facade::<PiggyFacade>().query::<PlayerEntity>() {
        // mut near = Vec::new();
        let _player_pos = player_entity.sprite.pos;
        let _radius = 1.0;
       /* registry.query_around(player_pos.truncate(), radius, &mut near);
        for id in near.drain(..) {
            if let Some(effector) = registry.effectors.get(id) {
                match effector {
                    crate::components::Effector::ExitMarker => {
                        panic!("you won!");
                    }
                }
            }
        }*/
    }
}
