use std::time::Instant;

use crate::{
    commands::default::command_base::CommandBase,
    logger,
    motor_sim::{Motor, MotorLogger, MotorSim},
    pid::pid_controller::{PidController, PidOut},
};

pub struct MotorPositionCommand {
    name: String,

    motor: MotorSim,

    pid: PidController,
    target_pos_rad: f64,

    prev_error: f64,
    integ_total: f64,

    last_time: Option<Instant>,

    done: bool,
}

impl MotorPositionCommand {
    pub fn new(name: &str, motor: MotorSim, pid: PidController, target_pos_rad: f64) -> Self {
        Self {
            name: name.to_string(),
            motor,
            pid,
            target_pos_rad,
            prev_error: 0.0,
            integ_total: 0.0,
            last_time: None,
            done: false,
        }
    }

    pub fn motor(&self) -> &MotorSim {
        &self.motor
    }

    pub fn motor_mut(&mut self) -> &mut MotorSim {
        &mut self.motor
    }
}

impl CommandBase for MotorPositionCommand {
    fn initialize(&mut self) {
        logger::info("[MotorCmd]", format!("Initializing '{}'", self.name));
        self.motor.reset();
        self.prev_error = 0.0;
        self.integ_total = 0.0;
        self.last_time = Some(Instant::now());
        self.done = false;
    }

    fn execute(&mut self) {
        
    }

    fn is_finished(&mut self) -> bool {
        self.done
    }

    fn end(&mut self, interrupted: bool) {
        if interrupted {
            logger::warn("[MotorCmd]", format!("'{}' interrupted", self.name));
        } else {
            logger::success("[MotorCmd]", format!("'{}' completed", self.name));
        }

        self.motor.set_voltage(0.0);

        if let Some(ref logger) = self.motor.logger {
        logger.write_csv("motor_log.csv");
        logger::info("[MotorCmd]", "Wrote motor_log.csv");

        // Plot to PNG
        logger.plot("motor_plot.png");
        logger::info("[MotorCmd]", "Wrote motor_plot.png");
        }
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn name(&mut self) -> String {
        return self.name.clone();
    }
}
