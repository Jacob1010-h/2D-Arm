use std::{thread::sleep, time::{Duration, Instant}};

pub struct Ticker {
    period: Duration,
}

impl Ticker {
    pub fn new(hz: u64) -> Self {
        let period = Duration::from_secs_f64(1.0 / hz as f64);
        Self { period }
    }

    pub fn wait(&self, start:Instant) {
        let elapsed = start.elapsed();
        if elapsed < self.period {
            sleep(self.period - elapsed);
        }
    }

    pub fn period(&self) -> Duration {
        self.period
    }
}
