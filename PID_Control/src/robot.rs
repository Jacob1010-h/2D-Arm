use crate::{commands::default::command_scheduler::CommandScheduler, logger, robot_base::{RobotBase, RobotMode}};


use crate::commands::motor_position_command;
pub struct Robot {
    pub scheduler: CommandScheduler,
    mode: RobotMode,
}

impl RobotBase for Robot {
    fn new() -> Self {
        Self {
            scheduler: CommandScheduler::new(),
            mode: RobotMode::None
        }
    }

    fn get_mode(&mut self) -> RobotMode {
        return self.mode;
    }

    fn set_mode(&mut self, mode: RobotMode) {
        self.mode = mode;
    }

    /// Gives the option to input a new [RobotMode]
    fn robot_init_mode(&mut self, mode: RobotMode) {
        self.set_mode(mode);
    }

    /// Assumes a [RobotMode] of [RobotMode::TELEOP]
    fn robot_init(&mut self) {
        self.set_mode(RobotMode::TELEOP);
    }

    fn robot_periodic(&mut self) {
        self.scheduler.run();
    }

    fn auto_init(&mut self) {
        logger::info("[Robot]","Automous Init");
        self.mode = RobotMode::AUTONOMOUS;
    }

    fn auto_periodic(&mut self) {

    }

    fn teleop_init(&mut self) {
        logger::info("[Robot]", "Teleop Init");
        self.mode = RobotMode::TELEOP;
    }

    fn teleop_periodic(&mut self) {

    }


}
