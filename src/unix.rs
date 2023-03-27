// espera::unix
//
// # DOCS
// - https://en.wikipedia.org/wiki/Unix_time
// - https://doc.rust-lang.org/std/time/struct.SystemTime.html
// - https://www.gnu.org/software/libc/manual/html_node/Getting-the-Time.html
// - https://www.gnu.org/software/libc/manual/html_node/Time-Functions-Example.html
//!
//

use core::fmt;

/// 64-bit Unix time, supports negative values.
///
/// Stores number of seconds since the Unix Epoch (`1970-01-01 00:00:00 UTC`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnixTime64 {
    pub seconds: i64,
}

/// 32-bit Unix time, supports only non-negative values.
///
/// Stores number of seconds since the Unix Epoch (`1970-01-01 00:00:00 UTC`).
///
/// It can represent time from 1970-01-01_00:00:00 to 2106-02-07_06:28:15.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnixTime32 {
    pub seconds: u32,
}

// A leap year occurs every four years to help synchronize the calendar year
// with the solar year or the length of time it takes the Earth to complete
// its orbit around the Sun, which is about 365.25 days. A year is
// considered a leap year if it is divisible by 4 but not by 100, or if it
// is divisible by 400.
#[inline]
const fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

impl UnixTime64 {
    /// Returns a new `UnixTime64` from the given amount of seconds.
    ///
    /// # Examples
    /// ```
    /// use espera::UnixTime64;
    ///
    /// assert_eq!["1970-01-01_00:00:01", UnixTime64::new(1).to_string()];
    /// assert_eq!["1969-12-31_23:59:59", UnixTime64::new(-1).to_string()];
    /// assert_eq!["2038-01-19_03:14:07", UnixTime64::new(i32::MAX as i64).to_string()];
    /// assert_eq!["2106-02-07_06:28:15", UnixTime64::new(u32::MAX as i64).to_string()];
    /// assert_eq!["1833-11-24_17:31:45", UnixTime64::new(u32::MAX as i64 * -1).to_string()];
    /// ```
    pub fn new(seconds: i64) -> Self {
        Self { seconds }
    }

