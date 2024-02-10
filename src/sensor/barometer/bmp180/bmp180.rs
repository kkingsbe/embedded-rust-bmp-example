use stm32f4xx_hal::{i2c::{I2c, Instance as I2cInstance}, pac::TIM1, timer::DelayMs};
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs; //Bring the DelayMs trait into scope
use crate::sensor::{SensorError, SensorState};

use super::{BmpData, Coeffs, RegisterMap, BMP180};

impl<'a, T  > BMP180<'a, T> where T: I2cInstance {
    pub fn new(i2c: &'a mut I2c<T>/*, delay: &'a mut DelayMs<TIM1>*/) -> Self {
        BMP180 {
            calib_coeffs: Coeffs {
                ac5: 0,
                ac6: 0,
                mc: 0,
                md: 0
            },
            addr: 0x77,
            register_map: RegisterMap {
                reg_id_addr: 0xD0,
                ac5_msb_addr: 0xB2,
                ac6_msb_addr: 0xB4,
                mc_msb_addr: 0xBC,
                md_msb_addr: 0xBE,
                ctrl_meas_addr: 0xF4,
                meas_out_lsb_addr: 0xF7,
                meas_out_msb_addr: 0xF6
            },
            i2c,
            state: SensorState::INITIAL,
            //delay,
            data: BmpData::new()
        }
    }

    //Sanity check to ensure the sensor is powered on and accessible
    pub fn sanity_check(&mut self) -> bool {
        let mut rx_buffer: [u8; 2] = [0; 2];
        let mut rx_word: i16;

        //Read the id from the sensor to confirm it is powered on and accessible
        self.i2c.write_read(self.addr, &[self.register_map.reg_id_addr], &mut rx_buffer).unwrap();
        return if rx_buffer[0] == 0x55 {
            // BMP180 detected
            true
        } else {
            // BMP180 not detected
            self.state = SensorState::ERROR(SensorError::NotFound);
            false
        }
    }

    pub fn read_calibration_coefficient(&mut self, addr: u8) -> Result<i16, ()> {
        let mut rx_buffer: [u8; 2] = [0; 2];
        let mut rx_word: i16 = 0;

        self.i2c.write_read(self.addr, &[addr], &mut rx_buffer).unwrap();
        rx_word = ((rx_buffer[0] as i16) << 8) | rx_buffer[1] as i16;

        return if rx_word == 0 {
            self.state = SensorState::ERROR(SensorError::FailedToCalibrate);
            Err(())
        } else {
            Ok(rx_word)
        }
    }
}