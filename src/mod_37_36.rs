use crate::hybrid::HybridSystem;
use crate::Alphabet;

pub const Mod37_36: HybridSystem = HybridSystem {
    name: "ISO/IEC 7064, MOD 37,36",
    designation: 8,
    alphabet: Alphabet::Alphanumeric,
    modulus: 36,
};
