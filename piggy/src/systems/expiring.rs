use engine_sdk::registry::Registry;

use crate::PiggyFacade;

/// ensures entities that are marked for expiring will be despawned
pub fn expiring(r:&mut Registry, dt:f32) {
    let f = r.facade::<PiggyFacade>();
    for (id, mut expire) in f.expires.iter_mut() {
        expire.timer.tick(dt);
        let a = expire.timer.alpha_capped();
        let ma = 0.9;
        if a > ma {
            let a = a - ma;
            let a = a / (1.0 - ma);
            let a = 1.0 - a;
            if let Some(mut sprite) = f.sprites.get_mut(id) {
                sprite.opacity = Some(a);
            }
        }
        if expire.timer.is_done() {
            r.push(move |r|{
                r.despawn(id);
            })
        }
    }

    r.execute();
}