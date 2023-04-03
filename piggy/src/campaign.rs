use engine_sdk::Map;

pub struct Level {
    pub map:Map
}

impl Level {
    pub fn from_bytes(bytes:&[u8]) -> Self {
        let map = bincode::deserialize::<Map>(bytes).unwrap();
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
        levels.push(Level::from_bytes(include_bytes!("../assets/maps/001.map")));
        Self {
            levels
        }
    }

    pub fn get(&self, index:usize) -> Option<&Level> {
        self.levels.get(index)
    }
}