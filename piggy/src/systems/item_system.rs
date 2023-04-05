use engine_sdk::{Engine, registry::{Registry, Facade, Commands}};
use crate::{PlayerEntity, ItemEntity, PiggyFacade, singletons::GameState, sounds};

pub fn item_system(registry:&mut Registry, engine:&mut dyn Engine) {
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
                        reg.singleton_mut::<GameState>().unwrap().flash.flash(0.2, 0.5);
                        reg.despawn(id);
                    });
                    engine.play_sound(sounds::PICKUP, 1.0);
                    player.player.inventory.add(texture, 1.0);
                }
            }
        }
    }

    commands.execute(registry);
}