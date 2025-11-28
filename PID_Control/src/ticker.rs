use std::{thread::sleep, time::{Duration, Instant}};

pub struct Ticker {
    period: Duration,
}

impl Ticker {
    /// Creates a new period by calculating creating a ratio of 1/hz
    pub fn new(hz: u64) -> Self {
        let period = Duration::from_secs_f64(1.0 / hz as f64);
        Self { period }
    }

    /// Calls the sleep command if the elapsed is less than the period, which is calculated in the [new()](Ticker::new()) function
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
