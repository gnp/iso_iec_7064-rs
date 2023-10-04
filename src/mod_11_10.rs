use crate::hybrid::HybridSystem;
// use crate::Alphabet;

pub const MOD_11_10: HybridSystem<9> = HybridSystem {
    name: "ISO/IEC 7064, MOD 11,10",
    designation: 6,
    // alphabet: Alphabet::Numeric,
    modulus: 10,
};

#[cfg(test)]
mod tests {
    use crate::mod_11_10::MOD_11_10;
    use crate::System;

    // Test case from The Standard, Section 10.1.2 "Example"
    #[test]
    fn validate_example_from_standard() {
        assert!(MOD_11_10.validate_string("07945"));
    }

    // Test cases manually derived from The Standard, Section 10.1.2 "Example"
    #[test]
    fn validate_examples_derived_from_standard() {
        assert!(MOD_11_10.validate_string("07904"));
        assert!(MOD_11_10.validate_string("07912"));
        assert!(MOD_11_10.validate_string("07929"));
        assert!(MOD_11_10.validate_string("07937"));

        assert!(MOD_11_10.validate_string("07953"));
        assert!(MOD_11_10.validate_string("07961"));
        assert!(MOD_11_10.validate_string("07970"));
        assert!(MOD_11_10.validate_string("07988"));
        assert!(MOD_11_10.validate_string("07996"));
    }

    // Test case from The Standard, Section 10.1.2 "Example"
    #[test]
    fn checksum_example_from_standard() {
        assert_eq!(Some(5), MOD_11_10.checksum_string("0794"));
    }

    // Test cases manually derived from The Standard, Section 10.1.2 "Example"
    #[test]
    fn checksum_examples_derived_from_standard() {
        assert_eq!(Some(4), MOD_11_10.checksum_string("0790"));
        assert_eq!(Some(2), MOD_11_10.checksum_string("0791"));
        assert_eq!(Some(9), MOD_11_10.checksum_string("0792"));
        assert_eq!(Some(7), MOD_11_10.checksum_string("0793"));

        assert_eq!(Some(3), MOD_11_10.checksum_string("0795"));
        assert_eq!(Some(1), MOD_11_10.checksum_string("0796"));
        assert_eq!(Some(0), MOD_11_10.checksum_string("0797"));
        assert_eq!(Some(8), MOD_11_10.checksum_string("0798"));
        assert_eq!(Some(6), MOD_11_10.checksum_string("0799"));
    }
}
