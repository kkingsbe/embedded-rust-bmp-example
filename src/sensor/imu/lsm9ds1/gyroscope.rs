use crate::sensor::imu::Gyroscope;
use cortex_m::asm::nop;
use stm32f4xx_hal::i2c::Instance as I2cInstance;
use super::lsm9ds1_s::{GyroOdr, GyroRM, LSM9DS1};

impl<'a, T  > LSM9DS1<'a, T> where T: I2cInstance {
    pub fn boot_gyroscope(&mut self) -> Result<(), ()> {
        let odr = GyroOdr::CONTINUOUS;
        let mut reg_value = (odr as u8) << 5;
        reg_value = reg_value | 0b00011;

        let initial_value = self.read_ctrl_reg1_g();
        let res = self.i2c.write(self.addr, &[GyroRM::CtrlReg1G as u8, reg_value]);
        let final_value = self.read_ctrl_reg1_g();

        if initial_value != final_value && final_value == reg_value {
            Ok(())
        } else {
            Err(())
        }
    }
    
    pub fn calibrate_gyroscope(&mut self) {
        //todo
        //todo!();
    }

    fn read_ctrl_reg1_g (&mut self) -> u8 {
        let mut rx_dat: [u8; 1] = [0; 1];
        let res = self.i2c.write_read(self.addr, &[GyroRM::CtrlReg1G as u8], &mut rx_dat);
        rx_dat[0]
    }
}


impl<'a, T> Gyroscope for LSM9DS1<'a, T> where T: I2cInstance {
    fn read_gyro(&mut self) -> (f32, f32, f32) {
        let range = 2000; //2000dps max reading

        let mut rx_buffer: [u8; 6] = [0; 6];
        let res = self.i2c.write_read(self.addr, &[GyroRM::OutXGL as u8], &mut rx_buffer);
        let x_raw = self.twos_complement(rx_buffer[1], rx_buffer[0]);
        let y_raw = self.twos_complement(rx_buffer[3], rx_buffer[2]);
        let z_raw = self.twos_complement(rx_buffer[5], rx_buffer[4]);
        
        let x = ((x_raw as i32 * range as i32) as f64 / i16::max_value() as f64) as f32;
        let y = ((y_raw as i32 * range as i32) as f64 / i16::max_value() as f64) as f32;
        let z = ((z_raw as i32 * range as i32) as f64 / i16::max_value() as f64) as f32;

        (x, y, z)
    }
}