pub mod bmp180;
use crate::sensor::Sensor;

pub trait Barometer: Sensor {
    fn read_pressure(&mut self) -> i32;
    fn read_temperature(&mut self) -> i32;

    fn pressure(&self) -> i32;
    fn temperature(&self) -> i32;
}