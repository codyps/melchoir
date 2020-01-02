#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

mod bluetooth;

use cortex_m_semihosting::{debug, hprintln};
use panic_semihosting as _;

struct Hfclkstat {
    r: nrf52840_pac::clock::hfclkstat::R,
}

impl core::fmt::Debug for Hfclkstat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Hfclkstat")
            .field("src", &self.r.src().variant())
            .field("state", &self.r.state().variant())
            .finish()
    }
}

struct LfclkstatSrc {
    r: nrf52840_pac::generic::Variant<u8, nrf52840_pac::clock::lfclkstat::SRC_A>
}

impl core::fmt::Debug for LfclkstatSrc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.r {
            nrf52840_pac::generic::Variant::Res(u) => { write!(f, "Res({:?})", u) },
            nrf52840_pac::generic::Variant::Val(v) => { write!(f, "Val({:?})", v) },
        }
    }
}

struct Lfclkstat {
    r: nrf52840_pac::clock::lfclkstat::R,
}

impl core::fmt::Debug for Lfclkstat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Lfclkstat")
            .field("src", &LfclkstatSrc { r: self.r.src().variant() })
            .field("state", &self.r.state().variant())
            .finish()
    }
}

pub struct Radio {
    radio: nrf52840_pac::RADIO
}

impl Radio {
    pub fn new(radio: nrf52840_pac::RADIO) -> Self {
        Self {
            radio
        }
    }

    // set everything up to perform a transmission of the provided data
    pub fn transmit(&mut self, _buf: &'static [u8]) {
        // configure:
        //  - address
        //  - field sizes (s0, s1, length)
        //  - PHY channel
        //  - PHY mode

        // connect:
        //  - trigger => RADIO::TX_EN
        //  - RADIO::READY => RADIO::START
        //  - RADIO::END (or PHYEND if mode is Ble_LR* or ieee802154_250Kbit) => RADIO::DISABLE
        //      - or RADIO::RXEN to switch to recving 
        self.radio.shorts.write(|w| w.ready_start().enabled());
        todo!()
    }
}

#[rtfm::app(device = nrf52840_pac, peripherals = true)]
const APP: () = {
    struct Resources {
        radio: Radio,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        static mut X: u32 = 0;

        // Cortex-M peripherals
        let _core: cortex_m::Peripherals = cx.core;

        // Device specific peripherals
        let device: nrf52840_pac::Peripherals = cx.device;

        // Safe access to local `static mut` variable
        let _x: &'static mut u32 = X;

        let hfclkstat = Hfclkstat { r: device.CLOCK.hfclkstat.read() };
        let lfclkstat = Lfclkstat { r: device.CLOCK.lfclkstat.read() };

        hprintln!("hfclkstat: {:?}", hfclkstat).unwrap();
        hprintln!("lfclkstat: {:?}", lfclkstat).unwrap();

        // we probably also need a timer here to trigger things
        let radio = Radio::new(device.RADIO); 

        hprintln!("init").unwrap();

        init::LateResources {
            radio: radio
        }
    }

     #[idle]
    fn idle(_: idle::Context) -> ! {
        static mut X: u32 = 0;

        // Safe access to local `static mut` variable
        let _x: &'static mut u32 = X;

        hprintln!("idle").unwrap();

        debug::exit(debug::EXIT_SUCCESS);

        loop {
            cortex_m::asm::wfi();
        }
    }
};
