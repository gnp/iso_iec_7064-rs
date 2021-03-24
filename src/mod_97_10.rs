use crate::Alphabet;
use crate::PureCheckCharacterSystem;

pub struct Mod97_10();

impl PureCheckCharacterSystem for Mod97_10 {
    const NAME: &'static str = "ISO/IEC 7064, MOD 97-10";
    const DESIGNATION: u8 = 3;
    const MODULUS: u16 = 97;
    const RADIX: u8 = 10;
    const ALPHABET: Alphabet = Alphabet::Numeric;
    const CHECK_LENGTH: u8 = 2;
}

#[cfg(test)]
mod tests {
    use crate::mod_97_10::Mod97_10;
    use crate::PureCheckCharacterSystem;

    // Test case from The Standard, Section 8.4 "Simplified procedure for ISO/IEC 7064, MOD 97-10"
    #[test]
    fn compute_example_from_standard() {
        assert_eq!(Some(44), Mod97_10::compute("794"));
    }
}
