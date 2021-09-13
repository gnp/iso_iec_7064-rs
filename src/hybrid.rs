//! # iso_iec_7064::hybrid
//!
//! A trait to help implement the _Hybrid Check Character Systems_ appearing in The Standard.

use crate::system::System;

/// Type for implementing all _Hybrid Check Character Systems_
pub struct HybridSystem<const MAX_DIGIT_VALUE: u8> {
    pub(crate) name: &'static str,
    pub(crate) designation: u8,
    pub(crate) modulus: usize,
}

impl<const MAX_DIGIT_VALUE: u8> System<MAX_DIGIT_VALUE> for HybridSystem<MAX_DIGIT_VALUE> {
    fn name(&self) -> &'static str {
        self.name
    }

    fn designation(&self) -> u8 {
        self.designation
    }

    fn check_length(&self) -> u8 {
        1
    }

    /// Validate that the input digit values, which must already have the check digit(s) appended,
    /// satisfy the check. If a digit value outside those allowed by the the ALPHABET is
    /// encountered, returns false immediately.
    fn validate_digit_values_iter<I>(&self, it: I) -> bool
    where
        I: IntoIterator<Item = u8>,
    {
        let mut p: usize = self.modulus;
        let mut s: usize = 0;
        let mut first_char: bool = true;

        for v in it.into_iter() {
            if v > MAX_DIGIT_VALUE {
                return false;
            }

            if first_char {
                first_char = false;
            } else {
                p = (s * 2) % (self.modulus + 1);
            }

            s = p + (v as usize);
            s %= self.modulus;
            s = if s == 0 { self.modulus } else { s };
        }

        s % self.modulus == 1
    }

    /// Compute the checksum for an iterator of payload digit values (for example, values in the
    /// range 0 to 9 inclusive for `Alphabet::Numeric`). If a digit value outside those allowed by
    /// the ALPHABET is encountered, returns None immediately.
    fn checksum_digit_values_iter<I>(&self, it: I) -> Option<u16>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut p: usize = self.modulus;

        for v in it.into_iter() {
            if v > MAX_DIGIT_VALUE {
                return None;
            }

            let mut s: usize = p + (v as usize);
            s %= self.modulus;
            s = if s == 0 { self.modulus } else { s };
            p = (s * 2) % (self.modulus + 1);
        }

        Some(((self.modulus + 1 - p) % self.modulus) as u16)
    }
}
