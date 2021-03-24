//! # iso_iec_7064
//!
//! Conforming implementation of [ISO/IEC 7064:2003](https://www.iso.org/standard/31531.html)
//! "Information technology &mdash; Security techniques &mdash; Check character systems" (The
//! Standard).
//!
//! This crate implements all the _Check Character Systems_ specified in The Standard.
//!
//! Note that the `mod_97_10` system is used by [ISO 17442-1:2020](https://www.iso.org/standard/78829.html)
//! "Financial services — Legal entity identifier (LEI) — Part 1: Assignment", which is implemented
//! by [the `lei` crate](https://crates.io/crates/lei).

use crate::alphabet::Alphabet;

mod alphabet;

// pub trait CheckCharacterSystem {
//     /// In The Standard, Section 5.4.2, Table 3 "Single digit designations" specifies official names
//     /// to be used to identify the different Check character systems it defines.
//     const NAME: &'static str;
//
//     /// In The Standard, Section 5.4.2, Table 3 "Single digit designations" specifies numbers to
//     /// be used to identify the different Check character systems it defines, should the need arise.
//     /// Implementations of this trait must provide a Standard-compliant designation. Note the value
//     /// zero is reserved for "No check character or non-standard system".
//     const DESIGNATION: u8;
//
//     /// The alphabet used in the Check character system, including the symbols allowed in the
//     /// payload and any additional symbols that may be allowed in the check character(s).
//     const ALPHABET: Alphabet;
//
//     /// The number of check characters the Check character system produces.
//     const CHECK_LENGTH: u8;
// }

pub trait PureCheckCharacterSystem {
    /// In The Standard, Section 5.4.2, Table 3 "Single digit designations" specifies official names
    /// to be used to identify the different Check character systems it defines.
    const NAME: &'static str;

    /// In The Standard, Section 5.4.2, Table 3 "Single digit designations" specifies numbers to
    /// be used to identify the different Check character systems it defines, should the need arise.
    /// Implementations of this trait must provide a Standard-compliant designation. Note the value
    /// zero is reserved for "No check character or non-standard system".
    const DESIGNATION: u8;

    const MODULUS: u16;

    const RADIX: u8;

    const ALPHABET: Alphabet;

    const CHECK_LENGTH: u8;

    /// Check that the input string, which must already have the check digit(s) appended, satisfies
    /// the check. If characters outside the ALPHABET are encountered, returns false immediately.
    fn validate(string: &str) -> bool {
        const MAX_CHECK_LENGTH: usize = 2;
        let check_length = Self::CHECK_LENGTH as usize;
        // We remember the last one or two char values so we can detect if we ever roll off a
        // _Supplementary Check Character_ ('X' for MOD 11-2 or '*' for MOD 32-2).
        let mut check_chars: [i8; MAX_CHECK_LENGTH] = [-1i8; MAX_CHECK_LENGTH];
        let supplementary_char: i8 = alphabet::supplementary_char_value(Self::ALPHABET);

        let modulus: usize = Self::MODULUS as usize;
        let radix: usize = Self::RADIX as usize;

        let mcv: usize = alphabet::max_char_value(Self::ALPHABET) as usize;
        let max_sum: usize = (usize::MAX - mcv) / radix;

        let mut sum: usize = 0;

        for c in string.as_bytes().iter() {
            let a = alphabet::char_value(Self::ALPHABET, *c);
            let a = if a < 0 {
                // The character encountered is not valid for our alphabet, so the input string is
                // not valid
                return false;
            } else {
                // If we are about to roll off a supplementary check character, that means it was
                // in the Payload portion of the input, and so the input is invalid.
                if check_chars[0] == supplementary_char {
                    return false;
                }
                for i in 1..check_length {
                    check_chars[i - 1] = check_chars[i]
                }
                check_chars[check_length - 1] = a;
                a as usize
            };

            if sum > max_sum {
                sum %= modulus
            }
            sum = (sum * radix) + a;
        }

        sum % modulus == 1
    }

