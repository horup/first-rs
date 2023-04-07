use engine_sdk::{Engine, registry::{Registry, Facade}, SpatialHashmap};
use crate::{PlayerEntity, PiggyFacade, components::{Event, PlayerEscapedEvent}};

pub fn effector_system(registry: &mut Registry, _engine: &mut dyn Engine) {
    let facade = registry.facade::<PiggyFacade>();
    let mut spatial = SpatialHashmap::new(registry);
    let mut near = Vec::with_capacity(64);
    for mut player_entity in facade.query::<PlayerEntity>().filter(|p|p.player.state.is_active()) {
        let radius = 1.0;
        let pos = player_entity.sprite.pos;

        spatial.query_around(pos.truncate(), radius, &mut near);
        for id in near.drain(..) {
            if let Some(effector) = facade.effectors.get(id) {
                match &*effector {
                    crate::components::Effector::ExitMarker => {
                        let _player_id = player_entity.id;
                        registry.push(move |r|{
                            r.spawn().attach(Event::PlayerEscaped(PlayerEscapedEvent {  }));
                        });
                        player_entity.player.state.set_won();
                    },
                }
            }
        }
    }
    
    registry.execute();
}