    /// Returns a new `UnixTime64` anchored to the current second.
    #[cfg(any(
        feature = "std",
        all(not(feature = "std"), not(feature = "safe"), feature = "libc")
    ))]
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(any(
            feature = "std",
            all(feature = "no-std", feature = "non-safe", feature = "libc")
        )))
    )]
    pub fn now() -> Self {
        Self {
            seconds: Self::unix_time_64(),
        }
    }

    /// Returns an `UnixTime64` decomposed in `(years, months, days, hours, minutes, seconds)`.
    ///
    /// # Examples
    /// ```
    /// use espera::UnixTime64;
    ///
    /// assert_eq![(1970, 1, 1, 0, 0, 1), UnixTime64::new(1).to_ymdhms()];
    /// assert_eq![(1969, 12, 31, 23, 59, 59), UnixTime64::new(-1).to_ymdhms()];
    /// ```
    pub const fn to_ymdhms(&self) -> (i32, u32, u32, u32, u32, u32) {
        let seconds_per_minute = 60;
        let minutes_per_hour = 60;
        let hours_per_day = 24;
        let days_per_year = 365;

        let mut seconds_left = self.seconds.abs();
        let mut year = if self.seconds >= 0 { 1970 } else { 1969 };

        while seconds_left
            >= (hours_per_day * minutes_per_hour * seconds_per_minute * days_per_year) as i64
        {
            let leap = is_leap_year(year);
            let days_in_year = if leap { 366 } else { 365 };
            seconds_left -=
                (hours_per_day * minutes_per_hour * seconds_per_minute * days_in_year) as i64;

            if self.seconds >= 0 {
                year += 1;
            } else {
                year -= 1;
            }
        }

        let mut month = 1;
        let month_lengths = if is_leap_year(year) {
            [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        } else {
            [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        };

        while seconds_left
            >= (hours_per_day
                * minutes_per_hour
                * seconds_per_minute
                * month_lengths[month as usize - 1]) as i64
        {
            seconds_left -= (hours_per_day
                * minutes_per_hour
                * seconds_per_minute
                * month_lengths[month as usize - 1]) as i64;
            month += 1;
        }

        let day =
            (seconds_left / (hours_per_day * minutes_per_hour * seconds_per_minute) as i64) + 1;
        seconds_left %= (hours_per_day * minutes_per_hour * seconds_per_minute) as i64;

        let hour = seconds_left / (minutes_per_hour * seconds_per_minute) as i64;
        seconds_left %= (minutes_per_hour * seconds_per_minute) as i64;

        let minute = seconds_left / seconds_per_minute as i64;
        let second = seconds_left % seconds_per_minute as i64;

        if self.seconds >= 0 {
            (
                year,
                month,
                day as u32,
                hour as u32,
                minute as u32,
                second as u32,
            )
        } else {
            (
                year,
                13 - month,
                month_lengths[12 - month as usize] - day as u32 + 1,
                23 - hour as u32,
                59 - minute as u32,
                60 - second as u32,
            )
        }
    }
}

// private functions
impl UnixTime64 {
    // Returns the number of seconds since `1970-01-01 00:00:00 UTC`.
    #[cfg(feature = "std")]
    fn unix_time_64() -> i64 {
        use std::time::SystemTime;
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .min(i64::MAX as u64) as i64
    }

    // Returns the number of seconds since 1970-01-01 00:00:00 UTC.
    //
    // Because of `u32` this will only work until `06:28:15 UTC on 07 February 2016`.
    #[cfg(all(not(feature = "std"), not(feature = "safe"), feature = "libc"))]
    fn unix_time_64() -> i64 {
        // https://docs.rs/libc/latest/libc/fn.time.html
        #[allow(clippy::unnecessary_cast)] // could be i32 in other platforms
        unsafe {
            libc::time(core::ptr::null_mut()) as i64
        }
    }
}

impl UnixTime32 {
    /// Returns a new `UnixTime32` from the given amount of seconds.
    ///
    /// # Examples
    /// ```
    /// use espera::UnixTime32;
    ///
    /// assert_eq!["1970-01-01_00:00:00", UnixTime32::new(0).to_string()];
    /// assert_eq!["2106-02-07_06:28:15", UnixTime32::new(u32::MAX).to_string()];
    /// ```
    pub fn new(seconds: u32) -> Self {
        Self { seconds }
    }

    /// Returns a new `UnixTime32` anchored to the current second.
    #[cfg(any(
        feature = "std",
        all(not(feature = "std"), not(feature = "safe"), feature = "libc")
    ))]
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(any(
            feature = "std",
            all(feature = "no-std", feature = "non-safe", feature = "libc")
        )))
    )]
    pub fn now() -> Self {
        Self {
            seconds: Self::unix_time_32(),
        }
    }

    /// Returns an `UnixTime32` decomposed in `(years, months, days, hours, minutes, seconds)`.
    ///
    /// # Examples
    /// ```
    /// use espera::UnixTime32;
    ///
    /// assert_eq![(1970, 1, 1, 0, 0, 1), UnixTime32::new(1).to_ymdhms()];
    /// assert_eq![(2038, 1, 19, 3, 14, 7), UnixTime32::new(i32::MAX as u32).to_ymdhms()];
    /// ```
    pub const fn to_ymdhms(&self) -> (u16, u32, u32, u32, u32, u32) {
        let seconds_per_minute = 60;
        let minutes_per_hour = 60;
        let hours_per_day = 24;
        let days_per_year = 365;

        let mut seconds_left = self.seconds;
        let mut year = 1970;

        while seconds_left
            >= (hours_per_day * minutes_per_hour * seconds_per_minute * days_per_year)
        {
            year += 1;
            let leap = is_leap_year(year);
            let days_in_year = if leap { 366 } else { 365 };
            seconds_left -= hours_per_day * minutes_per_hour * seconds_per_minute * days_in_year;
        }

        let mut month = 1;
        let month_lengths = if is_leap_year(year) {
            [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        } else {
            [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        };

        while seconds_left
            >= (hours_per_day
                * minutes_per_hour
                * seconds_per_minute
                * month_lengths[month as usize - 1])
        {
            seconds_left -= hours_per_day
                * minutes_per_hour
                * seconds_per_minute
                * month_lengths[month as usize - 1];
            month += 1;
        }

        let day = (seconds_left / (hours_per_day * minutes_per_hour * seconds_per_minute)) + 1;
        seconds_left %= hours_per_day * minutes_per_hour * seconds_per_minute;

        let hour = seconds_left / (minutes_per_hour * seconds_per_minute);
        seconds_left %= minutes_per_hour * seconds_per_minute;

        let minute = seconds_left / seconds_per_minute;
        let second = seconds_left % seconds_per_minute;

        (year as u16, month, day, hour, minute, second)
    }
}

// private functions
impl UnixTime32 {
    // Returns the number of seconds since `1970-01-01 00:00:00 UTC`.
    //
    // Because of `u32` this will only work until `06:28:15 UTC on 07 February 2016`.
    #[cfg(feature = "std")]
    fn unix_time_32() -> u32 {
        use std::time::SystemTime;
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .min(u32::MAX as u64) as u32
    }

    // Returns the number of seconds since 1970-01-01 00:00:00 UTC.
    //
    // Because of `u32` this will only work until `06:28:15 UTC on 07 February 2016`.
    #[cfg(all(not(feature = "std"), not(feature = "safe"), feature = "libc"))]
    fn unix_time_32() -> u32 {
        unsafe { libc::time(core::ptr::null_mut()).clamp(0, u32::MAX as i64) as u32 }
    }
}

impl fmt::Display for UnixTime64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (y, m, d, h, min, s) = self.to_ymdhms();
        write![f, "{y:04}-{m:02}-{d:02}_{h:02}:{min:02}:{s:02}"]
    }
}

impl fmt::Display for UnixTime32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (y, m, d, h, min, s) = self.to_ymdhms();
        write![f, "{y:04}-{m:02}-{d:02}_{h:02}:{min:02}:{s:02}"]
    }
}