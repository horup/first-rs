use engine_sdk::registry::Registry;

use crate::PiggyFacade;

/// ensures entities that are marked for expiring will be despawned
pub fn expiring(r:&mut Registry, dt:f32) {
    for (id, mut expire) in r.facade::<PiggyFacade>().expires.iter_mut() {
        expire.timer.tick(dt);
        if expire.timer.is_done() {
            r.push(move |r|{
                r.despawn(id);
            })
        }
    }

    r.execute();
}