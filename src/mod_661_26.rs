use crate::Alphabet;
use crate::PureCheckCharacterSystem;

pub struct Mod661_26();

impl PureCheckCharacterSystem for Mod661_26 {
    const NAME: &'static str = "ISO/IEC 7064, MOD 661-26";
    const DESIGNATION: u8 = 4;
    const MODULUS: u16 = 661;
    const RADIX: u8 = 26;
    const ALPHABET: Alphabet = Alphabet::Alphabetic;
    const CHECK_LENGTH: u8 = 2;
}
