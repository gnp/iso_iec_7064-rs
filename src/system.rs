use crate::alphabet::Alphabet;

/// Trait implemented by all _Check Character Systems_
pub trait System {
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
    fn alphabet(&self) -> &Alphabet;

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
            match self.alphabet().char_value(c) {
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
            match self.alphabet().char_value(c) {
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
