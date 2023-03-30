use engine_sdk::{Engine, registry::{Registry, Facade, Commands}};
use crate::{PlayerEntity, ItemEntity, PiggyFacade, singletons::Global};

pub fn item_system(registry:&mut Registry, _engine:&mut dyn Engine) {
    let mut commands = Commands::default();
    {
        let facade = registry.facade::<PiggyFacade>();
        for mut player in facade.query::<PlayerEntity>() {
            let pickup_radius = 0.5;

            for item in facade.query::<ItemEntity>() {
                let v = player.sprite.pos - item.sprite.pos;
                let id = item.id;
                if v.length() < pickup_radius {
                    let texture = item.sprite.texture;
                    commands.push(move |reg|{
                        reg.singleton_mut::<Global>().unwrap().flash.flash(0.2, 0.5);
                        reg.despawn(id);
                    });
                    player.player.inventory.add(texture, 1.0);
                }
            }
        }
    }

    commands.execute(registry);
}