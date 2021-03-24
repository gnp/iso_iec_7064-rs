use crate::Alphabet;
use crate::HybridCheckCharacterSystem;

pub struct Mod27_26();

impl HybridCheckCharacterSystem for Mod27_26 {
    const NAME: &'static str = "ISO/IEC 7064, MOD 27,26";
    const DESIGNATION: u8 = 7;
    const MODULUS: u8 = 26;
    const ALPHABET: Alphabet = Alphabet::Alphabetic;
}
