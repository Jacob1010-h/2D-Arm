use std::time::{Duration, Instant};

use crate::commands::command_base::CommandBase;

#[derive(Debug)]
pub struct TimedCommand {
    name: String,
    duration: Duration,
    start: Option<Instant>,
    is_running: bool,
}

impl TimedCommand {
    pub fn new(name: &str, duration: f32) -> Self {
        TimedCommand {
            name: name.to_string(),
            duration: Duration::from_secs_f32(duration),
            start: None,
            is_running: false,
        }
    }
}

impl CommandBase for TimedCommand {
    fn initialize(&mut self) {
        println!("[Timed] Initializing '{}' for {:?}", self.name, self.duration);
        self.start = Some(Instant::now())
    }

    fn execute(&mut self) {
        if let Some(start) = self.start {
            let e = start.elapsed();
            println!("[Timed] '{}' {:.2?} / {:.2?}", self.name, e, self.duration);
        }
    }

    fn is_finished(&mut self) -> bool {
        if let Some(start) = self.start {
            start.elapsed() >= self.duration
        } else {
            false
        }
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn end(&mut self, interrupted: bool) {
        if interrupted {
            println!("[Timed] '{}' interrupted.", self.name);
        } else {
            println!("[Timed] '{}' finished.", self.name);
        }
    }

    fn name(&mut self) -> String {
        return self.name.to_string();
    }
}
