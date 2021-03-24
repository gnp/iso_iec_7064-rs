use crate::Alphabet;
use crate::PureCheckCharacterSystem;

pub struct Mod11_2();

impl PureCheckCharacterSystem for Mod11_2 {
    const NAME: &'static str = "ISO/IEC 7064, MOD 11-2";
    const DESIGNATION: u8 = 1;
    const MODULUS: u16 = 11;
    const RADIX: u8 = 2;
    const ALPHABET: Alphabet = Alphabet::NumericWithX;
    const CHECK_LENGTH: u8 = 1;
}

#[cfg(test)]
mod tests {
    use crate::mod_11_2::Mod11_2;
    use crate::PureCheckCharacterSystem;

    // Test cases from The Standard, Section 7.1.2 "Example"
    #[test]
    fn validate_examples_from_standard() {
        assert_eq!(true, Mod11_2::validate("07940"));
        assert_eq!(false, Mod11_2::validate("0794X"));

        assert_eq!(true, Mod11_2::validate("079X"));
        assert_eq!(false, Mod11_2::validate("0790"));
    }

    #[test]
    fn validate_rejects_supplementary_chars_in_payload() {
        assert_eq!(false, Mod11_2::validate("X7940"));
    }

    // Test cases from The Standard, Section 7.1.2 "Example"
    #[test]
    fn compute_examples_from_standard() {
        assert_eq!(Some(0), Mod11_2::compute("0794"));

        assert_eq!(Some(10), Mod11_2::compute("079"));
    }
}