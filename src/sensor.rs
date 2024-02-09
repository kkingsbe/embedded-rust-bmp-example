pub mod barometer;
use stm32f4xx_hal::i2c::{I2c, Instance as I2cInstance};

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

pub trait Sensor {
    fn init(&mut self) -> Result<(), ()>;
    fn calibrate(&mut self) -> Result<(), ()>;
    fn sensor_state(&self) -> &SensorState;
}