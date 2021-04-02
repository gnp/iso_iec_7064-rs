//! # iso_iec_7064::hybrid
//!
//! A trait to help implement the _Hybrid Check Character Systems_ appearing in The Standard.

use crate::system::System;

/// Parameters shared by all _Hybrid Check Character Systems_
pub trait HybridSystem: System {
    const MODULUS: u8;

    fn modulus() -> u8 {
        Self::MODULUS
    }

    /// Validate that the input digit values, which must already have the check digit(s) appended,
    /// satisfy the check. If a digit value outside those allowed by the the ALPHABET is
    /// encountered, returns false immediately.
    fn validate_digit_values_iter<I>(it: I) -> bool
    where
        I: IntoIterator<Item = u8>,
    {
        let modulus: usize = Self::MODULUS as usize;
        let max_digit_value: u8 = Self::ALPHABET.max_digit_value();

        let mut p: usize = modulus;
        let mut s: usize = 0;
        let mut first_char: bool = true;

        for v in it.into_iter() {
            if v > max_digit_value {
                return false;
            }

            if first_char {
                first_char = false;
            } else {
                p = (s * 2) % (modulus + 1);
            }

            s = p + (v as usize);
            s %= modulus;
            s = if s == 0 { modulus } else { s };
        }

        s % modulus == 1
    }

    /// Validate that the input ASCII bytes, which must already have the check digit(s) appended,
    /// satisfy the check. If an ASCII byte outside the ALPHABET is encountered, returns false
    /// immediately.
    fn validate_ascii_bytes_iter<I>(it: I) -> bool
    where
        I: IntoIterator<Item = u8>,
    {
        let it = it.into_iter().map(|c| {
            match Self::ALPHABET.char_value(c) {
                // The character encountered is not valid for our alphabet, so the input string is
                // not valid. Return an illegal digit value to force compute_digit_values_iter() to
                // return None
                a if a < 0 => u8::MAX,
                a => a as u8,
            }
        });

        Self::validate_digit_values_iter(it)
    }

    /// Validate that the input string, which must already have the check digit(s) appended,
    /// satisfies the check. If a character outside the ALPHABET is encountered, returns false
    /// immediately.
    fn validate_string(string: &str) -> bool {
        let it = string.as_bytes().iter().copied();
        Self::validate_ascii_bytes_iter(it)
    }

    /// Compute the checksum for an iterator of payload digit values (for example, values in the
    /// range 0 to 9 inclusive for `Alphabet::Numeric`). If a digit value outside those allowed by
    /// the ALPHABET is encountered, returns None immediately.
    fn checksum_digit_values_iter<I>(it: I) -> Option<u8>
    where
        I: IntoIterator<Item = u8>,
    {
        let modulus: usize = Self::MODULUS as usize;
        let max_digit_value: u8 = Self::ALPHABET.max_digit_value();

        let mut p: usize = modulus;

        for v in it.into_iter() {
            if v > max_digit_value {
                return None;
            }

            let mut s: usize = p + (v as usize);
            s %= modulus;
            s = if s == 0 { modulus } else { s };
            p = (s * 2) % (modulus + 1);
        }

        Some(((modulus + 1 - p) % modulus) as u8)
    }

    /// Compute the checksum for an iterator of payload ASCII bytes. If an ASCII byte outside the
    /// ALPHABET is encountered, returns None immediately.
    fn checksum_ascii_bytes_iter<I>(it: I) -> Option<u8>
    where
        I: IntoIterator<Item = u8>,
    {
        let it = it.into_iter().map(|c| {
            match Self::ALPHABET.char_value(c) {
                // The character encountered is not valid for our alphabet, so the input string is
                // not valid. Return an illegal digit value to force compute_digit_values_iter() to
                // return None
                a if a < 0 => u8::MAX,
                a => a as u8,
            }
        });

        Self::checksum_digit_values_iter(it)
    }

    /// Compute the check digit for a payload string. If a character outside the ALPHABET is
    /// encountered, returns None immediately.
    fn checksum_string(string: &str) -> Option<u8> {
        let it = string.as_bytes().iter().copied();
        Self::checksum_ascii_bytes_iter(it)
    }
}
