// espera::unix
//
// # DOCS
// - https://en.wikipedia.org/wiki/Unix_time
// - https://doc.rust-lang.org/std/time/struct.SystemTime.html
// - https://www.gnu.org/software/libc/manual/html_node/Getting-the-Time.html
// - https://www.gnu.org/software/libc/manual/html_node/Time-Functions-Example.html
//
//! Unix time.
//

use crate::calendar::{is_leap_year, Month};
use core::{convert::TryFrom, fmt, num::TryFromIntError};

/// 64-bit Unix time, supporting negative values.
///
/// Stores number of seconds since the Unix Epoch (`1970-01-01 00:00:00 UTC`).
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnixTime {
    pub seconds: i64,
}

/// 32-bit Unix time, supporting only non-negative values.
///
/// Stores number of seconds since the Unix Epoch (`1970-01-01 00:00:00 UTC`).
///
/// It can represent time from `1970-01-01_00:00:00` to `2106-02-07_06:28:15`.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnixTime32 {
    pub seconds: u32,
}

impl UnixTime {
    /// Returns a new `UnixTime` from the given amount of seconds.
    ///
    /// # Examples
    /// ```
    /// use espera::all::UnixTime;
    ///
    /// assert_eq!["1970-01-01_00:00:01", UnixTime::new(1).to_string()];
    /// assert_eq!["1969-12-31_23:59:59", UnixTime::new(-1).to_string()];
    /// assert_eq!["2038-01-19_03:14:07", UnixTime::new(i32::MAX as i64).to_string()];
    /// assert_eq!["2106-02-07_06:28:15", UnixTime::new(u32::MAX as i64).to_string()];
    /// assert_eq!["1833-11-24_17:31:45", UnixTime::new(u32::MAX as i64 * -1).to_string()];
    /// ```
    pub fn new(seconds: i64) -> Self {
        Self { seconds }
    }

    /// Returns a new `UnixTime` anchored to the current second.
    #[cfg(any(
        feature = "std",
        all(not(feature = "std"), feature = "unsafe", feature = "libc")
    ))]
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(any(
            feature = "std",
            all(feature = "no_std", feature = "unsafe", feature = "libc")
        )))
    )]
    pub fn now() -> Self {
        Self {
            seconds: Self::unix_time_64(),
        }
    }

    /// Returns a `UnixTime` converted to `(year, month, day, hour, minute, second)`.
    ///
    /// # Examples
    /// ```
    /// use espera::all::UnixTime;
    ///
    /// assert_eq![(1970, 1, 1, 0, 0, 1), UnixTime::new(1).to_ymdhms()];
    /// assert_eq![(1969, 12, 31, 23, 59, 59), UnixTime::new(-1).to_ymdhms()];
    /// ```
    pub const fn to_ymdhms(&self) -> (i32, u8, u8, u8, u8, u8) {
        let seconds_per_minute: u32 = 60;
        let minutes_per_hour: u32 = 60;
        let hours_per_day: u32 = 24;
        let days_per_year: u32 = 365;

        let mut seconds_left = self.seconds.abs();
        let mut year = if self.seconds >= 0 { 1970 } else { 1969 };
        let mut leap = is_leap_year(year);

        while seconds_left
            >= (hours_per_day * minutes_per_hour * seconds_per_minute * days_per_year) as i64
        {
            leap = is_leap_year(year);
            let days_in_year = if leap { 366 } else { 365 };
            seconds_left -=
                (hours_per_day * minutes_per_hour * seconds_per_minute * days_in_year) as i64;

            if self.seconds >= 0 {
                year += 1;
            } else {
                year -= 1;
            }
        }

        let mut month = Month::January;
        while seconds_left
            >= (hours_per_day * minutes_per_hour * seconds_per_minute * month.len(leap) as u32)
                as i64
        {
            seconds_left -=
                (hours_per_day * minutes_per_hour * seconds_per_minute * month.len(leap) as u32)
                    as i64;
            month = month.next();
        }

        let day = (seconds_left / (hours_per_day * minutes_per_hour * seconds_per_minute) as i64)
            as u8
            + 1;
        seconds_left %= (hours_per_day * minutes_per_hour * seconds_per_minute) as i64;

        let hour = seconds_left / (minutes_per_hour * seconds_per_minute) as i64;
        seconds_left %= (minutes_per_hour * seconds_per_minute) as i64;

        let minute = seconds_left / seconds_per_minute as i64;
        let second = seconds_left % seconds_per_minute as i64;

        if self.seconds >= 0 {
            (
                year,
                month.number(),
                day,
                hour as u8,
                minute as u8,
                second as u8,
            )
        } else {
            (
                year,
                13 - month.number(),
                Month::December.previous_nth(month.index()).len(leap) - day + 1,
                23 - hour as u8,
                59 - minute as u8,
                60 - second as u8,
            )
        }
    }
}

