use crate::hybrid::HybridSystem;
use crate::Alphabet;

pub const Mod27_26: HybridSystem = HybridSystem {
    name: "ISO/IEC 7064, MOD 27,26",
    designation: 7,
    alphabet: Alphabet::Alphabetic,
    modulus: 26,
};
