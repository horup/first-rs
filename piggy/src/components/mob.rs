use engine_sdk::{glam::Vec3, glam::Vec2};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, Default)]
pub struct Mob {
    pub last_known_pos:Option<Vec3>,
    pub can_see_player:bool,
    pub dir:Vec2
}