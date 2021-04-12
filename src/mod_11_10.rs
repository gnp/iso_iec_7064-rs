use crate::hybrid::HybridSystem;
use crate::Alphabet;

pub const Mod11_10: HybridSystem = HybridSystem {
    name: "ISO/IEC 7064, MOD 11,10",
    designation: 6,
    alphabet: Alphabet::Numeric,
    modulus: 10,
};

#[cfg(test)]
mod tests {
    use crate::mod_11_10::Mod11_10;
    use crate::System;

    // Test case from The Standard, Section 10.1.2 "Example"
    #[test]
    fn validate_example_from_standard() {
        assert_eq!(true, Mod11_10.validate_string("07945"));
    }

    // Test cases manually derived from The Standard, Section 10.1.2 "Example"
    #[test]
    fn validate_examples_derived_from_standard() {
        assert_eq!(true, Mod11_10.validate_string("07904"));
        assert_eq!(true, Mod11_10.validate_string("07912"));
        assert_eq!(true, Mod11_10.validate_string("07929"));
        assert_eq!(true, Mod11_10.validate_string("07937"));

        assert_eq!(true, Mod11_10.validate_string("07953"));
        assert_eq!(true, Mod11_10.validate_string("07961"));
        assert_eq!(true, Mod11_10.validate_string("07970"));
        assert_eq!(true, Mod11_10.validate_string("07988"));
        assert_eq!(true, Mod11_10.validate_string("07996"));
    }

    // Test case from The Standard, Section 10.1.2 "Example"
    #[test]
    fn checksum_example_from_standard() {
        assert_eq!(Some(5), Mod11_10.checksum_string("0794"));
    }

    // Test cases manually derived from The Standard, Section 10.1.2 "Example"
    #[test]
    fn checksum_examples_derived_from_standard() {
        assert_eq!(Some(4), Mod11_10.checksum_string("0790"));
        assert_eq!(Some(2), Mod11_10.checksum_string("0791"));
        assert_eq!(Some(9), Mod11_10.checksum_string("0792"));
        assert_eq!(Some(7), Mod11_10.checksum_string("0793"));

        assert_eq!(Some(3), Mod11_10.checksum_string("0795"));
        assert_eq!(Some(1), Mod11_10.checksum_string("0796"));
        assert_eq!(Some(0), Mod11_10.checksum_string("0797"));
        assert_eq!(Some(8), Mod11_10.checksum_string("0798"));
        assert_eq!(Some(6), Mod11_10.checksum_string("0799"));
    }
}
