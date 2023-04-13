use engine_sdk::{registry::{Registry, Facade}, SoundEmitter};
use crate::{PlayerEntity, ItemEntity, PiggyFacade, singletons::Global};

pub fn item_pickup(registry:&mut Registry) {
    let facade = registry.facade::<PiggyFacade>();
    for mut player in facade.query::<PlayerEntity>() {
        let pickup_radius = 0.5;

        for item in facade.query::<ItemEntity>() {
            let v = player.sprite.pos - item.sprite.pos;
            let id = item.id;
            let pickup_sound = item.item.pickup_sound;
            if v.length() < pickup_radius {
                let texture = item.sprite.texture;
                registry.push(move |reg|{
                    reg.singleton_mut::<Global>().unwrap().flash.flash(0.2, 0.5);
                    reg.despawn(id);
                    reg.spawn().attach(SoundEmitter::once(pickup_sound));
                });
                
                player.player.inventory.add(texture, 1.0);
            }
        }
    }

    registry.execute();
}