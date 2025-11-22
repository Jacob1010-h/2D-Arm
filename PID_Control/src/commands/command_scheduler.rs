use crate::commands::command_base::{Boxable, CommandBase};

pub struct CommandScheduler {
    active: Vec<Box<dyn CommandBase>>,
    pending: Vec<Box<dyn CommandBase>>,
}

impl CommandScheduler {
    pub fn new() -> Self {
        CommandScheduler {
            active: Vec::new(),
            pending: Vec::new(),
        }
    }

    pub fn schedule<C: Boxable>(&mut self, command: C) {
        let mut boxed = command.boxed();
        println!("[Scheduler] Queuing '{}'", boxed.name());
        self.pending.push(boxed);
    }

    pub fn add_command<C: Boxable>(&mut self, command: C) {
        self.schedule(command);
    }

    pub fn run(&mut self) {
        // Start pending commands
        for mut cmd in self.pending.drain(..) {
            println!("[Scheduler] Starting '{}'", cmd.name());
            cmd.initialize();
            self.active.push(cmd);
        }

        // Execute all active commands
        for cmd in self.active.iter_mut() {
            cmd.execute();
        }

        // Clean up finished commands
        let mut i = 0;
        while i < self.active.len() {
            if self.active[i].is_finished() {
                let mut c = self.active.remove(i);
                println!("[Scheduler] Ending '{}'", c.name());
                c.end(false);
            } else {
                i += 1;
            }
        }
    }

    pub fn cancel_all(&mut self) {
        for mut cmd in self.active.drain(..) {
            println!("[Scheduler] Canceling '{}'", cmd.name());
            cmd.end(true);
        }
        self.pending.clear();
    }
}
