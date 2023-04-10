use engine_sdk::{registry::Registry, Engine};

use crate::{components::{Event, Player}, sounds};

pub fn events_process(r:&mut Registry, engine:&mut dyn Engine) {
    for (_, e) in r.components::<Event>().iter() {
        match &*e {
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
        }
    }

    r.execute();
}