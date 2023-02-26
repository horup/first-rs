use engine_sdk::{Camera, Entities, SpriteId, Sprite, Grid, Cell};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct State {
    pub camera: Camera,
    pub sprites: Entities<SpriteId, Sprite>,
    pub grid: Grid<Cell>,
    pub player_id: Option<SpriteId>
}