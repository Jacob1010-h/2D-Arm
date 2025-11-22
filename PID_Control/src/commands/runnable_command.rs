use std::sync::Arc;

use crate::commands::command_base::CommandBase;

pub struct RunnableCommand {
    name: String,
    action: Arc<dyn Fn() + Send + Sync + 'static>,
    is_running: bool
}

impl RunnableCommand {
    pub fn new(name: &str, action: impl Fn() + Send + Sync + 'static) -> Self {
        RunnableCommand {
            name: name.to_string(),
            action: Arc::new(action),
            is_running: false,
        }
    }
}

impl Clone for  RunnableCommand {
    fn clone(&self) -> Self {
        RunnableCommand { 
            name: self.name.clone(), 
            action: self.action.clone(), 
            is_running: self.is_running 
        }
    }
}

impl CommandBase for RunnableCommand {
    fn execute(&mut self) {
        if self.is_running {
            println!("Starting the Command: {}", self.name);
            self.is_running = true;

            (self.action)();
            self.is_running = false;

            println!("Command Finished: {}", self.name);
        }
    }
    
    fn is_finished(&mut self) -> bool{
        return !self.is_running;
    }
    
    fn stop(&mut self) {
        self.is_running = false;
    }
    
    fn initialize(&mut self) {
        self.is_running = true
    }
    
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    
    fn get_name(&mut self) -> String{
        return self.name.to_string();
    }

}