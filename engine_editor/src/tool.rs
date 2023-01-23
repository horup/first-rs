#[derive(PartialEq, Eq, Hash)]
pub enum Tool {
    PlaceWall,
    PlaceThing
}

impl Default for Tool {
    fn default() -> Self {
        Self::PlaceWall
    }
}