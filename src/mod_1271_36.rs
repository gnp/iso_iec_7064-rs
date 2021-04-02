use crate::pure::PureSystem;
use crate::system::System;
use crate::Alphabet;

pub struct Mod1271_36();

impl System for Mod1271_36 {
    const NAME: &'static str = "ISO/IEC 7064, MOD 1271-36";
    const DESIGNATION: u8 = 5;
    const ALPHABET: Alphabet = Alphabet::Alphanumeric;
    const CHECK_LENGTH: u8 = 2;
}

impl PureSystem for Mod1271_36 {
    const MODULUS: u16 = 1271;
    const RADIX: u8 = 36;
}

#[cfg(test)]
mod tests {
    use crate::mod_1271_36::Mod1271_36;
    use crate::pure::PureSystem;

    // Test case from The Standard, Section 8.2 "Example using recursive method"
    #[test]
    fn compute_example_from_standard() {
        assert_eq!(Some(140), Mod1271_36::checksum_string("ISO79"));
    }
}
