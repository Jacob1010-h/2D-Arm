#[derive(Clone, Copy)]
pub enum RobotMode {
    AUTONOMOUS,
    TELEOP,
    None,
}

pub trait RobotBase {

    fn new() -> Self;
    
    /// Getter for the current mode that the robot is in
    fn get_mode(&mut self) -> RobotMode;
    /// Setter for the robot mode
    fn set_mode(&mut self, mode: RobotMode);

    /// Initilize the robot (setting any variables), defaults sets to [RobotMode::TELEOP]
    fn robot_init(&mut self);
    /// Initilize the robot (setting any variables), choose init [RobotMode]
    fn robot_init_mode(&mut self, mode: RobotMode);
    /// Periodically preform a task every loop, no matter the mode
    fn robot_periodic(&mut self);

    /// Initilize the robot (setting any variables) in [RobotMode::AUTONOMOUS]
    fn auto_init(&mut self);
    /// Periodically preform a task every loop, in [RobotMode::AUTONOMOUS]
    fn auto_periodic(&mut self);

    /// Initilize the robot (setting any variables) in [RobotMode::TELEOP]
    fn teleop_init(&mut self);
    /// Periodically preform a task every loop, in [RobotMode::TELEOP]
    fn teleop_periodic(&mut self);
}