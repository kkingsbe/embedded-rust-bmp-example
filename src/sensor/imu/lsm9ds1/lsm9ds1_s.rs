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
    pub register_map: RegisterMap,
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

pub struct RegisterMap {
   pub magnetometer: MagnetometerRM,
   pub accelerometer: AccelerometerRM
}

pub struct MagnetometerRM {
    pub offset_x_reg_l_m: u8,
    pub offset_x_reg_h_m: u8,
    pub offset_y_reg_l_m: u8,
    pub offset_y_reg_h_m: u8,
    pub offset_z_reg_l_m: u8,
    pub offset_z_reg_h_m: u8,
    pub who_am_i: u8,
    pub ctrl_reg1_m: u8,
    pub ctrl_reg2_m: u8,
    pub ctrl_reg3_m: u8,
    pub ctrl_reg4_m: u8,
    pub ctrl_reg5_m: u8,
    pub status_reg_m: u8,
    pub out_x_l_m: u8,
    pub out_x_h_m: u8,
    pub out_y_l_m: u8,
    pub out_y_h_m: u8,
    pub out_z_l_m: u8,
    pub out_z_h_m: u8,
    pub int_cfg_m: u8,
    pub int_src_m: u8,
    pub int_ths_l_m: u8,
    pub int_ths_h_m: u8,
}

impl MagnetometerRM {
    pub fn new() -> Self {
        MagnetometerRM {
            offset_x_reg_l_m: 0x05,
            offset_x_reg_h_m: 0x06,
            offset_y_reg_l_m: 0x07,
            offset_y_reg_h_m: 0x08,
            offset_z_reg_l_m: 0x09,
            offset_z_reg_h_m: 0x0A,
            who_am_i: 0x0F,
            ctrl_reg1_m: 0x20,
            ctrl_reg2_m: 0x21,
            ctrl_reg3_m: 0x22,
            ctrl_reg4_m: 0x23,
            ctrl_reg5_m: 0x24,
            status_reg_m: 0x27,
            out_x_l_m: 0x28,
            out_x_h_m: 0x29,
            out_y_l_m: 0x2A,
            out_y_h_m: 0x2B,
            out_z_l_m: 0x2C,
            out_z_h_m: 0x2D,
            int_cfg_m: 0x30,
            int_src_m: 0x31,
            int_ths_l_m: 0x32,
            int_ths_h_m: 0x33
        }
    }
}

pub struct AccelerometerRM {
    pub ctrl_reg6_xl: u8,
}

impl AccelerometerRM {
    pub fn new() -> Self {
        AccelerometerRM {
            ctrl_reg6_xl: 0x20
        }
    }
}

impl RegisterMap {
    pub fn new() -> Self {
        RegisterMap {
            magnetometer: MagnetometerRM::new(),
            accelerometer: AccelerometerRM::new(),
        }
    }
}