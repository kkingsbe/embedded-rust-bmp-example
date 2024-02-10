pub mod bmp180;
use embedded_hal::blocking::delay::DelayMs;
use stm32f4xx_hal::{pac::TIM1, timer::Delay};

use crate::sensor::Sensor;

pub trait Barometer: Sensor {
    fn read_pressure(&mut self) -> i32;
    fn read_temperature(&mut self, delay: &mut Delay<TIM1, 1000>) -> i32;

    fn pressure(&self) -> i32;
    fn temperature(&self) -> i32;
}