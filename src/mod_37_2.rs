use crate::pure::PureSystem;
use crate::system::System;
use crate::Alphabet;

pub struct Mod37_2();

impl System for Mod37_2 {
    const NAME: &'static str = "ISO/IEC 7064, MOD 37-2";
    const DESIGNATION: u8 = 2;
    const ALPHABET: Alphabet = Alphabet::AlphanumericWithAsterisk;
    const CHECK_LENGTH: u8 = 1;
}

impl PureSystem<1> for Mod37_2 {
    const MODULUS: usize = 37;
    const RADIX: usize = 2;
}
