use engine_sdk::{Engine, registry::{Registry, Facade}};
use crate::{PlayerEntity, ItemEntity, PiggyFacade};

pub fn item_system(registry:&mut Registry, _engine:&mut dyn Engine) {
    let mut despawns = Vec::new();
    let facade = registry.facade::<PiggyFacade>();
    for mut player in facade.query::<PlayerEntity>() {
        let pickup_radius = 0.5;

        for item in facade.query::<ItemEntity>() {
            let v = player.sprite.pos - item.sprite.pos;
            if v.length() < pickup_radius {
                let texture = item.sprite.texture;
                despawns.push(item.id);
                player.player.inventory.add(texture, 1.0);
                // state.flash.flash(0.2, 0.5);
            }
        }
    }

    despawns.iter_mut().for_each(|id|registry.despawn(*id));
}