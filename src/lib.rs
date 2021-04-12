//! # iso_iec_7064
//!
//! Conforming implementation of [ISO/IEC 7064:2003](https://www.iso.org/standard/31531.html)
//! "Information technology &mdash; Security techniques &mdash; Check character systems" (The
//! Standard).
//!
//! This crate implements all the _Check Character Systems_ specified in The Standard.
//!
//! Note that the `Mod97_10` system is used by [ISO 17442-1:2020](https://www.iso.org/standard/78829.html)
//! "Financial services — Legal entity identifier (LEI) — Part 1: Assignment", which is implemented
//! by [the `lei` crate](https://crates.io/crates/lei).

use crate::alphabet::Alphabet;

mod alphabet;
pub mod hybrid;
pub mod pure;

pub mod system;
pub use system::System;

// The _Pure_ _Check Character Systems_ defined in The Standard.

mod mod_11_2;
pub use mod_11_2::Mod11_2;

mod mod_1271_36;
pub use mod_1271_36::Mod1271_36;

mod mod_37_2;
pub use mod_37_2::Mod37_2;

mod mod_661_26;
pub use mod_661_26::Mod661_26;

mod mod_97_10;
pub use mod_97_10::Mod97_10;

// The _Hybrid_ _Check Character Systems_ defined in The Standard.

mod mod_11_10;
pub use mod_11_10::Mod11_10;

mod mod_27_26;
pub use mod_27_26::Mod27_26;

mod mod_37_36;
pub use mod_37_36::Mod37_36;
