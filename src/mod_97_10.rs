use crate::pure::PureSystem;
use crate::system::System;
use crate::Alphabet;

pub struct Mod97_10();

impl System for Mod97_10 {
    const NAME: &'static str = "ISO/IEC 7064, MOD 97-10";
    const DESIGNATION: u8 = 3;
    const ALPHABET: Alphabet = Alphabet::Numeric;
    const CHECK_LENGTH: u8 = 2;
}

impl PureSystem for Mod97_10 {
    const MODULUS: u16 = 97;
    const RADIX: u8 = 10;
}

#[cfg(test)]
mod tests {
    use crate::mod_97_10::Mod97_10;
    use crate::pure::PureSystem;

    // Test case from The Standard, Section 8.4 "Simplified procedure for ISO/IEC 7064, MOD 97-10"
    #[test]
    fn checksum_example_from_standard() {
        assert_eq!(Some(44), Mod97_10::checksum_string("794"));
    }
}
