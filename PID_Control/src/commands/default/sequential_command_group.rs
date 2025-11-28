use crate::commands::default::command_base::CommandBase;

pub struct SequentialCommandGroup {
    name: String,
    commands: Vec<Box<dyn CommandBase>>,
    index: usize,
}

impl SequentialCommandGroup {
    pub fn new(name: &str, commands: Vec<Box<dyn CommandBase>>) -> Self {
        Self {
            name: name.to_string(),
            commands,
            index: 0,
        }
    }
}

impl CommandBase for SequentialCommandGroup {
    fn initialize(&mut self) {
        println!("[Sequential] Initializing '{}'", self.name);
        self.index = 0;

        if let Some(cmd) = self.commands.get_mut(0) {
            cmd.initialize();
        }
    }

    fn execute(&mut self) {
        if self.index >= self.commands.len() {
            return;
        }

        let current = &mut self.commands[self.index];
        current.execute();

        if current.is_finished() {
            current.end(false);
            self.index += 1;

            if self.index < self.commands.len() {
                println!(
                    "[Sequential] '{}' advancing to step {}",
                    self.name, self.index
                );
                self.commands[self.index].initialize();
            }
        }
    }

    fn is_finished(&mut self) -> bool {
        self.index >= self.commands.len()
    }

    fn end(&mut self, interrupted: bool) {
        if interrupted {
            println!("[Sequential] '{}' interrupted.", self.name);
        } else {
            println!("[Sequential] '{}' completed.", self.name);
        }
    }

    fn name(&mut self) -> String {
        return self.name.to_string();
    }
    
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
