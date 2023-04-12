use engine_sdk::{registry::Registry, Engine};

use crate::{components::{Event, Player}, sounds, listeners::on_start, Campaign};

pub fn events_process(r:&mut Registry, engine:&mut dyn Engine, campaign:&Campaign) {
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
                engine.stop_music();
                engine.play_sound(sounds::COUGHT, 1.0);
            },
            Event::PlayerEscaped(_) => {
                engine.stop_music();
                engine.play_sound(sounds::WIN, 1.0);
            },
            Event::PlayerCompletedFinalLevel(_) => {
                engine.stop_music();
                engine.play_sound(sounds::FINAL, 1.0);
                r.push(|r|{
                    for (_, mut player) in r.components::<Player>().iter_mut() {
                        player.state.set_final();
                    }
                });
            },
            Event::Start(start_event) => {
                on_start(r, campaign, start_event, engine);
            },
        }
    }

    r.execute();
}