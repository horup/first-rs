use engine_sdk::{Engine, world::World};
use crate::{PlayerEntity, ItemEntity};

pub fn item_system(world:&mut World, _engine:&mut dyn Engine) {
    for mut player in world.query::<PlayerEntity>() {
        let pickup_radius = 0.5;

        for item in world.query::<ItemEntity>() {
            let v = player.sprite.pos - item.sprite.pos;
            if v.length() < pickup_radius {
                let texture = item.sprite.texture;
                world.despawn(item.id);
                player.player.inventory.add(texture, 1.0);
                // state.flash.flash(0.2, 0.5);
            }
        }
    }

}