use engine_sdk::{registry::{Registry, Facade}};
use crate::{PlayerEntity, ItemEntity, PiggyFacade, singletons::GameState, sounds, components::EmitSound, textures};

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
                    reg.singleton_mut::<GameState>().unwrap().flash.flash(0.2, 0.5);
                    reg.despawn(id);
                    
                    reg.spawn().attach(EmitSound {
                        looping: false,
                        sound: pickup_sound,
                    });
                });
                
                player.player.inventory.add(texture, 1.0);
            }
        }
    }

    registry.execute();
}