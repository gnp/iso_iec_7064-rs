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

impl Alphabet {
    /// The maximum digit value assigned to any character of the _Alphabet_.
    pub const fn max_digit_value(&self) -> u8 {
        match self {
            Alphabet::Numeric => 9,
            Alphabet::NumericWithX => 10,
            Alphabet::Alphabetic => 25,
            Alphabet::Alphanumeric => 35,
            Alphabet::AlphanumericWithAsterisk => 36,
        }
    }

    /// Returns the numeric value of the character, or -1 if it is not a valid character for the
    /// alphabet.
    pub fn char_value(&self, c: u8) -> i8 {
        match self {
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

    /// Returns `Some(_)` for the supplementary check character for the alphabet, or `None` if there
    /// isn't one. These are used during validation to detect illegal characters in the payload portion
    /// of the input string.
    pub const fn supplementary_char_value(&self) -> Option<u8> {
        match self {
            Alphabet::Numeric => None,
            Alphabet::NumericWithX => Some(10),
            Alphabet::Alphabetic => None,
            Alphabet::Alphanumeric => None,
            Alphabet::AlphanumericWithAsterisk => Some(36),
        }
    }
}
