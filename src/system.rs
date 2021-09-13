//! # iso_iec_7064::system
//!
//! Trait implemented by all _Check Character Systems_, both pure and hybrid.

/// There are five _Alphabets_ used by the _Check Character Systems_ defined in The Standard. While
/// The Standard does not specify a character encoding, we will be using the ASCII subset of Unicode.
/// In this encoding, every character of every _Alphabet_ corresponds to a single byte of an input
/// string, allowing us to implement validation and computation operations on raw byte sequences,
/// without any overhead from handling UTF-8.
///
/// The five _Alphabets_ are distinguished by their maximum digit value, and the `char_value`
/// function takes the max digit value as a generic parameter.
///
/// Convert an input ASCII character into its corresponding numeric value, returning -1 if the
/// input value is out of range.
pub const fn char_value<const MAX_DIGIT_VALUE: u8>(c: u8) -> i8 {
    match MAX_DIGIT_VALUE {
        // aka "numeric"
        9 => match c {
            b'0'..=b'9' => (c - b'0') as i8,
            _ => -1i8,
        },
        // aka "numeric with 'x'"
        10 => match c {
            b'0'..=b'9' => (c - b'0') as i8,
            b'X' => 10i8,
            _ => -1i8,
        },
        // aka "alphabetic"
        25 => match c {
            b'A'..=b'Z' => (c - b'A') as i8,
            _ => -1i8,
        },
        // aka "alphanumeric"
        35 => match c {
            b'0'..=b'9' => (c - b'0') as i8,
            b'A'..=b'Z' => (c - b'A' + 10) as i8,
            _ => -1i8,
        },
        // aka "alphanumeric with '*'"
        36 => match c {
            b'0'..=b'9' => (c - b'0') as i8,
            b'A'..=b'Z' => (c - b'A' + 10) as i8,
            b'*' => 36i8,
            _ => -1i8,
        },
        _ => -1i8, // Compiler won't let us panic!() here
    }
}

/// Trait implemented by all _Check Character Systems_
pub trait System<const MAX_DIGIT_VALUE: u8> {
    /// In The Standard, Section 5.4.2, Table 3 "Single digit designations" specifies official names
    /// to be used to identify the different Check character systems it defines.
    fn name(&self) -> &'static str;

    /// In The Standard, Section 5.4.2, Table 3 "Single digit designations" specifies numbers to
    /// be used to identify the different Check character systems it defines, should the need arise.
    /// Implementations of this trait must provide a Standard-compliant designation. Note the value
    /// zero is reserved for "No check character or non-standard system".
    fn designation(&self) -> u8;

    /// The alphabet used in the Check character system, including the symbols allowed in the
    /// payload and any additional symbols that may be allowed in the check character(s).
    // fn alphabet(&self) -> &Alphabet;

    /// The number of check characters the Check character system produces.
    fn check_length(&self) -> u8;

    /// Validate that the input digit values, which must already have the check digit(s) appended,
    /// satisfy the check. If a digit value outside those allowed by the the ALPHABET is
    /// encountered, returns false immediately.
    fn validate_digit_values_iter<I>(&self, it: I) -> bool
    where
        I: IntoIterator<Item = u8>;

    /// Validate that the input ASCII bytes, which must already have the check digit(s) appended,
    /// satisfy the check. If an ASCII byte outside the ALPHABET is encountered, returns false
    /// immediately.
    fn validate_ascii_bytes_iter<I>(&self, it: I) -> bool
    where
        I: IntoIterator<Item = u8>,
    {
        let it = it.into_iter().map(|c| {
            match char_value::<MAX_DIGIT_VALUE>(c) {
                // The character encountered is not valid for our alphabet, so the input string is
                // not valid. Return an illegal digit value to force compute_digit_values_iter() to
                // return None
                a if a < 0 => u8::MAX,
                a => a as u8,
            }
        });

        self.validate_digit_values_iter(it)
    }

    /// Check that the input string, which must already have the check digit(s) appended, satisfies
    /// the check. If characters outside the ALPHABET are encountered, returns false immediately.
    fn validate_string(&self, string: &str) -> bool {
        let it = string.as_bytes().iter().copied();
        self.validate_ascii_bytes_iter(it)
    }

    /// Compute the checksum for an iterator of payload digit values (for example, values in the
    /// range 0 to 9 inclusive for `Alphabet::Numeric`). If a digit value outside those allowed by
    /// the ALPHABET is encountered, returns None immediately.
    fn checksum_digit_values_iter<I>(&self, it: I) -> Option<u16>
    where
        I: IntoIterator<Item = u8>;

    /// Compute the checksum for an iterator of payload ASCII bytes. If an ASCII byte outside the
    /// ALPHABET is encountered, returns None immediately.
    fn checksum_ascii_bytes_iter<I>(&self, it: I) -> Option<u16>
    where
        I: IntoIterator<Item = u8>,
    {
        let it = it.into_iter().map(|c| {
            match char_value::<MAX_DIGIT_VALUE>(c) {
                // The character encountered is not valid for our alphabet, so the input string is
                // not valid. Return an illegal digit value to force compute_digit_values_iter() to
                // return None
                a if a < 0 => u8::MAX,
                a => a as u8,
            }
        });

        self.checksum_digit_values_iter(it)
    }

    /// Compute the check digit for a payload string. If characters outside the ALPHABET are
    /// encountered, returns None immediately.
    fn checksum_string(&self, string: &str) -> Option<u16> {
        let it = string.as_bytes().iter().copied();
        self.checksum_ascii_bytes_iter(it)
    }
}
