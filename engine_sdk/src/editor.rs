pub struct LoadWallsParam {
    pub atlas:u32,
}

pub struct LoadEntityParam {
    pub atlas:u32
}

pub trait Editor {
    fn load_walls(&mut self, param:LoadWallsParam);
    fn load_entity(&mut self, param:LoadEntityParam);
}