use engine_sdk::{Map, registry::{Component, uuid::uuid}};
use serde::{Serialize, de::{DeserializeOwned, Visitor}, Deserialize};

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Campaign {
    levels:Vec<Level>
}

impl Campaign {
    pub fn new() -> Self {
        let mut levels = Vec::new();
        //levels.push(Level::from_bytes(include_str!("../../assets/maps/001.map")));
        //levels.push(Level::from_bytes(include_str!("../../assets/maps/002.map")));
        Self {
            levels
        }
    }

    pub fn get(&self, index:usize) -> Option<&Level> {
        self.levels.get(index)
    }
}

impl Default for Campaign {
    fn default() -> Self {
        Self::new()
    }
}

impl Serialize for Campaign {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_bool(true)
    }
}

impl<'de> Deserialize<'de> for Campaign {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let _ = bool::deserialize(deserializer);
        Ok(Self::default())
    }
}

impl Component for Campaign {
    fn type_id() -> engine_sdk::registry::uuid::Uuid {
        uuid!("181d5139-2391-4ac4-9296-547f533fb0a8")
    }
}

