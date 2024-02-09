pub mod barometer;
pub mod sensor;
pub mod bmp180;
pub mod bmp180_s;

use embedded_hal::blocking::i2c::WriteRead;
use stm32f4xx_hal::{hal, pac::TIM1};
use stm32f4xx_hal::i2c::{I2c, Instance as I2cInstance};
use stm32f4xx_hal::timer::{DelayMs, Timer, TimerExt, Instance as TimerInstance};
use stm32f4xx_hal::rcc::Clocks;
use embedded_hal::prelude::*;

use super::Barometer;
use super::super::{Sensor, SensorError, SensorState};

pub struct BmpData {
    pub temperature: i32,
    pub pressure: i32
}

impl BmpData {
    pub fn new() -> Self {
        BmpData {
            temperature: 0,
            pressure: 0
        }
    }
}

pub struct BMP180<'a, T> where T: I2cInstance {
    pub calib_coeffs: Coeffs,
    pub addr: u8,
    pub register_map: RegisterMap,
    pub i2c: &'a mut I2c<T>, //Allows for the BMP180 struct to not take ownership of the I2C instance, which means multiple devices can be on the same bus :)
    pub state: SensorState,
    pub delay: &'a mut DelayMs<TIM1>,
    data: BmpData
}

pub struct RegisterMap {
    pub reg_id_addr: u8,
    pub ac5_msb_addr: u8,
    pub ac6_msb_addr: u8,
    pub mc_msb_addr: u8,
    pub md_msb_addr: u8,
    pub ctrl_meas_addr: u8,
    pub meas_out_lsb_addr: u8,
    pub meas_out_msb_addr: u8
}

pub struct Coeffs {
    pub ac5: i16,
    pub ac6: i16,
    pub mc: i16,
    pub md: i16
}