use crate::pure::PureSystem;
// use crate::Alphabet;

pub const MOD_11_2: PureSystem<1, 10, 11, 2> = PureSystem {
    name: "ISO/IEC 7064, MOD 11-2",
    designation: 1,
    // alphabet: Alphabet::NumericWithX,
    // modulus: 11,
    // radix: 2,
};

#[cfg(test)]
mod tests {
    use crate::mod_11_2::MOD_11_2;
    use crate::System;

    // Test cases from The Standard, Section 7.1.2 "Example"
    #[test]
    fn validate_examples_from_standard() {
        assert!(MOD_11_2.validate_string("07940"));
        assert!(!MOD_11_2.validate_string("0794X"));

        assert!(MOD_11_2.validate_string("079X"));
        assert!(!MOD_11_2.validate_string("0790"));
    }

    #[test]
    fn validate_rejects_supplementary_chars_in_payload() {
        assert!(!MOD_11_2.validate_string("X7940"));
    }

    // Test cases from The Standard, Section 7.1.2 "Example"
    #[test]
    fn compute_examples_from_standard() {
        assert_eq!(Some(0), MOD_11_2.checksum_string("0794"));

        assert_eq!(Some(10), MOD_11_2.checksum_string("079"));
    }
}
