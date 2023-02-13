use std::time::Instant;

pub struct Timer {
    pub last_instant: Instant
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            last_instant: Instant::now()
        }
    }

    pub fn set_time_and_get_ellapsed(&mut self, new_time: Instant) -> f32 {
        let ellapsed_time = (new_time - self.last_instant).as_secs_f32();
        self.last_instant = new_time;
        return ellapsed_time;
    }
}