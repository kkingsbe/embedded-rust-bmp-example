use embedded_hal::blocking::i2c::WriteRead;
use stm32f4xx_hal::{hal, pac};
use stm32f4xx_hal::gpio::alt::I2cCommon;
use stm32f4xx_hal::i2c::I2c;
use stm32f4xx_hal::pac::{I2C1, TIM1};
use stm32f4xx_hal::i2c::Instance as I2cInstance;
use stm32f4xx_hal::timer::{DelayMs, Timer, TimerExt};
use stm32f4xx_hal::rcc::Clocks;
use stm32f4xx_hal::timer::Instance as TimerInstance;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs; //Bring the DelayMs trait into scope

pub enum SensorError {
    I2CError,
    NotFound,
    FailedToCalibrate
}

pub enum SensorState {
    INITIAL,
    STARTUP,
    CALIBRATING,
    READY,
    ERROR(SensorError)
}

pub struct BMP180<'a, T> where T: I2cInstance {
    pub calib_coeffs: Coeffs,
    pub addr: u8,
    pub register_map: RegisterMap,
    pub i2c: &'a mut I2c<T>, //Allows for the BMP180 struct to not take ownership of the I2C instance, which means multiple devices can be on the same bus :)
    pub state: SensorState,
    pub delay: &'a mut DelayMs<TIM1>
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

impl<'a, T  > BMP180<'a, T> where T: I2cInstance {
    pub fn new(i2c: &'a mut I2c<T>, delay: &'a mut DelayMs<TIM1>) -> Self {
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
            delay
        }
    }

    pub fn init(&mut self) -> Result<(), ()> {
        self.state = SensorState::STARTUP;
        let is_discovered = self.sanity_check();
        if !is_discovered {
            return Err(());
        }

        self.calibrate()
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

    fn read_calibration_coefficient(&mut self, addr: u8) -> Result<i16, ()> {
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

    pub fn calibrate(&mut self) -> Result<(), ()> {
        self.state = SensorState::CALIBRATING;

        self.calib_coeffs.ac5 = self.read_calibration_coefficient(self.register_map.ac5_msb_addr)?;
        self.calib_coeffs.ac6 = self.read_calibration_coefficient(self.register_map.ac6_msb_addr)?;
        self.calib_coeffs.mc = self.read_calibration_coefficient(self.register_map.mc_msb_addr)?;
        self.calib_coeffs.md = self.read_calibration_coefficient(self.register_map.md_msb_addr)?;

        self.state = SensorState::READY;

        Ok(())
    }

    pub fn read_temperature(&mut self) -> u16 {
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

        t as u16
    }
}