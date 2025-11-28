use std::sync::Arc;

use crate::{commands::default::command_base::CommandBase, logger::{info, success, warn}};

pub struct InstantCommand {
    name: String,
    action: Arc<dyn Fn() + Send + Sync + 'static>,
    has_run: bool,
}

impl InstantCommand {
    pub fn new(name: &str, action: impl Fn() + Send + Sync + 'static) -> Self {
        Self {
            name: name.to_string(),
            action: Arc::new(action),
            has_run: false,
        }
    }
}

impl CommandBase for InstantCommand {
    fn initialize(&mut self) {
        self.has_run = false;
        let msg = format!("Initializing '{}'", self.name);
        success("[Instant]", msg);
    }

    fn execute(&mut self) {
        if !self.has_run {
            let msg = format!("Running '{}'", self.name);
            info("[Instant]", msg);
            (self.action)();
            self.has_run = true;
        }
    }

    fn is_finished(&mut self) -> bool {
        self.has_run
    }

    fn end(&mut self, interrupted: bool) {
        if interrupted {

            let msg = format!("'{}' interrupted.", self.name);
            warn("[Instant]", msg);
        } else {

            let msg = format!("'{}' completed.", self.name);
            success("[Instant]", msg);
        }
    }

    fn name(&mut self) -> String {
        return self.name.to_string();
    }
    
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
