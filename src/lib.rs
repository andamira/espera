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

#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
#[cfg(all(feature = "safe", feature = "unsafe"))]
compile_error!("You can't enable the `safe` and `unsafe` features at the same time.");

devela::deprecate_feature![old: "no-std", new: "no_std", since: "0.3.0"];

#[cfg(feature = "alloc")]
extern crate alloc;

/// (re-exported from the [`time`] crate).
pub use time::Duration;

/// (re-exported from the [`time`] crate).
#[cfg(feature = "std")]
pub use time::Instant;

pub mod calendar;
pub mod error;
pub mod unix;

mod macros;
mod timecode;

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub mod looper;
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub mod rate;
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub mod sleeper;

/// Everything is directly available in here.
pub mod all {
    #[doc(inline)]
    pub use super::{
        calendar::{Month, Weekday},
        error::*,
        timecode::*,
        unix::{UnixTime, UnixTime32},
        Duration,
    };
    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::{
        looper::{LoopStatus, Looper},
        rate::{Rate, RateStats},
        sleeper::Sleeper,
        Instant,
    };
}
