#![deny(warnings)]
//#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_semihosting::{debug, hprintln};
use panic_semihosting as _;
use core::convert::TryInto;

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
    pub fn transmit(&mut self, buf: &'static [u8]) {
        // BLE PDU
        //
        // [ preamble | access-address | pdu         | crc | ct ]
        //   u8         u32              [u8;2..258]   u24 | 16 to 160 us
        // ^ BLE 1MBit
    
        // NRF PDU
        //
        // PREAMBLE | ADDR_BASE | ADDR_PREFIX | CI | TERM1 | S0 | LENGTH | S1 | PAYLOAD | CRC | TERM
        // pcnf0.plen
        //            txaddress/base{0,1}/prefix{0,1}
        //                                      pcnf0.cilen (lr)
        //                                           pcnf0.termleno
        //                                                   pcnf0.s0len
        //                                                        pcnf0.lflen
        //                                                                 pcnf0.s1len
        //                                                                                pcnf0.crcinc
        //                                                                                      pcnf0.termlen
        //                                                                                      (lr)

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

        // BLE Phy Channel 0, BLE Channel Index 37, a primary advertising channel
        // use 2402MHz ({2400, 2360}[0] + 2)
        self.radio.frequency.write(|w| {
            let w = w.frequency();
            // XXX-pac: all values are valid here
            let w = unsafe { w.bits(2) };
            w.map().default()
        });

        // Have to "own" buf until tx complete
        self.radio.packetptr.write(|w| {
            unsafe { w.packetptr().bits(buf.as_ptr() as u32) }
        });

        self.radio.txpower.write(|w| {
            w.txpower().neg40d_bm()
        });

        self.radio.mode.write(|w| {
            w.mode().ble_1mbit()
        });

        // TODO: select a base address based on some chip id
        self.radio.base0.write(|w| {
            unsafe { w.base0().bits(0xdeadbeaf) }
        });

        // TODO: part of the address in `base0` above
        self.radio.prefix0.write(|w| {
            unsafe { w.ap0().bits(0xaa) }
        });

        self.radio.pcnf0.write(|w| unsafe {
            // BLE 1MBit = preamble len = 8 bits
            w.plen()._8bit()
                // FIXME: check length field len
                .lflen().bits(1)
        });

        // TODO: validate length instead of truncating
        self.radio.pcnf1.write(|w| unsafe {
            w.maxlen().bits(buf.len().try_into().unwrap())
                // .statlen() ??
                .balen().bits(4)
                .endian().little()
                // FIXME: check the whitening setting for BLE
                .whiteen().enabled()
        });

        // index 0
        self.radio.txaddress.write(|w| {
            unsafe { w.txaddress().bits(0) } 
        });

        self.radio.crccnf.write(|_w| {
            // len
            // skipaddr
            todo!()
        });

        

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
