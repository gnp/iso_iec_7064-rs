use crate::hybrid::HybridSystem;
use crate::system::System;
use crate::Alphabet;

pub struct Mod27_26();

impl System for Mod27_26 {
    const NAME: &'static str = "ISO/IEC 7064, MOD 27,26";
    const DESIGNATION: u8 = 7;
    const ALPHABET: Alphabet = Alphabet::Alphabetic;
    const CHECK_LENGTH: u8 = 1;
}

impl HybridSystem for Mod27_26 {
    const MODULUS: u8 = 26;
}
