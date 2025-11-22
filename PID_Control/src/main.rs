

mod pid;
mod commands;

use crate::pid::pid_controller::Calculation;
use crate::commands::command_scheduler::{CommandScheduler};
use crate::commands::runnable_command::{RunnableCommand};
use crate::commands::timed_command::{TimedCommand};

fn main() {
    let mut scheduler = CommandScheduler::new();

    let command = RunnableCommand::new("Test", || {
        println!("This is all a test");
    });

    scheduler.add_command(command);

    // Create and clone the TimedCommand; ensure you initialize cloned ones
    let timed = TimedCommand::new("Timed Command", 2);
    let timed_clone = timed.clone(); // Clone it for later use

    scheduler.add_command(timed); // This command will run first
    scheduler.add_command(timed_clone); // Add the clone as a separate command

    loop {
        scheduler.run();
    }
}
