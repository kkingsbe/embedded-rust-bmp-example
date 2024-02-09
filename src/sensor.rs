/*
pub enum SensorState {
    CALIBRATING,
    READY,
    ERROR(String)
}

pub struct SensorConfig {
    pub address: u8
}

pub trait Sensor {
    fn init(&self) -> SensorState;
    fn get_config(&self) -> SensorConfig;
    fn set_config(&self, config: SensorConfig) -> SensorState;
    fn read(&self) -> f32;
}
*/