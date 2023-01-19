use engine_sdk::{glam::{Vec2, vec2}, Engine};

pub struct EditorCamera {
    pub pos:Vec2,
    pub dir:Vec2,
    pub zoom:f32,
    pub screen_size:Vec2,
}

impl EditorCamera {
    pub fn to_screen(&self, p:&Vec2) -> Vec2 {
        *p * self.zoom - self.pos * self.zoom + self.screen_size / 2.0
    }
    pub fn to_world(&self, p:&Vec2) -> Vec2 {
        if self.zoom > 0.0 {
            let mut p = *p - self.screen_size / 2.0;
            p = p / self.zoom;
            p = p + self.pos;
            return p;
        }

        Vec2::default()
    }
    pub fn update(&mut self, screen_size:Vec2, engine:&dyn Engine) {
        self.screen_size = screen_size;

        let my = engine.mouse_wheel_delta().y;
        let zoom_speed = 1.1;
        if my > 0.0 {
            self.zoom *=  zoom_speed;
        } else if my < 0.0 {
            self.zoom /=  zoom_speed;
        }

        if self.zoom < 1.0 {
            self.zoom = 1.0;
        }

        let move_speed = 1000.0 / self.zoom;
        self.pos += self.dir * engine.dt() * move_speed;
       
    }

}

impl Default for EditorCamera {
    fn default() -> Self {
        Self { pos: Default::default(), zoom: 64.0, screen_size:vec2(0.0, 0.0), dir:Vec2::default() }
    }
}
