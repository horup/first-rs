use engine_sdk::{
    registry::{Facade, Registry},
    SpatialHashmap,
};

use crate::{MobEntity, PiggyFacade, TrapEntity};

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
                                modifiers.trap(5.0);
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
