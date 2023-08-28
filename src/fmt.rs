// espera::fmt
//
//! Formatting.
//

#[cfg(feature = "alloc")]
use alloc::{format, string::String};

/// Returns the time code as `HH:MM:SS:MIL`.
#[cfg(any(feature = "std", all(feature = "alloc", feature = "libm")))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", all(feature = "alloc", feature = "libm"))))
)]
pub fn timecode_f64(seconds: f64) -> String {
    #[cfg(feature = "std")]
    let ms = (seconds.fract() * 1000.) as u64;
    #[cfg(all(not(feature = "std"), feature = "libm"))]
    let ms = (libm::modf(seconds).0 * 1000.) as u64;

    #[cfg(feature = "std")]
    let mut ts = seconds.trunc() as u64;
    #[cfg(all(not(feature = "std"), feature = "libm"))]
    let mut ts = libm::trunc(seconds) as u64;

    let h = ts / 3600;
    ts %= 3600;
    let m = ts / 60;
    let s = ts % 60;

    // IMPROVE: use itoa / ftoa for speed
    // let mut s = String::with_capacity(11);
    // let mut s = String::new();
    // s.push_str(itoa::Buffer::new().format(128u64));
    // s

    if h > 0 {
        // 12 chars
        format!["{h:02}:{m:02}:{s:02}.{ms:03}"]
    } else {
        // 9 chars
        format!["{m:02}:{s:02}.{ms:03}"]
    }
}

/// Returns the time code, up to seconds, as `1s 012ms 012µs 012345ns`.
// THINK: sub-second
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub fn timecode_ns_u64(ns: u64) -> String {
    let (us, ns_rem) = (ns / 1000, ns % 1000);
    let (ms, us_rem) = (us / 1000, us % 1000);
    let (s, ms_rem) = (ms / 1000, ms % 1000);

    if s > 0 {
        format!["{s}s {ms_rem:03}ms {us_rem:03}µs {ns_rem:06}ns"]
    } else if ms > 0 {
        format!["{ms_rem}ms {us_rem:03}µs {ns_rem:06}ns"]
    } else if us > 0 {
        format!["{us_rem}µs {ns_rem:06}ns"]
    } else {
        format!["{ns_rem:06}ns"]
    }
}
