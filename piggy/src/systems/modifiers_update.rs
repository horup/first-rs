use engine_sdk::registry::Registry;

use crate::components::{Modifiers, Modifier};

pub fn modifiers_update(r:&mut Registry, dt:f32) {
    for (_, mut modifiers) in r.components::<Modifiers>().iter_mut() {
        {
            let mut modifiers_org = modifiers.modifiers.clone();
            let mut modifiers_new = Vec::new();
            for modifier in modifiers_org.iter_mut() {
                match modifier {
                    Modifier::Trapped { expire } => {
                        expire.tick(dt);
                        if !expire.is_done() {
                            modifiers_new.push(modifier.clone());
                        }
                    },
                }
            }
            modifiers.modifiers = modifiers_new;
        }
    }
}