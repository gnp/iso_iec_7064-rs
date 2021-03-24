pub enum Alphabet {
    /// Just the digits '0' through '9' for both the input string and the check digit(s).
    Numeric,
    /// The digits '0' through '9' for the input string, and the same set plus the supplementary
    /// check character 'X' for the check digit.
    NumericWithX,
    /// Just the letters 'A' through 'Z' for both the input string and the check digit(s).
    Alphabetic,
    /// The digits '0' through '9' and the letters 'A' through 'Z'.
    Alphanumeric,
    /// The digits '0' through '9' and the letters 'A' through 'Z' for the input string, and the
    /// same set plus the supplementary check character '*' for the check digit.
    AlphanumericWithAsterisk,
}

fn max_char_value(alphabet: Alphabet) -> i8 {
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
fn char_value(alphabet: Alphabet, c: u8) -> i8 {
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

/// Returns the value of the supplementary check character for the alphabet, or i8::MIN if there
/// isn't one. These are used during validation to detect illegal characters in the payload portion
/// of the input string. We use i8::MIN for the "not applicable" value because it compares not equal
/// to any actual character value *and* it compares not-equal to -1, which is used internally in the
/// validation function.
fn supplementary_char_value(alphabet: Alphabet) -> i8 {
    match alphabet {
        Alphabet::Numeric => i8::MIN,
        Alphabet::NumericWithX => 10,
        Alphabet::Alphabetic => i8::MIN,
        Alphabet::Alphanumeric => i8::MIN,
        Alphabet::AlphanumericWithAsterisk => 36,
    }
}

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
        let supplementary_char: i8 = supplementary_char_value(Self::ALPHABET);

        let modulus: usize = Self::MODULUS as usize;
        let radix: usize = Self::RADIX as usize;

        let mcv: usize = max_char_value(Self::ALPHABET) as usize;
        let max_sum: usize = (usize::MAX - mcv) / radix;

        let mut sum: usize = 0;

        for c in string.as_bytes().iter() {
            let a = char_value(Self::ALPHABET, *c);
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
        let supplementary_char: i8 = supplementary_char_value(Self::ALPHABET);

        let modulus: usize = Self::MODULUS as usize;
        let radix: usize = Self::RADIX as usize;

        let mcv: usize = max_char_value(Self::ALPHABET) as usize;
        let max_sum: usize = (usize::MAX - mcv) / radix;

        let mut sum: usize = 0;

        for c in string.as_bytes().iter() {
            let a = char_value(Self::ALPHABET, *c);
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
            let a = char_value(Self::ALPHABET, *c);
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
            let a = char_value(Self::ALPHABET, *c);
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

mod mod_11_2;
mod mod_1271_36;
mod mod_37_2;
mod mod_661_26;
mod mod_97_10;

mod mod_11_10;
mod mod_27_26;
mod mod_37_36;
