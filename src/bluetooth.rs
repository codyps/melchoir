#![allow(dead_code)]

use core::convert::TryInto;

pub enum AddrKind {
    Static,
    PrivateNonResolvable,
    PrivateResolvable,
    Unknown,
}

pub struct Addr {
    r: [u8; 48/8],
}

impl Addr {
    pub fn from_bytes(r: &[u8]) -> Self {
        Addr {
            r: r.try_into().unwrap(),
        }
    }

    pub fn kind(&self) -> AddrKind {
        // examine the 2 low bits
        match self.r[48/8 - 1] & 0b11 {
            0b11 => AddrKind::Static,
            0b00 => AddrKind::PrivateNonResolvable,
            0b10 => AddrKind::PrivateResolvable,
            0b01 => AddrKind::Unknown,
            _ => panic!(),
        }
    }

    // rp addrs have 2 parts:
    //  - 24-bit hash
    //  - 24-bit prand
}
