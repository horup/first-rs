use engine_sdk::registry::Registry;
use crate::components::Event;

pub fn cleanup_events(r:&mut Registry) {
    for (id, _) in r.components::<Event>().iter() {
        r.push(move |r|r.despawn(id));
    }

    r.execute();
}