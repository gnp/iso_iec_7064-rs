use crate::pure::PureSystem;
use crate::Alphabet;

pub const Mod661_26: PureSystem<2> = PureSystem {
    name: "ISO/IEC 7064, MOD 661-26",
    designation: 4,
    alphabet: Alphabet::Alphabetic,
    modulus: 661,
    radix: 26,
};
