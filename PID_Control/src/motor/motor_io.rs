// pub trait MotorIO {
//     /// Set the motor's applied voltage
//     fn set_voltage(&mut self, volts: f64);

//     /// Set an external load torque
//     fn set_load_torque(&mut self, load: f64);

//     /// Update simulation by dt seconds
//     fn step(&mut self, dt: f64);

//     /// Reset the motor
//     fn reset(&mut self);

//     /// Getters
//     fn position_rad(&self) -> f64;
//     fn velocity_rad_s(&self) -> f64;
// }

use plotters::{chart::ChartBuilder, prelude::{BitMapBackend, IntoDrawingArea, PathElement}, series::LineSeries, style::{BLACK, BLUE, GREEN, RED, WHITE}};

use crate::{constants, motor::motor_io};

#[derive(Debug)]
pub struct MotorLogSample {
    pub time_s: f64,
    pub position: f64,
    pub velocity: f64,
    pub voltage: f64,
    pub load_torque: f64,
}

pub struct MotorLogger {
    pub(crate) start_time: std::time::Instant,
    samples: Vec<MotorLogSample>,
}

impl MotorLogger {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            samples: Vec::new(),
        }
    }

    pub fn log(&mut self, s: MotorLogSample) {
        self.samples.push(s);
    }

    pub fn plot(&self, path: &str) {
        let root = BitMapBackend::new(path, (1280, 720)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        if self.samples.is_empty() {
            return;
        }

        let min_t = self.samples.first().unwrap().time_s;
        let max_t = self.samples.last().unwrap().time_s;

        let (min_pos, max_pos) = self.samples.iter().fold(
            (f64::MAX, f64::MIN),
            |(mn, mx), s| (mn.min(s.position), mx.max(s.position)),
        );

        let mut chart = ChartBuilder::on(&root)
            .caption("Motor Telemetry", ("sans-serif", 30))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(min_t..max_t, min_pos..max_pos)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                self.samples.iter().map(|s| (s.time_s, s.position)),
                &RED,
            ))
            .unwrap()
            .label("Position (rad)")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .draw_series(LineSeries::new(
                self.samples.iter().map(|s| (s.time_s, s.velocity)),
                &BLUE,
            ))
            .unwrap()
            .label("Velocity (rad/s)")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        chart
            .draw_series(LineSeries::new(
                self.samples.iter().map(|s| (s.time_s, s.voltage)),
                &GREEN,
            ))
            .unwrap()
            .label("Voltage (V)")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

        chart
            .configure_series_labels()
            .background_style(&WHITE)
            .border_style(&BLACK)
            .draw()
            .unwrap();
    }

    pub fn write_csv(&self, path: &str) {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(path).expect("Unable to create motor log file");

        writeln!(file, "time_s,position,velocity,voltage,load_torque").unwrap();

        for s in &self.samples {
            writeln!(
                file,
                "{:.6},{:.6},{:.6},{:.6},{:.6}",
                s.time_s, s.position, s.velocity, s.voltage, s.load_torque
            )
            .unwrap();
        }
    }
}

pub struct MotorIO {
    pub(crate) position: f64,
    pub(crate) velocity: f64,

    pub(crate) input_voltage: f64,

    pub logger: Option<MotorLogger>
}

impl MotorIO {
    pub fn new() -> Self {
        Self { 
            position: 0.0, 
            velocity: 0.0, 
            input_voltage: 0.0, 
            logger: Some(MotorLogger::new())
        }
    }
}

pub trait MotorFn {
    fn set_voltage(&mut self, volts:f64);

    fn reset(&mut self);

    fn get_position_rad(&self) -> f64;

    fn get_velocity_rad_s(&self) -> f64;
}
