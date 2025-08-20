use esp_hal::{time::{Duration, Instant}};

use crate::states::Event;

pub struct Timer {
    start_time: Instant,
    duration: Duration,
}

impl Timer {
    pub(crate) fn new() -> Self {
        Timer { 
            start_time: Instant::now(), 
            duration: Duration::from_micros(0),
        }
    }

    pub fn set(&mut self, duration: Duration) {
        self.duration = duration;
        self.start_time = Instant::now();
    }
}

pub const TIMER_EXPIRED: Event<Timer> = |target: &Timer| {
    target.start_time.elapsed() > target.duration
};