use std::rc::Rc;
use image::DynamicImage;
use crate::Atlas;


#[derive(Clone, Debug, Default)]
pub struct TextureAtlas {
    id:u32,
    image:Rc<DynamicImage>,
    atlas:Atlas,
    width:u32,
    height:u32
}

impl TextureAtlas {
    pub fn new(id:u32, image:Rc<DynamicImage>, atlas:Atlas) -> Self {
        let width = image.width();
        let height = image.height();
        Self {
            id,
            image,
            atlas,
            width,
            height
        }
    }

    pub fn atlas(&self) -> Atlas {
        self.atlas
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn image(&self) -> Rc<DynamicImage> {
        self.image.clone()
    }

    pub fn aspect(&self, atlas_index:u16) -> f32 {
        self.height(atlas_index) as f32 / self.width(atlas_index) as f32
    } 

    pub fn width(&self, _atlas_index:u16) -> u32 {
        self.width / self.atlas.columns as u32
    }

    pub fn height(&self, _atlas_index:u16) -> u32 {
        self.height / self.atlas.rows as u32
    }
}