    /// Compute the check digit for a payload string. If characters outside the ALPHABET are
    /// encountered, returns None immediately.
    fn compute(string: &str) -> Option<u16> {
        const MAX_CHECK_LENGTH: usize = 2;
        let check_length = Self::CHECK_LENGTH as usize;
        // We remember the last one or two char values so we can detect if we ever roll off a
        // _Supplementary Check Character_ ('X' for MOD 11-2 or '*' for MOD 32-2).
        let mut check_chars: [i8; MAX_CHECK_LENGTH] = [-1i8; MAX_CHECK_LENGTH];
        let supplementary_char: i8 = alphabet::supplementary_char_value(Self::ALPHABET);

        let modulus: usize = Self::MODULUS as usize;
        let radix: usize = Self::RADIX as usize;

        let mcv: usize = alphabet::max_char_value(Self::ALPHABET) as usize;
        let max_sum: usize = (usize::MAX - mcv) / radix;

        let mut sum: usize = 0;

        for c in string.as_bytes().iter() {
            let a = alphabet::char_value(Self::ALPHABET, *c);
            let a = if a < 0 {
                // The character encountered is not valid for our alphabet, so the input string is
                // not valid
                return None;
            } else {
                // If we are about to roll off a supplementary check character, that means it was
                // in the Payload portion of the input, and so the input is invalid.
                if check_chars[0] == supplementary_char {
                    return None;
                }
                for i in 1..check_length {
                    check_chars[i - 1] = check_chars[i]
                }
                check_chars[check_length - 1] = a;
                a as usize
            };

            if sum > max_sum {
                sum %= modulus
            }
            sum = (sum * radix) + a;
            println!("sum = {}", sum);
        }

        // Act as if we had two zeros provided for the check digit positions.
        for _i in 0..check_length {
            // If we are about to roll off a supplementary check character, that means it was
            // in the Payload portion of the input, and so the input is invalid.
            if check_chars[0] == supplementary_char {
                return None;
            }
            for i in 1..check_length {
                check_chars[i - 1] = check_chars[i]
            }
            check_chars[check_length - 1] = 0;
            if sum > max_sum {
                sum %= modulus
            }
            sum *= radix;
            println!("(*)sum = {}", sum);
        }

        println!("modulus: {}, sum: {}", modulus, sum);

        let value = ((modulus + 1) - (sum % modulus)) % modulus;

        Some(value as u16)
    }
}

pub trait HybridCheckCharacterSystem {
    /// In The Standard, Section 5.4.2, Table 3 "Single digit designations" specifies official names
    /// to be used to identify the different Check character systems it defines.
    const NAME: &'static str;

    /// In The Standard, Section 5.4.2, Table 3 "Single digit designations" specifies numbers to
    /// be used to identify the different Check character systems it defines, should the need arise.
    /// Implementations of this trait must provide a Standard-compliant designation. Note the value
    /// zero is reserved for "No check character or non-standard system".
    const DESIGNATION: u8;

    const MODULUS: u8;

    const ALPHABET: Alphabet;

    /// Check that the input string, which must already have the check digit(s) appended, satisfies
    /// the check. If characters outside the ALPHABET are encountered, returns false immediately.
    fn validate(string: &str) -> bool {
        let modulus: usize = Self::MODULUS as usize;

        let mut p: usize = modulus;
        let mut s: usize = 0;
        let mut first_char: bool = true;

        for c in string.as_bytes().iter() {
            let a = alphabet::char_value(Self::ALPHABET, *c);
            let a = if a < 0 {
                // The character encountered is not valid for our alphabet, so the input string is
                // not valid
                return false;
            } else {
                a as usize
            };

            if first_char {
                first_char = false;
            } else {
                p = (s * 2) % (modulus + 1);
            }

            s = p + a;
            s %= modulus;
            s = if s == 0 { modulus } else { s };
        }

        s % modulus == 1
    }

    /// Compute the check digit for a payload string. If characters outside the ALPHABET are
    /// encountered, returns None immediately.
    fn compute(string: &str) -> Option<u8> {
        let modulus: usize = Self::MODULUS as usize;

        let mut p: usize = modulus;
        #[allow(unused_assignments)]
        let mut s: usize = 0;

        for c in string.as_bytes().iter() {
            let a = alphabet::char_value(Self::ALPHABET, *c);
            let a = if a < 0 {
                // The character encountered is not valid for our alphabet, so the input string is
                // not valid
                return None;
            } else {
                a as usize
            };

            s = p + a;
            s %= modulus;
            s = if s == 0 { modulus } else { s };
            p = (s * 2) % (modulus + 1);
        }

        Some(((modulus + 1 - p) % modulus) as u8)
    }
}

// The _Pure_ _Check Character Systems_ defined in The Standard.

mod mod_11_2;
pub use mod_11_2::Mod11_2;

mod mod_1271_36;
pub use mod_1271_36::Mod1271_36;

mod mod_37_2;
pub use mod_37_2::Mod37_2;

mod mod_661_26;
pub use mod_661_26::Mod661_26;

mod mod_97_10;
pub use mod_97_10::Mod97_10;

// The _Hybrid_ _Check Character Systems_ defined in The Standard.

mod mod_11_10;
pub use mod_11_10::Mod11_10;

mod mod_27_26;
pub use mod_27_26::Mod27_26;

mod mod_37_36;
pub use mod_37_36::Mod37_36;
