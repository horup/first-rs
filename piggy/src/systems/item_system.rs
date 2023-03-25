use engine_sdk::{Engine, world::{World, Entity}};
use crate::{PlayerEntity, ItemEntity};

pub fn item_system(world:&mut World, _engine:&mut dyn Engine) {
    let mut despawns = Vec::new();
    for mut player in world.query::<PlayerEntity>() {
        let pickup_radius = 0.5;

        for item in world.query::<ItemEntity>() {
            let v = player.sprite.pos - item.sprite.pos;
            if v.length() < pickup_radius {
                let texture = item.sprite.texture;
                despawns.push(item.id);
                player.player.inventory.add(texture, 1.0);
                // state.flash.flash(0.2, 0.5);
            }
        }
    }

    despawns.iter_mut().for_each(|id|world.despawn(*id));
}