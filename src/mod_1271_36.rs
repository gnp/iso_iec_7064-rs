use crate::pure::PureSystem;
use crate::Alphabet;

pub const Mod1271_36: PureSystem<2> = PureSystem {
    name: "ISO/IEC 7064, MOD 1271-36",
    designation: 5,
    alphabet: Alphabet::Alphanumeric,
    modulus: 1271,
    radix: 36,
};

#[cfg(test)]
mod tests {
    use crate::mod_1271_36::Mod1271_36;
    use crate::System;

    // Test case from The Standard, Section 8.2 "Example using recursive method"
    #[test]
    fn compute_example_from_standard() {
        assert_eq!(Some(140), Mod1271_36.checksum_string("ISO79"));
    }
}
