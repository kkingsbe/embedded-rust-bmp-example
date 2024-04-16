pub mod lsm9ds1;
use embedded_hal::blocking::delay::DelayMs;
use stm32f4xx_hal::{pac::TIM1, timer::Delay};

use crate::sensor::Sensor;

pub trait Accelerometer: Sensor {
    fn read_acceleration(&mut self) -> (f32, f32, f32);
}

pub trait Gyroscope: Sensor {
    fn read_gyro(&mut self) -> (f32, f32, f32);
}

pub trait Magnetometer: Sensor {
    fn read_magnetometer(&mut self) -> (i32, i32, i32);
}