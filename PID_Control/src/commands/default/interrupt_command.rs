use crate::commands::default::command_base::CommandBase;

pub struct InterruptCommand {
    name: String,
    command: Box<dyn CommandBase>,
    condition: Box<dyn FnMut() -> bool>,
}

impl InterruptCommand {
    pub fn new(
        name: &str,
        command: Box<dyn CommandBase>,
        condition: impl FnMut() -> bool + 'static,
    ) -> Self {
        Self {
            name: name.to_string(),
            command,
            condition: Box::new(condition),
        }
    }
}

impl CommandBase for InterruptCommand {
    fn initialize(&mut self) {
        self.command.initialize();
    }

    fn execute(&mut self) {
        if (self.condition)() {
            println!("Interrupt condition triggered for {}", self.name);
            self.command.end(true);
            return;
        }

        self.command.execute();
    }

    fn is_finished(&mut self) -> bool {
        if (self.condition)() {
            return true;
        }
        return self.command.is_finished();
    }

    fn end(&mut self, interrupted: bool) {
        self.command.end(interrupted);
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn name(&mut self) -> String {
        return self.name.clone();
    }
}
