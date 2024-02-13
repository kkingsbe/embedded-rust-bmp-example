use core::fmt::Write;
use stm32f4xx_hal::serial::Instance;
use stm32f4xx_hal::{gpio::{gpioa::Parts, GpioExt, Output, Pin, PushPull}, pac::{Peripherals, TIM1, USART1}, rcc::Clocks, serial::{CommonPins, Config, Instance as SerialInstance}, timer::Delay};
use crate::hal::{pac, prelude::*, serial::Serial}; // Add the import here

/**
 * Currently only supports USART1. Any pins can be used however.
 */
pub struct USB {
    serial: Serial<pac::USART1>
}

impl USB {
    pub fn new<'a, TxPin, RxPin>(usart: pac::USART1, rx_pin: RxPin, tx_pin: TxPin, clocks: &Clocks) -> Self 
    where 
        TxPin: Into<<pac::USART1 as CommonPins>::Tx<PushPull>>,
        RxPin: Into<<pac::USART1 as CommonPins>::Rx<PushPull>>
    {
        let serial_config = Config::default().baudrate(9600.bps());
        let mut serial = Serial::new(usart, (tx_pin, rx_pin), serial_config, &clocks).unwrap();

        USB {
            serial
        }
    }

    pub fn println(&mut self, message: &str) {
        writeln!(self.serial, "{}", message).unwrap();
    }
}
