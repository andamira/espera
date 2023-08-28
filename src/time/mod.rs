// espera::time
//
//! Time related types.
//

// mod duration;
mod unix;

pub use unix::{UnixTime, UnixTime32};

/// (re-exported from the [`time`] crate).
pub use time::Duration;

/// (re-exported from the [`time`] crate).
#[cfg(feature = "std")]
pub use time::Instant;
