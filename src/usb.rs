use core::cell::RefCell;
use core::fmt::Write;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use stm32f4xx_hal::otg_fs::{UsbBus, USB};
use stm32f4xx_hal::pac::{interrupt, Interrupt};
use stm32f4xx_hal::rcc::Clocks;
use stm32f4xx_hal::{pac, prelude::*};
use usb_device::class_prelude::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::SerialPort;
use heapless::String;
use stm32f4xx_hal::gpio::alt::otg_fs as alt;

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

struct MmlUsb<'a> {
    serial: Mutex<RefCell<Option<SerialPort<'a, UsbBus<USB>>>>>,
    device: Mutex<RefCell<Option<UsbDevice<'a, UsbBus<USB>>>>>,
    bus: UsbBusAllocator<UsbBus<USB>>,
}

impl<'a> MmlUsb<'a> {
    pub fn new(periphs: (pac::OTG_FS_GLOBAL, pac::OTG_FS_DEVICE, pac::OTG_FS_PWRCLK), pins: (impl Into<alt::Dm>, impl Into<alt::Dp>), clocks: &Clocks) -> Self {        
        let usb = USB::new(
            periphs,
            pins,
            &clocks
        );
        
        let mut mml_usb = MmlUsb {
            serial: Mutex::new(RefCell::new(None)),
            device: Mutex::new(RefCell::new(None)),
            bus: stm32f4xx_hal::otg_fs::UsbBusType::new(usb, unsafe { &mut EP_MEMORY })
        };

        mml_usb.init();

        mml_usb
    }

    fn init(&mut self) {
        cortex_m::interrupt::free(|cs| {
            *self.serial.borrow(cs).borrow_mut() = Some(usbd_serial::SerialPort::new(&self.bus));
            *self.device.borrow(cs).borrow_mut() = Some(
                UsbDeviceBuilder::new(&self.bus, UsbVidPid(0x16c0, 0x27dd))
                .strings(&[StringDescriptors::default()
                    .manufacturer("Fake company")
                    .product("Serial port")
                    .serial_number("TEST")
                ])
                .unwrap()
                .build(),
            );
        });

        unsafe {
            cortex_m::peripheral::NVIC::unmask(Interrupt::OTG_FS);
        }
    }
}