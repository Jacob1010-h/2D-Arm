mod constants;
mod logger;
mod motor;
mod pid;
mod robot;
mod robot_base;
mod ticker;

use crate::{
    constants::LOOP_RATE_HZ,
    motor::motor_sim::MotorSim,
    pid::pid_controller::PidController,
    robot::Robot,
    robot_base::{RobotBase, RobotMode},
    ticker::Ticker,
};
use std::time::Instant;

fn main() {
    let mut robot: Robot = Robot::new();

    // Init the robot
    robot.robot_init();
    match robot.get_mode() {
        RobotMode::AUTONOMOUS => robot.auto_init(),
        RobotMode::TELEOP => robot.teleop_init(),
        RobotMode::None => logger::error("[Main]", "Robot Mode not set"),
    }

    let motor: MotorSim = MotorSim::new();

    let pid: PidController = PidController::new(10.0, 0.05, 0.5);


    let target_pos_deg: f32 = 45.0;
    let target_pos_rad: f64 = target_pos_deg.to_radians().into();

    // Set the Motor Position 
    
    // Print that we are entering the loop
    let _msg = format!("Entering robot loop at {} Hz\n", LOOP_RATE_HZ);
    println!();
    logger::debug("[Main]", _msg);

    loop {
        // start and record the loop start time
        let start = Instant::now();

        robot.robot_periodic();

        match robot.get_mode() {
            RobotMode::AUTONOMOUS => robot.auto_periodic(),
            RobotMode::TELEOP => robot.teleop_periodic(),
            RobotMode::None => logger::error("[Main]", "Robot Mode not set"),
        }

        // wait the remaining time
        Ticker::new(LOOP_RATE_HZ).wait(start);
    }
}
