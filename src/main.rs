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
use hal::{dwt::{self, MonoTimer}, interrupt, pac::USART1, serial::Config};
use heapless::String;

use cortex_m::asm::nop;
use panic_halt as _;
use sensor::{barometer::bmp180::bmp180_s::BMP180, imu::{lsm9ds1::{self, lsm9ds1_s::LSM9DS1}, Gyroscope, Magnetometer, Accelerometer}};
use stm32f4xx_hal as hal;
use usb::USB;

use micromath::F32Ext;

use crate::hal::{pac, prelude::*, serial::Serial};
use cortex_m_rt::{entry, exception};
use stm32f4xx_hal::i2c::Mode;

use core::fmt::Write;

use imu_fusion::{FusionAhrsSettings, FusionVector};

use cortex_m::peripheral::scb::Exception::SysTick;
use cortex_m::interrupt::Mutex;
use core::cell::RefCell;

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

    let ahrs_settings = FusionAhrsSettings::new();
    let data_rate = 35;
    let mut fusion = imu_fusion::Fusion::new(data_rate, ahrs_settings); //50hz update rate

    
    let mut timestamp: f32 = 0.;
    let mut i: u32 = 0;

    loop {
        let acc = imu.read_acceleration();
        let g = imu.read_gyro();
        let m = imu.read_magnetometer();
        
        let acc_fv = FusionVector::new(acc.0 as f32, acc.1 as f32, acc.2 as f32);
        let g_fv = FusionVector::new(g.0 as f32, g.1 as f32, g.2 as f32);
        let m_fv = FusionVector::new(m.0 as f32, m.1 as f32, m.2 as f32);

        let heading = (m.0 as f32).atan2(m.1 as f32).to_degrees();
        let pitch = (acc.0 as f32).atan2(((acc.1 * acc.1) + (acc.2 * acc.2)).sqrt()).to_degrees();
        let roll = (acc.1 as f32).atan2(((acc.0 * acc.0) + (acc.2 * acc.2)).sqrt()).to_degrees();

        //timestamp += 1.0 / data_rate as f32;
        //fusion.update(g_fv, acc_fv, m_fv, timestamp as f32);
        //let euler = fusion.euler();

        //let angle = (m.0 as f32).atan2(m.1 as f32).to_degrees();

        //let write_res = write!(message, "Acc_X: {}, Acc_Y: {}, Acc_Z: {}", acc.0, acc.1, acc.2);
        
        let mut message: String<128> = String::new();
        //let write_res = write!(message, "Gyro_X: {}, Gyro_Y: {}, Gyro_Z: {}\n", g.0, g.1, g.2);
        
        //let write_res = write!(message, "Mag_X: {}, Mag_Y: {}, Mag_Z: {}\n", m.0, m.1, m.2);
        
        let write_res = write!(message, "Orientation: {}, {}, {}", roll, pitch, heading);

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