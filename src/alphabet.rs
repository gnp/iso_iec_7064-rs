#![warn(missing_docs)]
//! # iso_iec_7064::alphabet
//!
//! A type to represent the five _Alphabets_ appearing in The Standard.

/// There are five _Alphabets_ used by the _Check Character Systems_ defined in The Standard. While
/// The Standard does not specify a character encoding, we will b using the ASCII subset of Unicode.
/// In this encoding, every character of every _Alphabet_ corresponds to a single byte of an input
/// string, allowing us to implement validation and computation operations on raw byte sequences,
/// without any overhead from handling UTF-8.
pub enum Alphabet {
    /// Just the digits '0' through '9' for both the input string and the _Check Character(s)_.
    Numeric,
    /// The digits '0' through '9' for the input string, and the same set plus the _Supplementary
    /// Check Character_ 'X' for the _Check Character_.
    NumericWithX,
    /// Just the letters 'A' through 'Z' for both the input string and the _Check Character(s)_.
    Alphabetic,
    /// The digits '0' through '9' and the letters 'A' through 'Z' for both the input string and the
    /// _Check Character(s)_.
    Alphanumeric,
    /// The digits '0' through '9' and the letters 'A' through 'Z' for the input string, and the
    /// same set plus the _Supplementary Check Character_ '*' for the _Check Character_.
    AlphanumericWithAsterisk,
}

/// The maximum numeric value assigned to any character of the _Alphabet_.
pub fn max_char_value(alphabet: Alphabet) -> u8 {
    match alphabet {
        Alphabet::Numeric => 9,
        Alphabet::NumericWithX => 10,
        Alphabet::Alphabetic => 25,
        Alphabet::Alphanumeric => 35,
        Alphabet::AlphanumericWithAsterisk => 36,
    }
}

/// Returns the numeric value of the character, or -1 if it is not a valid character for the
/// alphabet.
pub fn char_value(alphabet: Alphabet, c: u8) -> i8 {
    match alphabet {
        Alphabet::Numeric => match c {
            b'0'..=b'9' => (c - b'0') as i8,
            _ => -1i8,
        },
        Alphabet::NumericWithX => match c {
            b'0'..=b'9' => (c - b'0') as i8,
            b'X' => 10i8,
            _ => -1i8,
        },
        Alphabet::Alphabetic => match c {
            b'A'..=b'Z' => (c - b'A') as i8,
            _ => -1i8,
        },
        Alphabet::Alphanumeric => match c {
            b'0'..=b'9' => (c - b'0') as i8,
            b'A'..=b'Z' => (c - b'A' + 10) as i8,
            _ => -1i8,
        },
        Alphabet::AlphanumericWithAsterisk => match c {
            b'0'..=b'9' => (c - b'0') as i8,
            b'A'..=b'Z' => (c - b'A' + 10) as i8,
            b'*' => 36i8,
            _ => -1i8,
        },
    }
}

/// Returns the value of the supplementary check character for the alphabet, or `i8::MIN` if there
/// isn't one. These are used during validation to detect illegal characters in the payload portion
/// of the input string. We use `i8::MIN` for the "not applicable" value because it compares not
/// equal to any actual character value *and* it compares not-equal to -1, which is used internally
/// in validation functions.
pub fn supplementary_char_value(alphabet: Alphabet) -> i8 {
    match alphabet {
        Alphabet::Numeric => i8::MIN,
        Alphabet::NumericWithX => 10,
        Alphabet::Alphabetic => i8::MIN,
        Alphabet::Alphanumeric => i8::MIN,
        Alphabet::AlphanumericWithAsterisk => 36,
    }
}
