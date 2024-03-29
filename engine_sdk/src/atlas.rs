#[derive(Copy, Clone, Debug)]
pub struct Atlas {
    pub columns:u8,
    pub rows:u8,
    pub size:u16
}

impl Default for Atlas {
    fn default() -> Self {
        Self { rows: 1, columns: 1, size:1 }
    }
}

impl Atlas {
    pub fn count(&self) -> u16 {
        self.rows as u16 * self.columns as u16
    }
    pub fn new(columns:u8, rows:u8, size:u16) -> Self {
        Self { rows, columns, size }
    }
    pub fn w(&self) -> f32 {
        1.0 / self.columns as f32
    }

    pub fn h(&self) -> f32 {
        1.0 / self.rows as f32
    }

    pub fn u(&self, index:u16) -> [f32;2] {
        let a = 1.0 / self.size as f32 / 2.0;
        let index = index % self.count();
        let x = index % self.columns as u16;
        let x = x as f32 / self.columns as f32;
        let a = a / 10.0;
        [x + a, x+self.w() - a]
    }

    pub fn v(&self, index:u16) -> [f32;2] {
        let a = 1.0 / self.size as f32 / 2.0;
        let index = index % self.count();
        let y = index / self.columns as u16;
        let y = y as f32 / self.rows as f32;
        let a = a / 10.0;
        [y + a , y+self.h() - a]
    }
}