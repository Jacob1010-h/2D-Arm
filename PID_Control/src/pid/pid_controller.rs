pub struct PidController {
    k_p: f64,
    k_i: f64,
    k_d: f64,
    error: f64,
    setpoint: f64
}

pub trait Calculation {
    fn step(&self, dt: f64, prev_err: f64, integ_total: f64) -> PidOut;
}

pub struct PidOut {
    pub output: f64,
    pub error: f64,
    pub integ_total: f64
}

impl PidController {
    pub fn new() -> Self {
        Self { k_p: 0.0, k_i: 0.0, k_d: 0.0, error: 0.0, setpoint: 0.0 }
    }
}

impl Calculation for PidController {
    /// The step() function will preform the calculation for PID given all of the required inputs.
    /// 
    /// @params: dt: f64, prevErr: f64, integTotal: f64 
    /// @returns: PID_OUT{output, error, integTotal}
    fn step(&self, dt: f64, prev_err: f64, mut integ_total: f64) -> PidOut {
        // Calculate the Proportional
        let p = self.k_p * self.error;
        // Calculate the Integral
        let i = self.k_i * self.error * dt;
        integ_total += i;
        // Calculate the Derivative
        let d = self.k_d * (self.error * prev_err) / dt;

        // Calculate the output and then return the error and total integral.
        let output = p + integ_total + d;
        return PidOut {
            output: output,
            error: self.error,
            integ_total
        };
    }


    
}