use crate::{constants, motor::motor_io::{MotorFn, MotorIO, MotorLogSample}};



pub struct MotorSim {
    motor_io: MotorIO,
    r: f64, // resistance constant
    kt: f64, // torque constant
    kv: f64, // back EMF constant

    j: f64, // inertia
    b: f64, // viscous friction
    load_torque: f64,
}


impl MotorSim {
    pub fn new() -> Self {
        Self { 
            motor_io: MotorIO::new(), 
            r: 0.5, 
            kt: 0.02, 
            kv: 0.02, 
            j: 0.0005, 
            b: 0.0001,

            load_torque: 0.0,
        }
    }
}

// General motor implimentation 
impl MotorFn for MotorSim {
    fn set_voltage(&mut self, volts:f64) {
        self.motor_io.input_voltage = volts.clamp(-constants::MAX_VOLTAGE, constants::MAX_VOLTAGE);
    }

    fn reset(&mut self) {
        self.motor_io.position = 0.0;
        self.motor_io.velocity = 0.0;
        self.motor_io.input_voltage = 0.0;
        self.load_torque = 0.0;
    }

    fn get_position_rad(&self) -> f64 {
        self.motor_io.position
    }

    fn get_velocity_rad_s(&self) -> f64 {
        self.motor_io.velocity
    }
}

// Simulated specific implimaintation
impl MotorSim {

    fn set_load_torque(&mut self, load: f64) {
        self.load_torque = load;
    }

    fn step(&mut self, dt: f64) {
        if dt <= 0.0 {
            return;
        }

        // Current from electrical model
        let i = (self.motor_io.input_voltage - self.kv * self.get_velocity_rad_s()) / self.r;

        // Torque from current
        let motor_torque = self.kt * i;

        // Net torque
        let net = motor_torque - self.b * self.get_velocity_rad_s() - self.load_torque;

        // Angular acceleration
        let acc = net / self.j;

        // Integrate
        self.motor_io.velocity += acc * dt;
        self.motor_io.position += self.get_velocity_rad_s() * dt;

        // logger
        if let Some(logger) = &mut self.motor_io.logger {
            let t = logger.start_time.elapsed().as_secs_f64();
            logger.log(MotorLogSample {
                time_s: t,
                position: self.motor_io.position,
                velocity: self.motor_io.velocity,
                voltage: self.motor_io.input_voltage,
                load_torque: self.load_torque,
            });
        }
    }

}

