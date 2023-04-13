use engine_sdk::{registry::Registry, Engine, SoundEmitter};

use crate::{components::{Event, Player}, sounds};

pub fn events_process(r:&mut Registry, engine:&mut dyn Engine) {
    let mut events = Vec::with_capacity(64);
    for (id, e) in r.components::<Event>().iter() {
        events.push(e.clone());
        r.push(move |r|r.despawn(id));
    }
    r.execute();
    for e in events.iter() {
        match e {
            Event::Empty => {},
            Event::PlayerCought(_) => {
                for (id, _) in r.components::<SoundEmitter>().iter() {
                    r.push(move |r| r.despawn(id));
                }
                r.execute();
                r.spawn().attach(SoundEmitter::once(sounds::COUGHT));
            },
            Event::PlayerEscaped(_) => {
                for (id, _) in r.components::<SoundEmitter>().iter() {
                    r.push(move |r| r.despawn(id));
                }
                r.execute();
                r.spawn().attach(SoundEmitter::once(sounds::WIN));
            },
            Event::PlayerCompletedFinalLevel(_) => {
                for (id, _) in r.components::<SoundEmitter>().iter() {
                    r.push(move |r| r.despawn(id));
                }
                r.execute();
                r.spawn().attach(SoundEmitter::once(sounds::FINAL));
                r.push(|r|{
                    for (_, mut player) in r.components::<Player>().iter_mut() {
                        player.state.set_final();
                    }
                });
            },
            Event::Start(start_event) => {
                crate::systems::start(r, start_event, engine);
            },
        }
    }

    r.execute();
}