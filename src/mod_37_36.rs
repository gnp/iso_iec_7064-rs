use crate::hybrid::HybridSystem;
use crate::system::System;
use crate::Alphabet;

pub struct Mod37_36();

impl System for Mod37_36 {
    const NAME: &'static str = "ISO/IEC 7064, MOD 37,36";
    const DESIGNATION: u8 = 8;
    const ALPHABET: Alphabet = Alphabet::Alphanumeric;
    const CHECK_LENGTH: u8 = 1;
}

impl HybridSystem for Mod37_36 {
    const MODULUS: u8 = 36;
}
