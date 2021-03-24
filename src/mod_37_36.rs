use crate::Alphabet;
use crate::HybridCheckCharacterSystem;

pub struct Mod37_36();

impl HybridCheckCharacterSystem for Mod37_36 {
    const NAME: &'static str = "ISO/IEC 7064, MOD 37,36";
    const DESIGNATION: u8 = 8;
    const MODULUS: u8 = 36;
    const ALPHABET: Alphabet = Alphabet::Alphanumeric;
}
