use crate::pure::PureSystem;
use crate::system::System;
use crate::Alphabet;

pub struct Mod661_26();

impl System for Mod661_26 {
    const NAME: &'static str = "ISO/IEC 7064, MOD 661-26";
    const DESIGNATION: u8 = 4;
    const ALPHABET: Alphabet = Alphabet::Alphabetic;
    const CHECK_LENGTH: u8 = 2;
}

impl PureSystem for Mod661_26 {
    const MODULUS: u16 = 661;
    const RADIX: u8 = 26;
}
