use std::time::{Duration, Instant};

use colored::Colorize;

use crate::{commands::default::command_base::CommandBase, logger::{info, success, warn}};

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
        let msg = format!("Initializing '{}' for {:?}", self.name, self.duration);
        success("[Timed]", msg);
        self.start = Some(Instant::now())
    }

    fn execute(&mut self) {
        if let Some(start) = self.start {
            let e = start.elapsed();
            let msg = format!("'{}' {:.2?} / {:.2?}", self.name, e, self.duration);
            info("[Timed]", msg);
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
            let msg = format!("'{}' interrupted.", self.name);
            warn("[Timed]", msg);
        } else {
            let msg = format!("'{}' finished.", self.name);
            success("[Timed]", msg);
        }
    }

    fn name(&mut self) -> String {
        return self.name.to_string();
    }
}
