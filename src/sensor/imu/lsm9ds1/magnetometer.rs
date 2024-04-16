use crate::sensor::imu::Magnetometer;
use cortex_m::asm::nop;
use stm32f4xx_hal::i2c::Instance as I2cInstance;
use super::lsm9ds1_s::{MagnetometerRM, LSM9DS1};

pub enum Axis {
    X,
    Y,
    Z
}

impl<'a, T> LSM9DS1<'a, T> where T: I2cInstance {
    pub fn boot_magnetometer(&mut self) -> Result<(), ()> {
        let initial_value = self.read_ctrl_reg3_m();
        let mut mode: u8 = 0x0; //Continuous mode. Refer to table 117 in the datasheet
        let res = self.i2c.write(self.m_addr, &[MagnetometerRM::CtrlReg3M as u8, mode]);
        let final_value = self.read_ctrl_reg3_m();

        if initial_value != final_value && final_value == mode {
            Ok(())
        } else {
            Err(())
        }
    }
    
    fn read_ctrl_reg3_m (&mut self) -> u8 {
        let mut rx_dat: [u8; 1] = [0; 1];
        let res = self.i2c.write_read(self.m_addr, &[MagnetometerRM::CtrlReg3M as u8], &mut rx_dat);
        rx_dat[0]
    }

    pub fn calibrate_magnetometer(&mut self) {
        let mut rx_buffer: [u8; 6] = [0; 6]; //The magnetometer has 6 registers that need to be read to get the calibration data, 2 for each axis
        let res = self.i2c.write_read(self.m_addr, &[MagnetometerRM::OutXLM as u8], &mut rx_buffer);

        let xl = rx_buffer[0];
        let xh = rx_buffer[1];
        let yl = rx_buffer[2];
        let yh = rx_buffer[3];
        let zl = rx_buffer[4];
        let zh = rx_buffer[5];

        let x_offset = self.twos_complement(xh, xl);
        let y_offset = self.twos_complement(yh, yl);
        let z_offset = self.twos_complement(zh, zl);

        self.calibration_info.magnetometer.x_offset = x_offset as i32;
        self.calibration_info.magnetometer.y_offset = y_offset as i32;
        self.calibration_info.magnetometer.z_offset = z_offset as i32;
    }

    pub fn read_raw_magnetometer_axis(&mut self, axis: Axis) -> i32 {
        let range = 4; //4 gauss full range

        let mut rx_buffer: [u8; 2] = [0; 2];
        let mut addr: u8 = match axis {
            Axis::X => MagnetometerRM::OutXLM as u8,
            Axis::Y => MagnetometerRM::OutYLM as u8,
            Axis::Z => MagnetometerRM::OutZLM as u8
        };

        //Incoming data is little-endian by default
        let res = self.i2c.write_read(self.m_addr, &[addr], &mut rx_buffer);
        let high = rx_buffer[1];
        let low = rx_buffer[0];
        let result = self.twos_complement(high, low);

        let correction_value = match axis {
            Axis::X => self.calibration_info.magnetometer.x_offset,
            Axis::Y => self.calibration_info.magnetometer.y_offset,
            Axis::Z => self.calibration_info.magnetometer.z_offset
        };

        (result as i32 - correction_value) / range
    }

    pub fn read_magnetometer_x(&mut self) -> i32 {
        self.read_raw_magnetometer_axis(Axis::X)
    }

    pub fn read_magnetometer_y(&mut self) -> i32 {
        self.read_raw_magnetometer_axis(Axis::Y)
    }

    pub fn read_magnetometer_z(&mut self) -> i32 {
        self.read_raw_magnetometer_axis(Axis::X)
    }
}

impl<'a, T> Magnetometer for LSM9DS1<'a, T> where T: I2cInstance {
    fn read_magnetometer(&mut self) -> (i32, i32, i32) {
        let x = self.read_magnetometer_x();
        let y = self.read_magnetometer_y();
        let z = self.read_magnetometer_z();

        (x, y, z)
    }
}