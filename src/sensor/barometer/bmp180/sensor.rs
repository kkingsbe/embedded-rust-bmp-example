use crate::BMP180;
use crate::sensor::Sensor;
use crate::sensor::SensorState;
use stm32f4xx_hal::i2c::{I2c, Instance as I2cInstance};

impl <'a, T> Sensor for BMP180<'a, T> where T: I2cInstance {
    fn init(&mut self) -> Result<(), ()> {
        self.state = SensorState::STARTUP;
        let is_discovered = self.sanity_check();
        if !is_discovered {
            return Err(());
        }

        self.calibrate()
    }

    fn calibrate(&mut self) -> Result<(), ()> {
        self.state = SensorState::CALIBRATING;

        self.calib_coeffs.ac5 = self.read_calibration_coefficient(self.register_map.ac5_msb_addr)?;
        self.calib_coeffs.ac6 = self.read_calibration_coefficient(self.register_map.ac6_msb_addr)?;
        self.calib_coeffs.mc = self.read_calibration_coefficient(self.register_map.mc_msb_addr)?;
        self.calib_coeffs.md = self.read_calibration_coefficient(self.register_map.md_msb_addr)?;

        self.state = SensorState::READY;

        Ok(())
    }

    fn sensor_state(&self) -> &SensorState {
        return &self.state;
    }
}