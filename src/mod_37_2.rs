use crate::Alphabet;
use crate::PureCheckCharacterSystem;

pub struct Mod37_2();

impl PureCheckCharacterSystem for Mod37_2 {
    const NAME: &'static str = "ISO/IEC 7064, MOD 37-2";
    const DESIGNATION: u8 = 2;
    const MODULUS: u16 = 37;
    const RADIX: u8 = 2;
    const ALPHABET: Alphabet = Alphabet::AlphanumericWithAsterisk;
    const CHECK_LENGTH: u8 = 1;
}
