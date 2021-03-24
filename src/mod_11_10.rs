use crate::Alphabet;
use crate::HybridCheckCharacterSystem;

pub struct Mod11_10();

impl HybridCheckCharacterSystem for Mod11_10 {
    const NAME: &'static str = "ISO/IEC 7064, MOD 11,10";
    const DESIGNATION: u8 = 6;
    const MODULUS: u8 = 10;
    const ALPHABET: Alphabet = Alphabet::Numeric;
}

#[cfg(test)]
mod tests {
    use crate::mod_11_10::Mod11_10;
    use crate::HybridCheckCharacterSystem;

    // Test case from The Standard, Section 10.1.2 "Example"
    #[test]
    fn validate_example_from_standard() {
        assert_eq!(true, Mod11_10::validate("07945"));
    }

    // Test cases manually derived from The Standard, Section 10.1.2 "Example"
    #[test]
    fn validate_examples_derived_from_standard() {
        assert_eq!(true, Mod11_10::validate("07904"));
        assert_eq!(true, Mod11_10::validate("07912"));
        assert_eq!(true, Mod11_10::validate("07929"));
        assert_eq!(true, Mod11_10::validate("07937"));

        assert_eq!(true, Mod11_10::validate("07953"));
        assert_eq!(true, Mod11_10::validate("07961"));
        assert_eq!(true, Mod11_10::validate("07970"));
        assert_eq!(true, Mod11_10::validate("07988"));
        assert_eq!(true, Mod11_10::validate("07996"));
    }

    // Test case from The Standard, Section 10.1.2 "Example"
    #[test]
    fn compute_example_from_standard() {
        assert_eq!(Some(5), Mod11_10::compute("0794"));
    }

    // Test cases manually derived from The Standard, Section 10.1.2 "Example"
    #[test]
    fn compute_examples_derived_from_standard() {
        assert_eq!(Some(4), Mod11_10::compute("0790"));
        assert_eq!(Some(2), Mod11_10::compute("0791"));
        assert_eq!(Some(9), Mod11_10::compute("0792"));
        assert_eq!(Some(7), Mod11_10::compute("0793"));

        assert_eq!(Some(3), Mod11_10::compute("0795"));
        assert_eq!(Some(1), Mod11_10::compute("0796"));
        assert_eq!(Some(0), Mod11_10::compute("0797"));
        assert_eq!(Some(8), Mod11_10::compute("0798"));
        assert_eq!(Some(6), Mod11_10::compute("0799"));
    }
}
