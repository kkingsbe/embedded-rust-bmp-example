use embedded_hal::blocking::i2c::WriteRead;
use stm32f4xx_hal::{hal, pac::TIM1};
use stm32f4xx_hal::i2c::{I2c, Instance as I2cInstance};
use stm32f4xx_hal::timer::{DelayMs, Timer, TimerExt, Instance as TimerInstance};
use stm32f4xx_hal::rcc::Clocks;
use embedded_hal::prelude::*;

use crate::sensor::{SensorState, SensorError};

pub struct ImuAccelerationData {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

#[derive(Debug, Copy, Clone)]
pub enum XlOdr {
    PowerDown = 0,
    Hz10 = 1,
    Hz50 = 2,
    Hz119 = 3,
    Hz238 = 4,
    Hz476 = 5,
    Hz952 = 6,
    CONTINUOUS = 7
}

#[derive(Debug, Copy, Clone)]
pub enum GyroOdr {
    PowerDown = 0,
    Hz15 = 1,
    Hz60 = 2,
    Hz119 = 3,
    Hz238 = 4,
    Hz952 = 5,
    CONTINUOUS = 6
}

pub struct ImuData {
    pub acceleration: ImuAccelerationData
}

impl ImuData {
    pub fn new() -> Self {
        ImuData {
            acceleration: ImuAccelerationData {
                x: 0,
                y: 0,
                z: 0
            }
        }
    }
}

pub struct LSM9DS1<'a, T> where T: I2cInstance {
    pub m_addr: u8, //Magnetometer address
    pub addr: u8, //Accelerometer and Gyroscope address
    pub i2c: &'a mut I2c<T>, //Allows for the BMP180 struct to not take ownership of the I2C instance, which means multiple devices can be on the same bus :)
    pub state: SensorState,
    pub data: ImuData,
    pub calibration_info: CalibrationInfo
}

pub struct CalibrationInfo {
    pub magnetometer: MagnetometerCalibration
}

impl CalibrationInfo {
    pub fn new() -> Self {
        CalibrationInfo {
            magnetometer: MagnetometerCalibration {
                x_offset: 0,
                y_offset: 0,
                z_offset: 0
            }
        }
    }
}

pub struct MagnetometerCalibration {
    pub x_offset: i32,
    pub y_offset: i32,
    pub z_offset: i32
}

pub enum MagnetometerRM {
    OffsetXRegLM = 0x05,
    OffsetXRegHM = 0x06,
    OffsetYRegLM = 0x07,
    OffsetYRegHM = 0x08,
    OffsetZRegLM = 0x09,
    OffsetZRegHM = 0x0A,
    WhoAmI = 0x0F,
    CtrlReg1M = 0x20,
    CtrlReg2M = 0x21,
    CtrlReg3M = 0x22,
    CtrlReg4M = 0x23,
    CtrlReg5M = 0x24,
    StatusRegM = 0x27,
    OutXLM = 0x28,
    OutXHM = 0x29,
    OutYLM = 0x2A,
    OutYHM = 0x2B,
    OutZLM = 0x2C,
    OutZHM = 0x2D
}

pub enum AccelerometerRM {
    CtrlReg6Xl = 0x20,
    OutXXlL = 0x28,
    OutXXlH = 0x29,
    OutYXlL = 0x2A,
    OutYXlH = 0x2B,
    OutZXlL = 0x2C,
    OutZXlH = 0x2D
}

pub enum GyroRM {
    CtrlReg1G = 0x10,
    OutXGL = 0x18,
    OutXGH = 0x19,
    OutYGL = 0x1A,
    OutYGH = 0x1B,
    OutZGL = 0x1C,
    OutZGH = 0x1D
}