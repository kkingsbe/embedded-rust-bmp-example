use byteorder::{ByteOrder, LittleEndian};
use cortex_m::asm::nop;
use stm32f4xx_hal::{i2c::{I2c, Instance as I2cInstance}, pac::TIM1, timer::DelayMs};
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs; //Bring the DelayMs trait into scope
use crate::sensor::{SensorError, SensorState};
use super::lsm9ds1_s::{CalibrationInfo, ImuData, MagnetometerRM, XlOdr, LSM9DS1};

impl<'a, T  > LSM9DS1<'a, T> where T: I2cInstance {
    pub fn new(i2c: &'a mut I2c<T>) -> Self {
        LSM9DS1 {
            m_addr: 0x1E,
            addr: 0x6B,
            i2c,
            state: SensorState::INITIAL,
            data: ImuData::new(),
            calibration_info: CalibrationInfo::new()
        }
    }

    //Sanity check to ensure the sensor is powered on and accessible
    pub fn sanity_check(&mut self) -> bool {
        let mut rx_buffer: [u8; 2] = [0; 2];

        //Read the id from the sensor to confirm it is powered on and accessible
        let res = self.i2c.write_read(self.m_addr, &[MagnetometerRM::WhoAmI as u8], &mut rx_buffer);

        //Check if an i2c error occured. If so, it would be due to the sensor not being found (incorrect i2c address or not powered on)
        match res {
            Ok(_) => {},
            Err(_) => {
                self.state = SensorState::ERROR(SensorError::NotFound);
                return false;
            }
        }

        return if rx_buffer[0] == 0x3D {
            // Sensor detected
            true
        } else {
            // Sensor not detected
            self.state = SensorState::ERROR(SensorError::NotFound);
            false
        }
    }

    pub fn twos_complement(&self, high: u8, low: u8) -> i16 {
        // Reads the two bytes as a little-endian 16-bit unsigned integer
        let combined = LittleEndian::read_u16(&[low, high]);
        
        // Directly cast the unsigned integer to a signed integer
        // This inherently interprets the value as two's complement if the sign bit is set
        combined as i16
    }
}