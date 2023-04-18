use engine_sdk::{Engine, registry::{Registry, Facade}, Tilemap};

use crate::{singletons::Global, PiggyFacade, DecorationEntity};

pub fn physics_system(registry:&mut Registry, engine:&mut dyn Engine) {
    {
        let mut tilemap = registry.singleton_mut::<Tilemap>().unwrap();
        for e in registry.facade::<PiggyFacade>().query::<DecorationEntity>() {
            let index = e.sprite.pos.as_ivec3().truncate();
            if let Some(cell) = tilemap.grid.get_mut(index.into()) {
                cell.clips = true;
            }
        }
    }
    let mut global = registry.singleton_mut::<Global>().unwrap();
    engine.physics_step(registry, &mut global.collisions);
}