// espera::lib
//
//! Time management.
//

#![warn(clippy::all)]
#![allow(
    clippy::needless_return,
    clippy::module_inception,
    non_upper_case_globals
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

#[cfg(all(feature = "std", feature = "no-std"))]
compile_error!("You can't enable the `std` and `no-std` features at the same time.");

#[cfg(feature = "alloc")]
extern crate alloc;

/// (re-exported from the [`time`][::time] crate).
pub use ::time::Duration;

/// (re-exported from the [`time`][::time] crate).
#[cfg(feature = "std")]
pub use ::time::Instant;

pub mod calendar;
mod macros;
mod timecode;
mod unix;

#[doc(inline)]
pub use calendar::{Month, Weekday};
pub use timecode::*;
pub use unix::{UnixTime, UnixTime32};

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
mod sleeper;
#[cfg(feature = "std")]
pub use sleeper::Sleeper;
