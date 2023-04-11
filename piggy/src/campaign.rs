use engine_sdk::Map;

pub struct Level {
    pub map:Map
}

impl Level {
    pub fn from_bytes(map_json:&str) -> Self {
        let map = serde_json::from_str::<Map>(map_json).unwrap();
        Self {
            map
        }
    }
}

pub struct Campaign {
    levels:Vec<Level>
}

impl Campaign {
    pub fn new() -> Self {
        let mut levels = Vec::new();
        levels.push(Level::from_bytes(include_str!("../assets/maps/test.map")));
        // levels.push(Level::from_bytes(include_str!("../assets/maps/002.map")));
        Self {
            levels
        }
    }

    pub fn get(&self, index:usize) -> Option<&Level> {
        self.levels.get(index)
    }
}