use crate::commands::command_base::{Boxable, CommandBase};

pub struct CommandScheduler {
    commands: Vec<Box<dyn CommandBase>>,
    current_command: Option<Box<dyn CommandBase>>,
}

impl CommandScheduler {
    pub fn new() -> Self {
        CommandScheduler {
            commands: Vec::new(),
            current_command: None,
        }
    }

    pub fn add_command<C: Boxable>(&mut self, command: C) {
        self.commands.push(command.boxed()); // Use the boxed method
        for command in &mut self.commands {
            println!("{}", command.get_name());
        }
        println!("----------------")
    }

    pub fn run(&mut self) {
        // Check if the current command is finished
        if let Some(current) = &mut self.current_command {
            if current.is_finished() {
                println!("Current Command finished. Removing it.");
                self.current_command = None; // Set current_command to None
            } else {
                current.execute();
                return;
            }
        }

        // If there's no current command, get the next one if available
        if self.current_command.is_none() && !self.commands.is_empty() {
            println!("Starting a new command.");
            let mut next = self.commands.remove(0);
            next.initialize();
            self.current_command = Some(next);
        }
    }
}
