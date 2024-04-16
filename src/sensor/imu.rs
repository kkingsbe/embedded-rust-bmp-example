pub mod lsm9ds1;
use embedded_hal::blocking::delay::DelayMs;
use stm32f4xx_hal::{pac::TIM1, timer::Delay};

use crate::sensor::Sensor;

pub trait IMU: Sensor {
    fn read_acceleration(&mut self) -> (f32, f32, f32);
    fn read_gyro(&mut self) -> (i32, i32, i32);
    fn read_magnetometer(&mut self) -> (i32, i32, i32);
}