use colored::Colorize;

use crate::{commands::command_base::{Boxable, CommandBase}, logger::{info, success, warn}};

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
        let msg = format!("Queuing '{}'", boxed.name());
        info("[Scheduler]", msg);
        self.pending.push(boxed);
    }

    pub fn add_command<C: Boxable>(&mut self, command: C) {
        self.schedule(command);
    }

    pub fn run(&mut self) {
        // Start pending commands
        for mut cmd in self.pending.drain(..) {
            let msg = "Starting ".to_owned()+&cmd.name();
            success("[Timed]", msg);
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
                let msg = format!("Ending '{}'", c.name());
                success("[Scheduler]", msg);
                c.end(false);
            } else {
                i += 1;
            }
        }
    }

    pub fn cancel_all(&mut self) {
        for mut cmd in self.active.drain(..) {
            let msg = format!("Canceling '{}'", cmd.name());
            warn("[Scheduler]", msg);
            cmd.end(true);
        }
        self.pending.clear();
    }
}
