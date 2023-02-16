use std::rc::Rc;
use image::DynamicImage;
use crate::Atlas;


#[derive(Clone, Debug, Default)]
pub struct TextureInfo {
    pub id:u32,
    pub width:f32,
    pub height:f32,
    pub image:Rc<DynamicImage>,
    pub atlas:Atlas
}

impl TextureInfo {
    pub fn aspect(&self) -> f32 {
        self.height / self.width
    } 
}