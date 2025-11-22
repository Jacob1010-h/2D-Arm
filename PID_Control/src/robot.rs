use crate::{commands::command_scheduler::CommandScheduler, logger::info};

#[derive(Clone, Copy)]
pub enum RobotMode {
    AUTONOMOUS,
    TELEOP,
    None,
}

pub struct Robot {
    pub scheduler: CommandScheduler,
    mode: RobotMode,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            scheduler: CommandScheduler::new(),
            mode: RobotMode::None
        }
    }

    pub fn get_mode(&mut self) -> RobotMode {
        return self.mode;
    }

    pub fn set_mode(&mut self, mode: RobotMode) {
        self.mode = mode;
    }

    pub fn init(&mut self) {
        self.set_mode(RobotMode::TELEOP);
    }

    pub fn periodic(&mut self) {
        self.scheduler.run();
    }

    pub fn auto_init(&mut self) {
        info("[Robot]","Automous Init");
        self.mode = RobotMode::AUTONOMOUS;
    }

    pub fn auto_periodic(&mut self) {

    }

    pub fn teleop_init(&mut self) {
        info("[Robot]", "Teleop Init");
        self.mode = RobotMode::TELEOP;
    }

    pub fn teleop_periodic(&mut self) {

    }


}
