use std::sync::Arc;
use crate::commands::command_base::CommandBase;

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
        println!("[Runnable] Initializing '{}'", self.name);
    }

    fn execute(&mut self) {
        if !self.has_run {
            println!("[Runnable] Running '{}'", self.name);
            (self.action)();
            self.has_run = true;
        }
    }

    fn is_finished(&mut self) -> bool {
        self.has_run
    }

    fn end(&mut self, interrupted: bool) {
        if interrupted {
            println!("[Runnable] '{}' interrupted.", self.name);
        } else {
            println!("[Runnable] '{}' completed.", self.name);
        }
    }

    fn name(&mut self) -> String {
        return self.name.to_string();
    }
    
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
