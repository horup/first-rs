use std::rc::Rc;
use image::DynamicImage;
use crate::Atlas;


#[derive(Clone, Debug, Default)]
pub struct TextureInfo {
    id:u32,
    image:Rc<DynamicImage>,
    atlas:Atlas,
    width:u32,
    height:u32
}

impl TextureInfo {
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

    pub fn aspect(&self) -> f32 {
        self.height() as f32 / self.width() as f32
    } 

    pub fn width(&self) -> u32 {
        self.width / self.atlas.columns as u32
    }

    pub fn height(&self) -> u32 {
        self.height / self.atlas.rows as u32
    }
}