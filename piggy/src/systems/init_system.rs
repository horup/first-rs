use engine_sdk::{LoadAtlasParams, Engine, EditorProps, image, Atlas, Map, registry::Registry};
use crate::{textures, Signal, Start, sounds};

pub fn init_system(registry:&mut Registry, engine:&mut dyn Engine, start_signals:&mut Signal<Start>) {
    macro_rules! wall {
        ($id:expr, $path:expr) => {
            engine.load_atlas($id, &image::load_from_memory(include_bytes!($path)).unwrap(), LoadAtlasParams {
                editor_props:EditorProps::wall(),
                ..Default::default()
            });
        };
    }

    macro_rules! thing {
        ($id:expr, $path:expr, $atlas:expr) => {
            engine.load_atlas
            ($id, &image::load_from_memory(include_bytes!($path)).unwrap(), LoadAtlasParams {
                atlas:$atlas,
                editor_props:EditorProps::thing(),
                ..Default::default()
            });
        };
    }

    macro_rules! sound {
        ($id:expr, $path:expr) => {
            engine.load_sound($id, include_bytes!($path));
        };
    }
    
    wall!(textures::WALL_BRICK, "../../assets/textures/wall_brick.png");
    wall!(textures::WALL_BUSH, "../../assets/textures/wall_bush.png");
    wall!(textures::WALL_WHITE, "../../assets/textures/wall_white.png");

    thing!(textures::THING_MARKER_SPAWN_PLAYER, "../../assets/textures/thing_player.png", Atlas::new(1, 1));
    thing!(textures::THING_VIKTOR, "../../assets/textures/thing_player_viktor.png", Atlas::new(1, 1));
    thing!(textures::THING_WILLIAM, "../../assets/textures/thing_player_william.png", Atlas::new(2, 1));
    thing!(textures::THING_DOOR_BLUE, "../../assets/textures/thing_door_blue.png", Atlas::new(1, 1));
    thing!(textures::THING_DOOR_WHITE, "../../assets/textures/thing_door_white.png", Atlas::new(1, 1));
    thing!(textures::THING_DOOR_GOLD, "../../assets/textures/thing_door_gold.png", Atlas::new(1, 1));
    thing!(textures::THING_ITEM_POKEMONCARD, "../../assets/textures/thing_item_pokemoncard.png", Atlas::new(1, 1));
    thing!(textures::THING_ITEM_KEY_GOLD, "../../assets/textures/thing_item_key_gold.png", Atlas::new(1, 1));
    thing!(textures::THING_ITEM_KEY_BLUE, "../../assets/textures/thing_item_key_blue.png", Atlas::new(1, 1));
    thing!(textures::THING_MONSTER_PIGGY, "../../assets/textures/thing_monster_piggy.png", Atlas::new(1, 1));
    thing!(textures::THING_PLANT, "../../assets/textures/thing_plant.png", Atlas::new(1, 1));
    thing!(textures::THING_MARKER_EXIT, "../../assets/textures/thing_marker_exit.png", Atlas::new(1, 1));
    wall!(textures::FLOOR_GREY, "../../assets/textures/floor_grey.png");
    wall!(textures::CEILING_GREY, "../../assets/textures/ceiling_grey.png");


    sound!(sounds::PICKUP, "../../assets/audio/pickup.ogg");
    sound!(sounds::PICKUP_KEY, "../../assets/audio/pickup_key.ogg");
    sound!(sounds::DOOR_OPEN, "../../assets/audio/door_open.ogg");
    sound!(sounds::DOOR_CLOSE, "../../assets/audio/door_close.ogg");
    //let map:Map = serde_json::from_str(include_str!("../../assets/maps/test.map")).unwrap();
    start_signals.push(Start::default());
    
}