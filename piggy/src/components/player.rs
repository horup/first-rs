use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct Inventory {
    items: [(u32, f32); 32],
}

impl Inventory {
    pub fn get(&self, item_type: u32) -> f32 {
        return self.items.iter().find(|(item_type2, _)| {
            if *item_type2 == item_type {
                return true;
            } else {
                return false;
            }
        }).map_or(0.0, |item| item.1);
    }

    pub fn set(&mut self, item_type: u32, amount:f32) {
        let mut item = self.items.iter_mut().find(|(item_type2, _)| *item_type2 == item_type);
        if item.is_none() {
            item = self.items.iter_mut().find(|(item_type2, _)| *item_type2 == 0);
        }

        let item = item.expect("failed to set item");
        item.0 = item_type;
        item.1 = amount;
    }
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct Player {
    pub pokemoncards: u32,
    pub has_key_gold: bool,
    pub has_key_blue: bool,
    pub inventory: Inventory,
}
