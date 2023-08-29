// espera::lib
//
//! Time management.
//

// warnings
#![warn(clippy::all)]
#![allow(
    clippy::needless_return,
    clippy::module_inception,
    non_upper_case_globals
)]
// environment
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
#[cfg(feature = "alloc")]
extern crate alloc;

// safeguards
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
#[cfg(all(feature = "safe", feature = "unsafe"))]
compile_error!("You can't enable the `safe` and `unsafe` features at the same time.");
// deprecated
devela::deprecate_feature![old: "no-std", new: "no_std", since: "0.3.0"];

pub mod calendar;
pub mod error;
pub mod fmt;
pub mod time;

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub mod control;

/// All items are reexported here.
pub mod all {
    #[doc(inline)]
    pub use super::{
        calendar::{Month, Weekday},
        error::*,
        fmt::*,
        time::*,
    };

    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::control::*;
}
