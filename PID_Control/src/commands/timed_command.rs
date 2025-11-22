use crate::commands::command_base::CommandBase;

#[derive(Debug)]
pub struct TimedCommand {
    name: String,
    duration: u64,
    is_running: bool
}

impl TimedCommand {
    pub fn new(name: &str, duration: u64) -> Self {
        TimedCommand { 
            name: name.to_string(), 
            duration, 
            is_running: false 
        }
    }
}

impl Clone for  TimedCommand {
    fn clone(&self) -> Self {
        TimedCommand { 
            name: self.name.clone(), 
            duration: self.duration,
            is_running: self.is_running 
        }
    }
}

impl CommandBase for TimedCommand {
    fn execute(&mut self) {
        if self.is_running {
            println!("Running {}...", self.name);
            std::thread::sleep(std::time::Duration::from_secs(self.duration));
            self.is_running = false;
        }

    }
    
    fn is_finished(&mut self) -> bool {
        return !self.is_running;
    }
    
    fn stop(&mut self) {
        self.is_running = false;
    }
    
    fn initialize(&mut self) {
        self.is_running = true;
    }
    
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    
    fn get_name(&mut self) -> String {
        return self.name.to_string();
    }
    
}