use engine_sdk::Map;

#[derive(Clone, Default)]
pub struct Start {
    pub override_map:Option<Map>,
    pub level:usize
}