// private functions
impl UnixTime {
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
    // Because of `u32` this will only work until `06:28:15 UTC on 07 February 2106`.
    #[cfg(all(not(feature = "std"), feature = "unsafe", feature = "libc"))]
    fn unix_time_64() -> i64 {
        // https://docs.rs/libc/latest/libc/fn.time.html
        #[allow(clippy::unnecessary_cast)] // could be i32 in other platforms?
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
    /// use espera::all::UnixTime32;
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
        all(not(feature = "std"), feature = "unsafe", feature = "libc")
    ))]
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(any(
            feature = "std",
            all(feature = "no_std", feature = "unsafe", feature = "libc")
        )))
    )]
    pub fn now() -> Self {
        Self {
            seconds: Self::unix_time_32(),
        }
    }

    /// Returns a `UnixTime32` converted to `(year, month, day, hour, minute, second)`.
    ///
    /// # Examples
    /// ```
    /// use espera::all::UnixTime32;
    ///
    /// assert_eq![(1970, 1, 1, 0, 0, 1), UnixTime32::new(1).to_ymdhms()];
    /// assert_eq![(2038, 1, 19, 3, 14, 7), UnixTime32::new(i32::MAX as u32).to_ymdhms()];
    /// ```
    pub const fn to_ymdhms(&self) -> (u16, u8, u8, u8, u8, u8) {
        let seconds_per_minute: u32 = 60;
        let minutes_per_hour: u32 = 60;
        let hours_per_day: u32 = 24;
        let days_per_year: u32 = 365;

        let mut seconds_left = self.seconds;
        let mut year = 1970;
        let mut leap = is_leap_year(year);

        while seconds_left
            >= (hours_per_day * minutes_per_hour * seconds_per_minute * days_per_year)
        {
            year += 1;
            leap = is_leap_year(year);
            let days_in_year = if leap { 366 } else { 365 };
            seconds_left -= hours_per_day * minutes_per_hour * seconds_per_minute * days_in_year;
        }

        let mut month = Month::January;
        while seconds_left
            >= hours_per_day * minutes_per_hour * seconds_per_minute * month.len(leap) as u32
        {
            seconds_left -=
                hours_per_day * minutes_per_hour * seconds_per_minute * month.len(leap) as u32;
            month = month.next();
        }

        let day = (seconds_left / (hours_per_day * minutes_per_hour * seconds_per_minute)) + 1;
        seconds_left %= hours_per_day * minutes_per_hour * seconds_per_minute;

        let hour = seconds_left / (minutes_per_hour * seconds_per_minute);
        seconds_left %= minutes_per_hour * seconds_per_minute;

        let minute = seconds_left / seconds_per_minute;
        let second = seconds_left % seconds_per_minute;

        (
            year as u16,
            month.number(),
            day as u8,
            hour as u8,
            minute as u8,
            second as u8,
        )
    }
}

// private functions
impl UnixTime32 {
    // Returns the number of seconds since `1970-01-01 00:00:00 UTC`.
    //
    // Because of `u32` this will only work until `06:28:15 UTC on 07 February 2106`.
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
    // Because of `u32` this will only work until `06:28:15 UTC on 07 February 2106`.
    #[cfg(all(not(feature = "std"), feature = "unsafe", feature = "libc"))]
    fn unix_time_32() -> u32 {
        unsafe { libc::time(core::ptr::null_mut()).clamp(0, u32::MAX as i64) as u32 }
    }
}

impl fmt::Display for UnixTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (y, m, d, h, min, s) = self.to_ymdhms();
        write![f, "{y:04}-{m:02}-{d:02}_{h:02}:{min:02}:{s:02}"]
    }
}
impl fmt::Debug for UnixTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (y, m, d, h, min, s) = self.to_ymdhms();
        write![
            f,
            "UnixTime {{ {y:04}-{m:02}-{d:02}_{h:02}:{min:02}:{s:02} }}"
        ]
    }
}

impl fmt::Display for UnixTime32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (y, m, d, h, min, s) = self.to_ymdhms();
        write![f, "{y:04}-{m:02}-{d:02}_{h:02}:{min:02}:{s:02}"]
    }
}

impl fmt::Debug for UnixTime32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (y, m, d, h, min, s) = self.to_ymdhms();
        write![
            f,
            "UnixTime32 {{ {y:04}-{m:02}-{d:02}_{h:02}:{min:02}:{s:02} }}"
        ]
    }
}

impl From<UnixTime32> for UnixTime {
    fn from(ut: UnixTime32) -> UnixTime {
        UnixTime {
            seconds: ut.seconds.into(),
        }
    }
}

impl TryFrom<UnixTime> for UnixTime32 {
    type Error = TryFromIntError;

    fn try_from(ut: UnixTime) -> Result<UnixTime32, Self::Error> {
        Ok(UnixTime32 {
            seconds: u32::try_from(ut.seconds)?,
        })
    }
}

// Implements From<primitive> for UnixTime*
macro_rules! impl_from_prim {
    // for many
    ($ut:ty, $($prim:ty),+) => { $( impl_from_prim![@ $ut, $prim]; )+ };
    (@ $ut:ty, $prim:ty) => {
        impl From<$prim> for $ut {
            fn from(seconds: $prim) -> $ut {
                Self { seconds: seconds.into() }
            }
        }
    };
}
impl_from_prim![UnixTime, i64, i32, i16, i8, u32, u16, u8];
impl_from_prim![UnixTime32, u32, u16, u8];

// Implements TryFrom<primitive> for UnixTime*
macro_rules! impl_try_from_prim {
    ($ut:ty, $($prim:ty),+) => { $( impl_try_from_prim![@ $ut, $prim]; )+ };
    (@ $ut:ty, $prim:ty) => {
        impl TryFrom<$prim> for $ut {
            type Error = TryFromIntError;
            fn try_from(seconds: $prim) -> Result<$ut, Self::Error> {
                Ok(Self { seconds: seconds.try_into()? })
            }
        }
    };
}
impl_try_from_prim![UnixTime, u64, u128, usize, i128, isize];
impl_try_from_prim![UnixTime32, u64, u128, usize, i8, i16, i32, i64, i128, isize];
