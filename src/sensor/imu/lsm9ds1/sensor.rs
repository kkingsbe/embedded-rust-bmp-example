use crate::sensor::{Sensor, SensorState, imu::lsm9ds1::lsm9ds1_s::LSM9DS1};
use stm32f4xx_hal::i2c::{I2c, Instance as I2cInstance};

impl <'a, T> Sensor for LSM9DS1<'a, T> where T: I2cInstance {
    fn init(&mut self) -> Result<(), ()> {
        self.state = SensorState::STARTUP;
        let is_discovered = self.sanity_check();
        if !is_discovered {
            return Err(());
        }

        self.boot_magnetometer();

        self.calibrate()
    }

    fn calibrate(&mut self) -> Result<(), ()> {
        self.state = SensorState::CALIBRATING;

        self.calibrate_magnetometer();

        self.state = SensorState::READY;

        Ok(())
    }

    fn sensor_state(&self) -> &SensorState {
        &self.state
    }
}