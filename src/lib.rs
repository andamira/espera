// espera
//
//! Time management.
//

#![warn(clippy::all)]
#![allow(
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::needless_return,
    clippy::blanket_clippy_restriction_lints,
    clippy::pattern_type_mismatch
)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(not(feature = "std"), no_std)]

pub use time::{Duration, Instant};

mod sleeper;
pub use sleeper::Sleeper;

#[cfg(feature = "std")]
mod timecode;
#[cfg(feature = "std")]
pub use timecode::{timecode_f64, timecode_ns_u64};

mod macros;
