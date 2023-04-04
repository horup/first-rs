use engine_sdk::{glam::Vec3, glam::Vec2, registry::{Component, uuid::{uuid, Uuid}}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, Default)]
pub struct Mob {
    pub active:bool,
    pub last_known_pos:Option<Vec3>,
    pub can_see_player:bool,
    pub dir:Vec2,
    pub is_killer:bool
}

impl Component for Mob {
    fn type_id() -> Uuid {
        uuid!("d7b2f167-0b77-4a7b-aeb6-77ae0ebae15b")
    }
}