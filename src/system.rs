use crate::alphabet::Alphabet;

/// Parameters shared by all _Check Character Systems_
pub trait System {
    /// In The Standard, Section 5.4.2, Table 3 "Single digit designations" specifies official names
    /// to be used to identify the different Check character systems it defines.
    const NAME: &'static str;

    /// In The Standard, Section 5.4.2, Table 3 "Single digit designations" specifies numbers to
    /// be used to identify the different Check character systems it defines, should the need arise.
    /// Implementations of this trait must provide a Standard-compliant designation. Note the value
    /// zero is reserved for "No check character or non-standard system".
    const DESIGNATION: u8;

    /// The alphabet used in the Check character system, including the symbols allowed in the
    /// payload and any additional symbols that may be allowed in the check character(s).
    const ALPHABET: Alphabet;

    /// The number of check characters the Check character system produces.
    const CHECK_LENGTH: u8;

    fn name() -> &'static str {
        Self::NAME
    }

    fn designation() -> u8 {
        Self::DESIGNATION
    }

    fn alphabet() -> Alphabet {
        Self::ALPHABET
    }

    fn check_length() -> u8 {
        Self::CHECK_LENGTH
    }
}
