//cargo flash --chip stm32f411ceux --release

#![deny(unsafe_code)]
#![deny(warnings)]
#![allow(unused)]
#![no_main]
#![no_std]

pub mod sensor;
pub mod pwm;
pub mod usb;

use multi_mission_library;

use crate::sensor::barometer::Barometer;
use crate::sensor::Sensor;
use pwm::servo::pca9685::pca9685_s::Pca9685;
use hal::{pac::USART1, serial::Config};
use heapless::String;

use cortex_m::asm::nop;
use panic_halt as _;
use sensor::{barometer::bmp180::bmp180_s::BMP180, imu::{lsm9ds1::{self, lsm9ds1_s::LSM9DS1}, Gyroscope, Magnetometer, Accelerometer}};
use stm32f4xx_hal as hal;
use usb::USB;

use micromath::F32Ext;

use crate::hal::{pac, prelude::*, serial::Serial};
use cortex_m_rt::entry;
use stm32f4xx_hal::i2c::Mode;

use core::fmt::Write;

#[entry]
fn main() -> ! {
    let mut p = pac::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.MHz()).freeze();

    let gpioa = p.GPIOA.split();
    let tx_pin = gpioa.pa9;
    let rx_pin = gpioa.pa10;

    let mut usb = USB::new(p.USART1, rx_pin, tx_pin, &clocks);
    
    let gpiob = p.GPIOB.split();

    let scl = gpiob.pb6;
    let sda = gpiob.pb7;

    let mut i2c = p.I2C1.i2c(
        (scl, sda),
        Mode::Standard {
            frequency: 100.kHz()
        },
        &clocks
    );

    let mut delay = p.TIM1.delay_ms(&clocks);

    let mut imu = LSM9DS1::new(&mut i2c);
    imu.init();
    imu.calibrate();
    let mut i: u32 = 0;

    loop {
        let acc = imu.read_acceleration();
        let g = imu.read_gyro();
        let m = imu.read_magnetometer();

        let angle = (m.0 as f32).atan2(m.1 as f32).to_degrees();

        //let write_res = write!(message, "Acc_X: {}, Acc_Y: {}, Acc_Z: {}", acc.0, acc.1, acc.2);
        
        let mut message: String<128> = String::new();
        //let write_res = write!(message, "Gyro_X: {}, Gyro_Y: {}, Gyro_Z: {}\n", g.0, g.1, g.2);
        
        //let write_res = write!(message, "Mag_X: {}, Mag_Y: {}, Mag_Z: {}\n", m.0, m.1, m.2);
        
        let write_res = write!(message, "Angle: {}deg\n", angle);

        usb.println(&message.as_str());
        delay.delay_ms(10);
    }

    /*
    let mut driver = Pca9685::new(&mut i2c);

    let mut i = 0.0;
    let min_ds = 0.1;
    let max_ds = 0.5;
    let mut forwards = true;
    //driver.set_pwm(0, 0.3);
    loop {
        driver.set_pwm(0, i);

        if i >= max_ds {
            forwards = false;
        } else if i <= min_ds {
            forwards = true;
        }

        if forwards {
            i += 0.01;
        } else {
            i -= 0.01;
        }

        delay.delay_ms(10);
    }
    */

    //Initialize the sensor
    /*
    let mut lsm9ds1 = LSM9DS1::new(&mut i2c);
    let init_res = lsm9ds1.init();
    if init_res.is_err() {
        loop {}
    }

    loop {
        let val = lsm9ds1.read_magnetometer();
        let mut message: String<128> = String::new();
        let write_res = write!(message, "Mag_X: {}, Mag_Y: {}, Mag_Z: {}", val.0, val.1, val.2);
        if write_res.is_err() {
            loop {}
        }
        usb.println(&message.as_str());
        delay.delay_ms(100);
    }
    */

    /*
    let mut bmp180 = BMP180::new(&mut i2c);
    let init_res = bmp180.init();
    if init_res.is_err() {
        loop {}
    }

    loop {
        //Take temperature measurement. Only pass reference to delay struct when needed, so that other parts of the program can use it
        let temperature = bmp180.read_temperature(&mut delay);
        //let mut teststring: String<64> = String::new();
        //teststring.write_fmt(format_args!("Temperature: {}", temperature)).unwrap();

        //usb.println(&teststring);
        writeln!(serial, "Temperature: {}", temperature).unwrap();
        delay.delay_ms(100);
        nop();
    }
    */
}