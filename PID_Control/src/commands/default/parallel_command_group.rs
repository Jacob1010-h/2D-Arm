use crate::commands::default::command_base::CommandBase;

pub struct ParallelCommandGroup {
    name: String,
    commands: Vec<Box<dyn CommandBase>>,
}

impl ParallelCommandGroup {
    pub fn new(name: &str, commands: Vec<Box<dyn CommandBase>>) -> Self {
        Self {
            name: name.to_string(),
            commands,
        }
    }
}

impl CommandBase for ParallelCommandGroup {
    fn initialize(&mut self) {
        println!("[Parallel] Initializing '{}'", self.name);
        for cmd in self.commands.iter_mut() {
            cmd.initialize();
        }
    }

    fn execute(&mut self) {
        for cmd in self.commands.iter_mut() {
            cmd.execute();
        }
    }

    fn is_finished(&mut self) -> bool {
        self.commands.iter_mut().all(|cmd| cmd.is_finished())
    }

    fn end(&mut self, interrupted: bool) {
        if interrupted {
            println!("[Parallel] '{}' interrupted.", self.name);
            for cmd in self.commands.iter_mut() {
                cmd.end(true);
            }
        } else {
            println!("[Parallel] '{}' completed.", self.name);
            for cmd in self.commands.iter_mut() {
                cmd.end(false);
            }
        }
    }

    fn name(&mut self) -> String {
        return self.name.to_string()
    }
    
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
