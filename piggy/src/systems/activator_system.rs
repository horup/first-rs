use engine_sdk::{Engine, registry::{Registry, Facade}, SoundEmitter};

use crate::{PiggyFacade, ActivateeEntity, ActivatorEntity, components::{Inventory}, PlayerEntity, sounds};

pub fn activator_system(registry:&mut Registry, _engine:&mut dyn Engine) {
    let facade = registry.facade::<PiggyFacade>();
    let query_radius = 1.0;
    let mut player_inventory = Inventory::default();
    for player in facade.query::<PlayerEntity>() {
        player_inventory = player.player.inventory;
    }
    for activatee in facade.query::<ActivateeEntity>() {
        let pos = activatee.sprite.pos;
        for activator in facade.query::<ActivatorEntity>() {
            let v = pos - activator.sprite.pos;
            if v.length() < query_radius {
                match *activator.activator {
                    crate::components::Activator::Door { key } => {
                        let can_open = match key {
                            Some(key) => {
                                let mut can_open = false;
                                if player_inventory.has(key) {
                                    can_open = true;
                                }

                                can_open
                            },
                            None => true,
                        };
                        if can_open {
                            if let Some(mut door) = activator.door {
                                if door.openess == 0.0 {
                                    registry.push(|r|{
                                        r.spawn().attach(SoundEmitter::once(sounds::DOOR_OPEN));
                                    });
                                }
                                door.open();
                            }
                        }
                    },
                }
            }
        }

    }

    registry.execute();
}