use engine_sdk::{registry::Registry, Engine};

use crate::components::EmitSound;

pub fn sound_playback(r:&mut Registry, engine:&mut dyn Engine) {
    
    for (e, emit_sound) in r.components::<EmitSound>().iter() {
        engine.play_sound(emit_sound.sound, 1.0);
        r.push(move |r|r.despawn(e));
    }

    r.execute();
}