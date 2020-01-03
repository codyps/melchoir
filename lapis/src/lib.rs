#![no_std]
#![allow(dead_code)]

use core::convert::TryInto;

const ADV_ACCESS_ADDR: u32 = 0x8E89_BED6;

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

/*
    The Access Address shall be a 32-bit value. Each time it needs a new Access Address, the Link
    Layer shall generate a new random value that meets the following requirements:

    1. It shall not be the Access Address for any existing Link Layer connection on this device.
    2. It shall not be the Access Address for any enabled periodic advertising train.
    3. It shall have no more than six consecutive zeros or ones.
    4. It shall not be the advertising channel packets’ Access Address.
    5. It shall not be a sequence that differs from the advertising physical channel packets’ Access Address by only one bit.
    6. It shall not have all four octets equal.
    7. It shall have no more than 24 transitions.
    8. It shall have a minimum of two transitions in the most significant six bits.

    The seed for the random number generator shall be from a physical source of entropy and should
    have at least 20 bits of entropy.
    If the random number does not meet the above requirements, new random numbers shall be
    generated until the requirements are met.

    On an implementation that also supports the LE Coded PHY (see Section 2.2), the Access Address
    shall also meet the following requirements:

    9. It shall have at least three ones in the least significant 8 bits.
    10. It shall have no more than eleven transitions in the least significant 16 bits.
*/
pub fn is_valid_access_addr(r: u32) -> bool {
    // 3. It shall have no more than six consecutive zeros or ones.
    let m = max_consecutive_bits(r);
    if m > 6 {
        return false;
    }

    // 4. It shall not be the advertising channel packets’ Access Address.
    if r == ADV_ACCESS_ADDR {
        return false;
    }

    // 5. It shall not be a sequence that differs from the advertising physical channel packets’ Access Address by only one bit.
    if (r ^ ADV_ACCESS_ADDR).count_ones() == 1 {
        return false;
    }

    // 6. It shall not have all four octets equal.
    {
        let a = r & 0xff;
        let b = (r >> 8) & 0xff;
        let c = (r >> 16) & 0xff;
        let d = (r >> 24) & 0xff;

        if a == b && b == c && c == d {
            return false;
        }
    }

    // 7. It shall have no more than 24 transitions.
    let t = transition_count(r);
    if t > 24 {
        return false;
    }

    // 8. It shall have a minimum of two transitions in the most significant six bits.
    
    todo!()
}

fn max_consecutive_bits(r: u32) -> u8 {
    let mut m = 1;
    let mut ct = 0;
    let mut ct_m = 0;
    let mut p = r & m;
    loop {
        m <<= 1;
        let v = r & m;
        let v = if v > 0 { 1 } else { 0 };
        if v == p {
            ct += 1;
        } else {
            if ct > ct_m {
                ct_m = ct;
            }

            ct = 0;
            p = v;
        }

        if m == (1 << 31) {
            break;
        }
    }

    if ct > ct_m {
        ct_m = ct;
    }

    ct_m
}

fn transition_count(r: u32) -> u8 {
    let mut m = 1;
    let mut p = r & m;
    let mut ct = 0;
    loop {
        m <<= 1;
        let v = r & m;
        let v = if v > 0 { 1 } else { 0 };
        if v != p {
            ct += 1;
        }
        p = v;

        if m == (1 << 31) {
            break
        }
    }

    ct
}

#[cfg(test)]
mod test {
    #[test]
    fn transition_count() {
        use super::transition_count;
        assert_eq!(transition_count(0b01), 1);
        assert_eq!(transition_count(0b010), 2);
        assert_eq!(transition_count(0b011), 1);
        assert_eq!(transition_count(0b0110), 2);
        assert_eq!(transition_count(0b0111), 1);
        assert_eq!(transition_count(0b0101), 3);
    }
}
