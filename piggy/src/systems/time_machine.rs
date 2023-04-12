use engine_sdk::{registry::Registry, Engine, VirtualKeyCode};

use crate::singletons::Timemachine;

pub fn time_machine_tick(r:&mut Registry, e:&mut dyn Engine) {
    let mut timemachine = r.singleton_mut::<Timemachine>().unwrap().clone();
    *r.singleton_mut::<Timemachine>().unwrap() = Timemachine::default();

    let time = e.time();
    if time > timemachine.last_snapshot_time + 0.05 {
        timemachine.push(r);
        timemachine.last_snapshot_time = time;
    }
    if e.key_down(VirtualKeyCode::Back) {
        timemachine.pop(r);
    }

    *r.singleton_mut::<Timemachine>().unwrap() = timemachine;
}