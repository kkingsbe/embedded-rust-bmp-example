use crate::sensor::imu::IMU;
use cortex_m::asm::nop;
use stm32f4xx_hal::i2c::Instance as I2cInstance;
use super::lsm9ds1_s::LSM9DS1;

pub enum Axis {
    X,
    Y,
    Z
}

impl<'a, T> LSM9DS1<'a, T> where T: I2cInstance {
    pub fn read_raw_magnetometer_axis(&mut self, axis: Axis) -> u32 {
        let mut rx_buffer: [u8; 2] = [0; 2];
        let mut addr: u8 = match axis {
            Axis::X => self.register_map.magnetometer.out_x_l_m,
            Axis::Y => self.register_map.magnetometer.out_y_l_m,
            Axis::Z => self.register_map.magnetometer.out_z_l_m
        };

        let res = self.i2c.write_read(self.addr, &[addr], &mut rx_buffer);
        let combined = ((rx_buffer[1] as u32) << 8) | rx_buffer[0] as u32;
        let result = combined; /*= if combined & 0x8000 != 0 {
            !combined + 1
        } else {
            combined
        };*/

        let correction_value = match axis {
            Axis::X => self.calibration_info.magnetometer.x_offset,
            Axis::Y => self.calibration_info.magnetometer.y_offset,
            Axis::Z => self.calibration_info.magnetometer.z_offset
        };
        
        result - correction_value as u32
    }

    pub fn read_magnetometer_x(&mut self) -> u32 {
        self.read_raw_magnetometer_axis(Axis::X)
    }

    pub fn read_magnetometer_y(&mut self) -> u32 {
        self.read_raw_magnetometer_axis(Axis::Y)
    }

    pub fn read_magnetometer_z(&mut self) -> u32 {
        self.read_raw_magnetometer_axis(Axis::X)
    }
}

impl<'a, T> IMU for LSM9DS1<'a, T> where T: I2cInstance {
    fn read_acceleration(&mut self) -> (i32, i32, i32) {
        (0, 0, 0)
    }

    fn read_gyro(&mut self) -> (i32, i32, i32) {
        (0, 0, 0)
    }

    fn read_magnetometer(&mut self) -> (u32, u32, u32) {
        let x = self.read_magnetometer_x();
        let y = self.read_magnetometer_y();
        let z = self.read_magnetometer_z();

        (x, y, z)
    }
}