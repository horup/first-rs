pub mod textures {
    pub const WALL_BRICK:u32                    = 1;
    pub const WALL_BUSH:u32                     = 2;
    pub const WALL_WHITE:u32                    = 3;
    pub const THING_MARKER_SPAWN_PLAYER:u32     = 4;
    pub const THING_VIKTOR:u32                  = 5;
    pub const THING_WILLIAM:u32                 = 6;
    pub const FLOOR_GREY:u32                    = 7;
    pub const CEILING_GREY:u32                  = 8;
    pub const THING_DOOR_BLUE:u32               = 9;
    pub const THING_DOOR_WHITE:u32              = 10;
    pub const THING_DOOR_GOLD:u32               = 11;
    pub const THING_ITEM_KEY_BLUE:u32           = 12;
    pub const THING_ITEM_KEY_GOLD:u32           = 13;
    pub const THING_MONSTER_PIGGY:u32           = 14;
    pub const THING_ITEM_POKEMONCARD:u32        = 15;
    pub const THING_PLANT:u32                   = 16;
    pub const THING_MARKER_EXIT:u32             = 17;
    pub const WALLS:u32                         = 18;
    pub const MARKERS:u32                       = 19;
}

pub mod atlases {
    pub const MARKERS:u32                       = 1;
    pub const CREATURES:u32                     = 2;
    pub const TILES:u32                         = 3;
    pub const DECORATIONS:u32                   = 4;
    pub const ITEMS:u32                         = 5;
    pub const DOORS:u32                         = 6;
}

pub mod sounds {
    pub const PICKUP:u32 = 1;
    pub const DOOR_OPEN:u32 = 2;
    pub const DOOR_CLOSE:u32 = 3;
    pub const PICKUP_KEY:u32 = 4;
    pub const MUSIC01:u32 = 5;
    pub const COUGHT:u32 = 6;
    pub const WIN:u32 = 7;
    pub const LOSE:u32 = 8;
    pub const FINAL:u32 = 9;
    pub const TRAP:u32 = 10;
}

pub mod items {
    use engine_sdk::Pic;
    use crate::atlases;

    pub const POKEMONCARD:Pic = Pic::new(atlases::ITEMS, 0);
    pub const KEY_BLUE:Pic =  Pic::new(atlases::ITEMS, 1);
    pub const KEY_GOLD:Pic =  Pic::new(atlases::ITEMS, 2);
    pub const TRAP:Pic =  Pic::new(atlases::ITEMS, 3);
}