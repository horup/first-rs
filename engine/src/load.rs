use engine_sdk::{image::DynamicImage, LoadAtlasParams};


pub enum Load {
    Atlas {
        id:u32,
        img:DynamicImage,
        params:LoadAtlasParams
    },
    Sound {
        id:u32,
        bytes:Vec<u8>
    }
}