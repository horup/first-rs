use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Timer {
    timeout:f32,
    timer:f32
}

impl Timer {
    pub fn new(timeout:f32) -> Self {
        Self {
            timeout,
            timer:0.0
        }
    }

    pub fn reset(&mut self) {
        self.timer = 0.0;
    }    

    pub fn tick(&mut self, dt:f32) {
        self.timer += dt;
    }

    pub fn is_done(&self) -> bool {
        self.timer >= self.timeout
    }

    pub fn alpha(&self) -> f32 { 
        if self.timeout > 0.0 {
            return self.timer / self.timeout;
        }

        1.0
    }

    pub fn alpha_capped(&self) -> f32 {
        let alpha = self.alpha();
        if alpha < 0.0 {
            return 0.0;
        } else if alpha > 1.0 {
            return 1.0;
        }
        alpha
    }
}