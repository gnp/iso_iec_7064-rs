//! # iso_iec_7064::pure
//!
//! A trait to help implement the _Pure Check Character Systems_ appearing in The Standard.

use crate::system::System;

struct PureCheckCharacterSystemConfig {
    check_length: usize,
    modulus: usize,
    radix: usize,
    max_digit_value: u8,
    supplementary_char_value: Option<u8>,
    max_sum: usize,
}

impl PureCheckCharacterSystemConfig {
    fn new(
        check_length: usize,
        modulus: usize,
        radix: usize,
        max_digit_value: u8,
        supplementary_char_value: Option<u8>,
    ) -> PureCheckCharacterSystemConfig {
        let max_sum: usize = (usize::MAX - (max_digit_value as usize)) / radix;

        PureCheckCharacterSystemConfig {
            check_length,
            modulus,
            radix,
            max_digit_value,
            supplementary_char_value,
            max_sum,
        }
    }
}

const MAX_CHECK_LENGTH: usize = 2;

/// This is the state that will change with each iteration
struct PureCheckCharacterSystemState {
    /// We maintain the count so we can fail if there isn't at least one payload character.
    count: usize,
    /// The work-in-progress checksum.
    sum: usize,
    /// We remember the last one or two char values so we can detect if we ever roll off a
    /// _Supplementary Check Character_ ('X' for MOD 11-2 or '*' for MOD 32-2).
    check_char_values: [u8; MAX_CHECK_LENGTH],
}

impl PureCheckCharacterSystemState {
    fn new() -> PureCheckCharacterSystemState {
        PureCheckCharacterSystemState {
            count: 0,
            sum: 0,
            check_char_values: [0; MAX_CHECK_LENGTH],
        }
    }

    /// Returns true if it successfully processed the digit value, false otherwise (for example, if
    /// the value was out of range).
    fn process_digit_value(&mut self, config: &PureCheckCharacterSystemConfig, v: u8) -> bool {
        if v > config.max_digit_value {
            return false;
        }

        self.count += 1;

        // If our alphabet has a supplementary character, then we need to be sure we are not
        // about to roll off a value corresponding to the supplementary character, because if we
        // are, that means it was in the Payload portion of the input, making the input invalid.
        match config.supplementary_char_value {
            None => (),
            Some(n) => {
                if self.check_char_values[0] == n {
                    return false;
                }
                for i in 1..config.check_length {
                    self.check_char_values[i - 1] = self.check_char_values[i]
                }
                self.check_char_values[config.check_length - 1] = v;
            }
        }

        // If the sum is great enough we cannot guarantee to not overflow, reduce it before
        // performing the next multiply-add step.
        if self.sum > config.max_sum {
            self.sum %= config.modulus
        }

        self.sum = (self.sum * config.radix) + (v as usize);

        true
    }
}

/// Parameters shared by all _Pure Check Character Systems_
pub trait PureSystem: System {
    const MODULUS: u16;

    const RADIX: u8;

    fn modulus() -> u16 {
        Self::MODULUS
    }

    fn radix() -> u8 {
        Self::RADIX
    }

    /// Validate that the input digit values, which must already have the check digit(s) appended,
    /// satisfy the check. If a digit value outside those allowed by the the ALPHABET is
    /// encountered, returns false immediately.
    fn validate_digit_values_iter<I>(it: I) -> bool
    where
        I: IntoIterator<Item = u8>,
    {
        // const MAX_CHECK_LENGTH: usize = 2;
        let check_length = Self::CHECK_LENGTH as usize;
        // We remember the last one or two char values so we can detect if we ever roll off a
        // _Supplementary Check Character_ ('X' for MOD 11-2 or '*' for MOD 32-2).
        // let mut check_char_values: [u8; MAX_CHECK_LENGTH] = [0; MAX_CHECK_LENGTH];
        let supplementary_char_value: Option<u8> = Self::ALPHABET.supplementary_char_value();

        let modulus: usize = Self::MODULUS as usize;
        let radix: usize = Self::RADIX as usize;
        let max_digit_value: u8 = Self::ALPHABET.max_digit_value();

        let config = PureCheckCharacterSystemConfig::new(
            check_length,
            modulus,
            radix,
            max_digit_value,
            supplementary_char_value,
        );

        let mut state = PureCheckCharacterSystemState::new();

        for v in it.into_iter() {
            if !state.process_digit_value(&config, v) {
                return false;
            }
        }

        // If we have processed fewer than `check_length` + 1 items, then the input cannot be valid
        // because it has no Payload.
        if state.count < (config.check_length + 1) {
            return false;
        }

        state.sum % config.modulus == 1
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

    /// Check that the input string, which must already have the check digit(s) appended, satisfies
    /// the check. If characters outside the ALPHABET are encountered, returns false immediately.
    fn validate_string(string: &str) -> bool {
        let it = string.as_bytes().iter().copied();
        Self::validate_ascii_bytes_iter(it)
    }

    /// Compute the checksum for an iterator of payload digit values (for example, values in the
    /// range 0 to 9 inclusive for `Alphabet::Numeric`). If a digit value outside those allowed by
    /// the ALPHABET is encountered, returns None immediately.
    fn checksum_digit_values_iter<I>(it: I) -> Option<u16>
    where
        I: IntoIterator<Item = u8>,
    {
        let check_length = Self::CHECK_LENGTH as usize;
        let supplementary_char_value: Option<u8> = Self::ALPHABET.supplementary_char_value();

        let modulus: usize = Self::MODULUS as usize;
        let radix: usize = Self::RADIX as usize;
        let max_digit_value: u8 = Self::ALPHABET.max_digit_value();

        let config = PureCheckCharacterSystemConfig::new(
            check_length,
            modulus,
            radix,
            max_digit_value,
            supplementary_char_value,
        );

        let mut state = PureCheckCharacterSystemState::new();

        for v in it.into_iter() {
            if !state.process_digit_value(&config, v) {
                return None;
            }
        }

        // Act as if we had zero(s) provided for the check digit position(s).
        for _ in 0..check_length {
            if !state.process_digit_value(&config, 0) {
                return None;
            }
        }

        // If we have processed no items, then the input cannot be valid because it has no Payload.
        // NOTE: We have to check this after all the calls to process_digit_value() so we can get
        // back ownership of the state.
        if state.count < (check_length + 1) {
            return None;
        }

        let value = ((modulus + 1) - (state.sum % modulus)) % modulus;

        Some(value as u16)
    }

    /// Compute the checksum for an iterator of payload ASCII bytes. If an ASCII byte outside the
    /// ALPHABET is encountered, returns None immediately.
    fn checksum_ascii_bytes_iter<I>(it: I) -> Option<u16>
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

    /// Compute the check digit for a payload string. If characters outside the ALPHABET are
    /// encountered, returns None immediately.
    fn checksum_string(string: &str) -> Option<u16> {
        let it = string.as_bytes().iter().copied();
        Self::checksum_ascii_bytes_iter(it)
    }
}
