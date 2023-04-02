use engine_sdk::{Engine, registry::{Registry, Facade}, SpatialHashmap};
use crate::{PlayerEntity, PiggyFacade};

pub fn effector_system(registry: &mut Registry, _engine: &mut dyn Engine) {
    let facade = registry.facade::<PiggyFacade>();
    let mut spatial = SpatialHashmap::new(registry);
    let mut near = Vec::with_capacity(64);
    for player_entity in facade.query::<PlayerEntity>() {
        let radius = 1.0;
        let pos = player_entity.sprite.pos;

        spatial.query_around(pos.truncate(), radius, &mut near);
        for id in near.drain(..) {
            if let Some(effector) = facade.effectors.get(id) {
                match &*effector {
                    crate::components::Effector::ExitMarker => {
                        dbg!("you won");
                    },
                }
            }
        }
    }
}
