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
    for trap in traps {
        q.query_around(trap.sprite.pos.truncate(), 0.5, &mut res);
        for id in res.iter() {
            if f.mobs.get(*id).is_some() {
                let id = id.clone();
                r.push(move |r|{
                    r.despawn(id);
                });
            }
        }
    }

    r.execute();
}
