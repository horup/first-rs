use engine_sdk::registry::{Component, uuid::uuid};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PlayerCoughtEvent {
    
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PlayerEscapedEvent {
    
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PlayerCompletedFinalLevelEvent {
    
}



#[derive(Clone, Serialize, Deserialize)]
pub enum Event {
    Empty,
    PlayerCought(PlayerCoughtEvent),
    PlayerEscaped(PlayerEscapedEvent),
    PlayerCompletedFinalLevel(PlayerCompletedFinalLevelEvent)
}

impl Default for Event {
    fn default() -> Self {
        Self::Empty
    }
}

impl Component for Event {
    fn type_id() -> engine_sdk::registry::uuid::Uuid {
        uuid!("97f99432-3ee1-4ca8-929a-14a73af93e9d")
    }
}