use byteorder::{ByteOrder, LittleEndian};
use cortex_m::asm::nop;
use stm32f4xx_hal::{i2c::{I2c, Instance as I2cInstance}, pac::TIM1, timer::DelayMs};
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs; //Bring the DelayMs trait into scope
use crate::sensor::{SensorError, SensorState};
use super::lsm9ds1_s::{CalibrationInfo, ImuData, RegisterMap, LSM9DS1};

impl<'a, T  > LSM9DS1<'a, T> where T: I2cInstance {
    pub fn new(i2c: &'a mut I2c<T>) -> Self {
        LSM9DS1 {
            addr: 0x1E,
            register_map: RegisterMap::new(),
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
        let res = self.i2c.write_read(self.addr, &[self.register_map.magnetometer.who_am_i], &mut rx_buffer);

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

    fn read_ctrl_reg3_m (&mut self) -> u8 {
        let mut rx_dat: [u8; 1] = [0; 1];
        let res = self.i2c.write_read(self.addr, &[self.register_map.magnetometer.ctrl_reg3_m], &mut rx_dat);
        rx_dat[0]
    }

    pub fn boot_magnetometer(&mut self) -> Result<(), ()> {
        let initial_value = self.read_ctrl_reg3_m();
        let mut mode: u8 = 0x0; //Continuous mode. Refer to table 117 in the datasheet
        let res = self.i2c.write(self.addr, &[self.register_map.magnetometer.ctrl_reg3_m, mode]);
        let final_value = self.read_ctrl_reg3_m();

        if initial_value != final_value && final_value == mode {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn twos_compliment(&self, high: u8, low: u8) -> i16 {
        let combined = LittleEndian::read_u16(&[low, high]);
        let is_negative = (combined & 0x8000) != 0;

        let mut result = combined as i16;
        if is_negative {
            result = -result;
        }

        result
    }

    pub fn calibrate_magnetometer(&mut self) {
        let mut rx_buffer: [u8; 6] = [0; 6]; //The magnetometer has 6 registers that need to be read to get the calibration data, 2 for each axis
        let res = self.i2c.write_read(self.addr, &[self.register_map.magnetometer.out_x_l_m], &mut rx_buffer);

        let xl = rx_buffer[0];
        let xh = rx_buffer[1];
        let yl = rx_buffer[2];
        let yh = rx_buffer[3];
        let zl = rx_buffer[4];
        let zh = rx_buffer[5];

        let x_offset = self.twos_compliment(xh, xl);
        let y_offset = self.twos_compliment(yh, yl);
        let z_offset = self.twos_compliment(zh, zl);

        self.calibration_info.magnetometer.x_offset = x_offset as i32;
        self.calibration_info.magnetometer.y_offset = y_offset as i32;
        self.calibration_info.magnetometer.z_offset = z_offset as i32;
    }
}