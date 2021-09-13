//! # iso_iec_7064::pure
//!
//! A trait to help implement the _Pure Check Character Systems_ appearing in The Standard.

use crate::system::System;

/// This is the state that will change with each iteration. Constant generic parameter CHECK_LENGTH
/// must be non-zero. The type of CHECK_LENGTH is usize instead of u8 because even though it in
/// practice only ever contains value 1 or 2, it is used in the size of an array, requiring it to be
/// of type usize.
struct State<
    const CHECK_LENGTH: usize,
    const MAX_DIGIT_VALUE: u8,
    const MODULUS: usize,
    const RADIX: usize,
> {
    /// We maintain the count so we can fail if there isn't at least one payload character.
    count: usize,
    /// The work-in-progress checksum.
    sum: usize,
    /// We remember the last one or two char values so we can detect if we ever roll off a
    /// _Supplementary Check Character_ ('X' for MOD 11-2 or '*' for MOD 32-2).
    check_char_values: [u8; CHECK_LENGTH],
}

impl<
        const CHECK_LENGTH: usize,
        const MAX_DIGIT_VALUE: u8,
        const MODULUS: usize,
        const RADIX: usize,
    > State<CHECK_LENGTH, MAX_DIGIT_VALUE, MODULUS, RADIX>
{
    const MAX_SUM: usize = (usize::MAX - (MAX_DIGIT_VALUE as usize)) / RADIX;

    /// The supplementary check character for the alphabet, or `-1` if there
    /// isn't one. These are used during validation to detect illegal characters in the payload
    /// portion
    /// of the input string.
    const SUPPLEMENTARY_CHAR_VALUE: i8 = match MAX_DIGIT_VALUE {
        10 => 10,
        36 => 36,
        _ => -1,
    };

    fn new() -> State<CHECK_LENGTH, MAX_DIGIT_VALUE, MODULUS, RADIX> {
        State {
            count: 0,
            sum: 0,
            check_char_values: [0; CHECK_LENGTH],
        }
    }

    /// Returns true if it successfully processed the digit value, false otherwise (for example, if
    /// the value was out of range).
    fn process_digit_value(&mut self, v: u8) -> bool {
        if v > MAX_DIGIT_VALUE {
            return false;
        }

        self.count += 1;

        // If our alphabet has a supplementary character, then we need to be sure we are not
        // about to roll off a value corresponding to the supplementary character, because if we
        // are, that means it was in the Payload portion of the input, making the input invalid.
        match Self::SUPPLEMENTARY_CHAR_VALUE {
            -1 => (),
            n => {
                if self.check_char_values[0] == (n as u8) {
                    return false;
                }
                for i in 1..CHECK_LENGTH {
                    self.check_char_values[i - 1] = self.check_char_values[i]
                }
                self.check_char_values[CHECK_LENGTH - 1] = v;
            }
        }

        // If the sum is great enough we cannot guarantee to not overflow, reduce it before
        // performing the next multiply-add step.
        if self.sum > Self::MAX_SUM {
            self.sum %= MODULUS
        }

        self.sum = (self.sum * RADIX) + (v as usize);

        true
    }
}

/// Type for implementing all _Pure Check Character Systems_
pub struct PureSystem<
    const CHECK_LENGTH: usize,
    const MAX_DIGIT_VALUE: u8,
    const MODULUS: usize,
    const RADIX: usize,
> {
    pub(crate) name: &'static str,
    pub(crate) designation: u8,
}

impl<
        const CHECK_LENGTH: usize,
        const MAX_DIGIT_VALUE: u8,
        const MODULUS: usize,
        const RADIX: usize,
    > PureSystem<CHECK_LENGTH, MAX_DIGIT_VALUE, MODULUS, RADIX>
{
}

impl<
        const CHECK_LENGTH: usize,
        const MAX_DIGIT_VALUE: u8,
        const MODULUS: usize,
        const RADIX: usize,
    > System<MAX_DIGIT_VALUE> for PureSystem<CHECK_LENGTH, MAX_DIGIT_VALUE, MODULUS, RADIX>
{
    fn name(&self) -> &'static str {
        self.name
    }

    fn designation(&self) -> u8 {
        self.designation
    }

    // fn alphabet(&self) -> Alphabet<MAX_DIGIT_VALUE> {
    //     Self::ALPHABET
    // }

    fn check_length(&self) -> u8 {
        CHECK_LENGTH as u8
    }

    /// Validate that the input digit values, which must already have the check digit(s) appended,
    /// satisfy the check. If a digit value outside those allowed by the the ALPHABET is
    /// encountered, returns false immediately.
    fn validate_digit_values_iter<I>(&self, it: I) -> bool
    where
        I: IntoIterator<Item = u8>,
    {
        let mut state: State<CHECK_LENGTH, MAX_DIGIT_VALUE, MODULUS, RADIX> = State::new();

        for v in it.into_iter() {
            if !state.process_digit_value(v) {
                return false;
            }
        }

        // If we have processed fewer than `check_length` + 1 items, then the input cannot be valid
        // because it has no Payload.
        if state.count < (CHECK_LENGTH + 1) {
            return false;
        }

        state.sum % MODULUS == 1
    }

    /// Compute the checksum for an iterator of payload digit values (for example, values in the
    /// range 0 to 9 inclusive for `Alphabet::Numeric`). If a digit value outside those allowed by
    /// the ALPHABET is encountered, returns None immediately.
    fn checksum_digit_values_iter<I>(&self, it: I) -> Option<u16>
    where
        I: IntoIterator<Item = u8>,
    {
        let mut state: State<CHECK_LENGTH, MAX_DIGIT_VALUE, MODULUS, RADIX> = State::new();

        for v in it.into_iter() {
            if !state.process_digit_value(v) {
                return None;
            }
        }

        // Act as if we had zero(s) provided for the check digit position(s).
        for _ in 0..CHECK_LENGTH {
            if !state.process_digit_value(0) {
                return None;
            }
        }

        // If we have processed no items, then the input cannot be valid because it has no Payload.
        // NOTE: We have to check this after all the calls to process_digit_value() so we can get
        // back ownership of the state.
        if state.count < (CHECK_LENGTH + 1) {
            return None;
        }

        let value = ((MODULUS + 1) - (state.sum % MODULUS)) % MODULUS;

        Some(value as u16)
    }
}
