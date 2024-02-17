//cargo flash --chip stm32f411ceux --release

#![deny(unsafe_code)]
#![deny(warnings)]
#![allow(unused)]
#![no_main]
#![no_std]

pub mod sensor;
use crate::sensor::barometer::Barometer;
use crate::sensor::Sensor;
use hal::{pac::USART1, serial::Config};
use heapless::String;

pub mod usb;

use cortex_m::asm::nop;
use panic_halt as _;
use sensor::{barometer::bmp180::bmp180_s::BMP180, imu::{lsm9ds1::{self, lsm9ds1_s::LSM9DS1}, IMU}};
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

    //Initialize the sensor
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