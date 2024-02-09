use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs; //Bring the DelayMs trait into scope
use crate::sensor::I2cInstance;
use super::BMP180;
use super::super::Barometer;

impl<'a, T> Barometer for BMP180<'a, T> where T: I2cInstance {
    fn pressure(&self) -> i32 {
        self.data.pressure
    }

    fn temperature(&self) -> i32 {
        self.data.temperature
    }

    fn read_pressure(&mut self) -> i32 {
        self.data.pressure = 0; //TODO
        return 0
    }

    //Reads the temperature in celcius and stores it
    fn read_temperature(&mut self) -> i32 {
        self.i2c.write(self.addr, &[self.register_map.ctrl_meas_addr, 0x2E]).unwrap();

        self.delay.delay_ms(5_u32);

        let mut rx_buffer: [u8; 2] = [0; 2];
        let mut rx_word: i16;
        self.i2c.write(self.addr, &[self.register_map.meas_out_msb_addr]).unwrap();
        self.i2c.read(self.addr, &mut rx_buffer).unwrap();
        rx_word = (rx_buffer[0] as i16) << 8;

        self.i2c.write(self.addr, &[self.register_map.meas_out_lsb_addr]).unwrap();
        self.i2c.read(self.addr, &mut rx_buffer).unwrap();
        rx_word |= rx_buffer[0] as i16;

        //Correct the temperature value
        let x1 = (rx_word as i32 - self.calib_coeffs.ac6 as i32) * (self.calib_coeffs.ac5 as i32) >> 15;
        let x2 = ((self.calib_coeffs.mc as i32) << 11) / (x1 + self.calib_coeffs.md as i32);
        let b5 = x1 + x2;
        let t = ((b5 + 8) >> 4) / 10;

        self.data.temperature = t;

        return t
    }
}