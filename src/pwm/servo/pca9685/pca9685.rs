use cortex_m::asm::nop;
/**
 * LED_ON stores the counter value at which the PWM signal should go high
 * LED_OFF stores the counter value at which the PWM signal should go low
 * Therefore, if a delay is needed, LED_ON will be the delay value, and LED_OFF will be the delay value + the on time
 */

use stm32f4xx_hal::{i2c::{I2c, Instance as I2cInstance}, pac::TIM1, timer::DelayMs};
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs; //Bring the DelayMs trait into scope
use super::pca9685_s::Pca9685;

pub enum SetPwmError {
    InvalidChannel
}

impl<'a, T  > Pca9685<'a, T> where T: I2cInstance {
    pub fn new(i2c: &'a mut I2c<T>) -> Self {
        let mut device = Pca9685 {
            addr: 0x40,
            i2c,
        };
        device.initialize();
        device
    }

    //Take the device out of sleep mode and enable its internal oscillator
    pub fn initialize(&mut self) {
        let mut initial_mode_data: [u8; 1] = [0; 1];
        let mut final_mode_data: [u8; 1] = [0; 1];

        //Read existing value from mode register
        self.i2c.write_read(self.addr, &[0x00], &mut initial_mode_data).unwrap();
        nop();

        let mut mode1 = initial_mode_data[0];
        mode1 = mode1 & 0b0111; //Set bit 4 low while keeping other bits with original value
        mode1 = mode1 | 0b10000000; //Set bit 7 high while keeping other bits with original value

        //Write new value to mode register
        let write_res = self.i2c.write(self.addr, &[0x00, mode1]);
        nop();

        //Read existing value from mode register
        self.i2c.write_read(self.addr, &[0x00], &mut final_mode_data).unwrap();
        nop();
    }

    pub fn set_pwm(&mut self, channel: u8, duty_cycle: f32) -> Result<(), SetPwmError> {
        if channel > 15 {
            return Err(SetPwmError::InvalidChannel);
        }

        let on_time: u16 = (duty_cycle * 4095.0) as u16;

        //Determine address of the first of the 4 registers for the channel.
        let addr = 0x06 + (4 * channel);
        
        let mut data: [u8; 4] = [0; 4];
        data[0] = 0;
        data[1] = 0;
        data[2] = (on_time & 0xFF) as u8;
        data[3] = ((on_time >> 8) & 0xFF) as u8;

        self.send_pwm_value(addr, data[0]);
        self.send_pwm_value(addr + 1, data[1]);
        self.send_pwm_value(addr + 2, data[2]);
        self.send_pwm_value(addr + 3, data[3]);

        Ok(())
    }

    fn send_pwm_value(&mut self, register_addr: u8, value: u8) {
        let register_with_data: [u8; 2] = [register_addr, value];
        self.i2c.write(self.addr, &register_with_data).unwrap();
    }
}