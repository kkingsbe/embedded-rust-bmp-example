//cargo flash --chip stm32f411ceux --release

#![deny(unsafe_code)]
#![deny(warnings)]
#![allow(unused)]
#![no_main]
#![no_std]

pub mod sensor;
use crate::sensor::barometer::Barometer;
use crate::sensor::Sensor;
use sensor::barometer::bmp180::BMP180;

use cortex_m::asm::nop;
use panic_halt as _;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};
use cortex_m_rt::entry;
use stm32f4xx_hal::i2c::Mode;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

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
    let mut bmp180 = BMP180::new(&mut i2c, &mut delay);
    let init_res = bmp180.init();
    if init_res.is_err() {
        loop {}
    }

    loop {
        //Take temperature measurement
        let temperature = bmp180.read_temperature();
        nop();
    }
}