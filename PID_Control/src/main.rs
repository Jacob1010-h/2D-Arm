mod commands;
mod constants;
mod ticker;
mod robot;
mod logger;


use std::time::Instant;
use crate::{commands::{instant_command::InstantCommand, timed_command::TimedCommand}, constants::LOOP_RATE_HZ, logger::warn, robot::{Robot, RobotMode}, ticker::Ticker};


fn main() {
    let ticker = Ticker::new(LOOP_RATE_HZ);
    let mut robot = Robot::new();

    robot.init();

    robot.scheduler.schedule(
        InstantCommand::new("StartupPrint", || {
            println!(">>> Robot startup scheck!");
        })
    );

    robot.scheduler.schedule(
        TimedCommand::new("WarmUpTimer", 1.0)
    );

    let _msg = format!("Entering robot loop at {} Hz\n", LOOP_RATE_HZ);
    println!();
    warn("[Main]", _msg);


    loop {
        let start = Instant::now();

        robot.periodic();

        match robot.get_mode() {
            RobotMode::AUTONOMOUS => robot.auto_periodic(),
            RobotMode::TELEOP => robot.teleop_periodic(),
            RobotMode::None => println!("[Main] Robot Mode not set"),
        }

        ticker.wait(start);
    }

    
}
