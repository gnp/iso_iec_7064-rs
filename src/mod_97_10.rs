use crate::pure::PureSystem;
// use crate::Alphabet;

pub const MOD_97_10: PureSystem<2, 9, 97, 10> = PureSystem {
    name: "ISO/IEC 7064, MOD 97-10",
    designation: 3,
    // alphabet: Alphabet::Numeric,
    // modulus: 97,
    // radix: 10,
};

#[cfg(test)]
mod tests {
    use crate::mod_97_10::MOD_97_10;
    use crate::System;

    // Test case from The Standard, Section 8.4 "Simplified procedure for ISO/IEC 7064, MOD 97-10"
    #[test]
    fn checksum_example_from_standard() {
        assert_eq!(Some(44), MOD_97_10.checksum_string("794"));
    }
}
