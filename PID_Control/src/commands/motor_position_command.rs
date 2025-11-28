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
        let now = Instant::now();
        let dt = if let Some(last) = self.last_time {
            (now - last).as_secs_f64()
        } else {
            0.0
        };

        self.last_time = Some(now);

        let current_pos = self.motor.position_rad();

        let PidOut {
            output,
            error,
            integ_total,
        } = self.pid.step(
            dt,
            self.target_pos_rad,
            current_pos,
            self.prev_error,
            self.integ_total,
        );

        self.prev_error = error;
        self.integ_total = integ_total;

        let v_clamped = output.clamp(-self.motor.max_voltage, self.motor.max_voltage);
        self.motor.set_voltage(v_clamped);
        self.motor.step(dt);

        logger::debug(
            "[MotorCmd]",
            format!(
                "dt={:.3} pos={:.3} target={:.3} err={:.3} vel={:.3} V={:.3}",
                dt,
                current_pos,
                self.target_pos_rad,
                error,
                self.motor.velocity_rad_s(),
                output,
            ),
        );

        if error.abs() < 0.01 && self.motor.velocity_rad_s().abs() < 0.01 {
            self.done = true;
        }
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
