// espera::timecode
//
//!
//

/// Returns the time code as `HH:MM:SS:MIL`.
pub fn timecode_f64(seconds: f64) -> String {
    let ms = (seconds.fract() * 1000.) as u64;
    let mut ts = seconds.trunc() as u64;

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

/// Returns the time code up to seconds
// THINK: sub-second
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
