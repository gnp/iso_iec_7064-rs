use crate::pure::PureSystem;
use crate::Alphabet;

pub const Mod37_2: PureSystem<1> = PureSystem {
    name: "ISO/IEC 7064, MOD 37-2",
    designation: 2,
    alphabet: Alphabet::AlphanumericWithAsterisk,
    modulus: 37,
    radix: 2,
};
