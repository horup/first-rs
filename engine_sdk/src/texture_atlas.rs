use crate::Atlas;
use image::DynamicImage;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct EditorProps {
    pub is_thing: bool,
    pub is_wall: bool,
}

impl EditorProps {
    pub fn all() -> Self {
        Self {
            is_thing: true,
            is_wall: true,
        }
    }

    pub fn thing() -> Self {
        Self {
            is_thing:true,
            ..Default::default()
        }
    }

    pub fn wall() -> Self {
        Self {
            is_wall:true,
            ..Default::default()
        }
    }
}

impl Default for EditorProps {
    fn default() -> Self {
        Self::all()
    }
}

#[derive(Clone, Debug, Default)]
pub struct TextureAtlas {
    id: u32,
    image: Rc<DynamicImage>,
    atlas: Atlas,
    width: u32,
    height: u32,
    editor_props: EditorProps,
}

impl TextureAtlas {
    pub fn new(id: u32, image: Rc<DynamicImage>, atlas: Atlas, editor_props: EditorProps) -> Self {
        let width = image.width();
        let height = image.height();
        Self {
            id,
            image,
            atlas,
            width,
            height,
            editor_props,
        }
    }

    pub fn editor_props(&self) -> &EditorProps {
        &self.editor_props
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

    pub fn aspect(&self, atlas_index: u16) -> f32 {
        self.height(atlas_index) as f32 / self.width(atlas_index) as f32
    }

    pub fn width(&self, _atlas_index: u16) -> u32 {
        self.width / self.atlas.columns as u32
    }

    pub fn height(&self, _atlas_index: u16) -> u32 {
        self.height / self.atlas.rows as u32
    }
}
