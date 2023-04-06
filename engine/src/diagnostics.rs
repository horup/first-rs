use instant::{Instant, Duration};

pub struct Diagnostics {
    last_tick:Instant,
    pub frame_time:Duration
}

impl Diagnostics {
    pub fn now_ms(&self) -> u128 {
        let now = Instant::now();
        now.elapsed().as_millis()
    }
    pub fn measure_frame_time(&mut self) {
        let tick = Instant::now();
        self.frame_time = tick - self.last_tick;
        self.last_tick = tick;
    }
}

impl Default for Diagnostics {
    fn default() -> Self {
        Self { last_tick: Instant::now(), frame_time: Default::default() }
    }
}

