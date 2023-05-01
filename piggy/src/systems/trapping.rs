use engine_sdk::{
    registry::{Facade, Registry},
    SpatialHashmap, Timer,
};

use crate::{PiggyFacade, TrapEntity, components::Expire};

pub fn trapping(r: &mut Registry) {
    let mut res = Vec::with_capacity(2);
    let f = r.facade::<PiggyFacade>();
    let mut q = SpatialHashmap::new(r);
    q.update_all();
    let traps = f.query::<TrapEntity>();
    let radius = 0.25;
    for mut trap in traps {
        if !trap.trap.triggered {
            q.query_around(trap.sprite.pos.truncate(), 1.0, &mut res);
            for id in res.iter() {
                if let Some(other_sprite) = f.sprites.get(*id) {
                    let v = other_sprite.pos - trap.sprite.pos;
                    if v.length() < radius {
                        if f.mobs.get(*id).is_some() {
                            if let Some(mut modifiers) = f.modifiers.get_mut(*id) {
                                trap.trap.triggered = true;
                                trap.sprite.pic.index = trap.sprite.pic.index - 1; 
                                let secs = 5.0;
                                modifiers.trap(secs);
                                let trap_id = trap.id;
                                r.push(move |r|{
                                    r.component_attach(trap_id, Expire {
                                        timer: Timer::new(secs),
                                    })
                                });
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    r.execute();
}
