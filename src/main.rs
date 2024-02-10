//cargo flash --chip stm32f411ceux --release

#![deny(unsafe_code)]
#![deny(warnings)]
#![allow(unused)]
#![no_main]
#![no_std]

pub mod sensor;
use crate::sensor::barometer::Barometer;
use crate::sensor::Sensor;
use hal::serial::Config;
use heapless::String;
use sensor::barometer::bmp180::BMP180;

//pub mod usb;

use cortex_m::asm::nop;
use panic_halt as _;
use stm32f4xx_hal as hal;
//use usb::USB;

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

    //WIP
    //let mut usb = USB::new(&p, rx_pin, tx_pin, &clocks);
    let serial_config = Config::default().baudrate(9600.bps());
    let mut serial = Serial::new(p.USART1, (tx_pin, rx_pin), serial_config, &clocks).unwrap();
    
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
}