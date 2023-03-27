// espera::calendar::week
//
//!
//

use core::{fmt, str::FromStr};
use Weekday::*;

/// The week days.
///
/// Goes from 0 (Monday) to 6 (Sunday).
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Weekday {
    Monday = 0,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

/// # 3 letter abbreviations.
impl Weekday {
    pub const Mon: Weekday = Weekday::Monday;
    pub const Tue: Weekday = Weekday::Tuesday;
    pub const Wed: Weekday = Weekday::Wednesday;
    pub const Thu: Weekday = Weekday::Thursday;
    pub const Fri: Weekday = Weekday::Friday;
    pub const Sat: Weekday = Weekday::Saturday;
    pub const Sun: Weekday = Weekday::Sunday;
}

/// # 2 letter abbreviations.
impl Weekday {
    pub const MO: Weekday = Weekday::Monday;
    pub const TU: Weekday = Weekday::Tuesday;
    pub const WE: Weekday = Weekday::Wednesday;
    pub const TH: Weekday = Weekday::Thursday;
    pub const FR: Weekday = Weekday::Friday;
    pub const SA: Weekday = Weekday::Saturday;
    pub const SU: Weekday = Weekday::Sunday;
}

/// # 1 letter abbreviations.
impl Weekday {
    pub const M: Weekday = Weekday::Monday;
    pub const T: Weekday = Weekday::Tuesday;
    pub const W: Weekday = Weekday::Wednesday;
    pub const H: Weekday = Weekday::Thursday;
    pub const F: Weekday = Weekday::Friday;
    pub const S: Weekday = Weekday::Saturday;
    pub const U: Weekday = Weekday::Sunday;
}

impl Weekday {
    /// Returns the previous weekday,
    #[inline]
    pub const fn previous(&self) -> Weekday {
        match self {
            Monday => Sunday,
            Tuesday => Monday,
            Wednesday => Tuesday,
            Thursday => Wednesday,
            Friday => Thursday,
            Saturday => Friday,
            Sunday => Saturday,
        }
    }

    /// Returns the next weekday,
    #[inline]
    pub const fn next(self) -> Self {
        match self {
            Monday => Tuesday,
            Tuesday => Wednesday,
            Wednesday => Thursday,
            Thursday => Friday,
            Friday => Saturday,
            Saturday => Sunday,
            Sunday => Monday,
        }
    }

    /* numbers */

    /// Returns the weekday number from Monday being 1.
    #[inline(always)]
    pub const fn number_from_monday(self) -> u8 {
        self.index_from_monday() as u8 + 1
    }

    /// Returns the weekday number from Sunday being 1.
    #[inline(always)]
    pub const fn number_from_sunday(self) -> u8 {
        self.index_from_sunday() as u8 + 1
    }

    /// Returns the weekday index from Monday being 0.
    #[inline(always)]
    pub const fn index_from_monday(self) -> usize {
        self as _
    }

    /// Returns the weekday index from Sunday being 0.
    #[inline]
    pub const fn index_from_sunday(self) -> usize {
        match self {
            Monday => 1,
            Tuesday => 2,
            Wednesday => 3,
            Thursday => 4,
            Friday => 5,
            Saturday => 6,
            Sunday => 0,
        }
    }

    /* abbreviations */

    /// Returns the 3-letter abbreviated weekday name, in ASCII, UpperCamelCase.
    pub fn abbr3(&self) -> &'static str {
        match self {
            Monday => "Mon",
            Tuesday => "Tue",
            Wednesday => "Wed",
            Thursday => "Thu",
            Friday => "Fru",
            Saturday => "Sat",
            Sunday => "Sun",
        }
    }
    /// Returns the 2-letter abbreviated weekday name, in ASCII, uppercase.
    pub fn abbr2(&self) -> &'static str {
        match self {
            Monday => "MO",
            Tuesday => "TU",
            Wednesday => "WE",
            Thursday => "TH",
            Friday => "FR",
            Saturday => "SA",
            Sunday => "SU",
        }
    }
    /// Returns the 1-letter abbreviated weekday name, in ASCII, uppercase.
    pub fn abbr1(&self) -> &'static str {
        match self {
            Monday => "M",
            Tuesday => "T",
            Wednesday => "W",
            Thursday => "H",
            Friday => "F",
            Saturday => "S",
            Sunday => "U",
        }
    }
}

impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Monday => "Monday",
            Tuesday => "Tuesday",
            Wednesday => "Wednesday",
            Thursday => "Thursday",
            Friday => "Friday",
            Saturday => "Saturday",
            Sunday => "Sunday",
        })
    }
}

impl FromStr for Weekday {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // full name
        if s.eq_ignore_ascii_case("Monday") {
            Ok(Monday)
        } else if s.eq_ignore_ascii_case("Tuesday") {
            Ok(Tuesday)
        } else if s.eq_ignore_ascii_case("Wednesday") {
            Ok(Wednesday)
        } else if s.eq_ignore_ascii_case("Thursday") {
            Ok(Thursday)
        } else if s.eq_ignore_ascii_case("Friday") {
            Ok(Friday)
        } else if s.eq_ignore_ascii_case("Saturday") {
            Ok(Saturday)
        } else if s.eq_ignore_ascii_case("Sunday") {
            Ok(Sunday)
        // abbr3
        } else if s.eq_ignore_ascii_case("Mon") {
            Ok(Monday)
        } else if s.eq_ignore_ascii_case("Tue") {
            Ok(Tuesday)
        } else if s.eq_ignore_ascii_case("Wed") {
            Ok(Wednesday)
        } else if s.eq_ignore_ascii_case("Thu") {
            Ok(Thursday)
        } else if s.eq_ignore_ascii_case("Fri") {
            Ok(Friday)
        } else if s.eq_ignore_ascii_case("Sat") {
            Ok(Saturday)
        } else if s.eq_ignore_ascii_case("Sun") {
            Ok(Sunday)
        // abbr2
        } else if s.eq_ignore_ascii_case("MO") {
            Ok(Monday)
        } else if s.eq_ignore_ascii_case("TU") {
            Ok(Tuesday)
        } else if s.eq_ignore_ascii_case("WE") {
            Ok(Wednesday)
        } else if s.eq_ignore_ascii_case("TH") {
            Ok(Thursday)
        } else if s.eq_ignore_ascii_case("FR") {
            Ok(Friday)
        } else if s.eq_ignore_ascii_case("SA") {
            Ok(Saturday)
        } else if s.eq_ignore_ascii_case("SU") {
            Ok(Sunday)
        // abbr1
        } else if s.eq_ignore_ascii_case("M") {
            Ok(Monday)
        } else if s.eq_ignore_ascii_case("T") {
            Ok(Tuesday)
        } else if s.eq_ignore_ascii_case("W") {
            Ok(Wednesday)
        } else if s.eq_ignore_ascii_case("H") {
            Ok(Thursday)
        } else if s.eq_ignore_ascii_case("F") {
            Ok(Friday)
        } else if s.eq_ignore_ascii_case("S") {
            Ok(Saturday)
        } else if s.eq_ignore_ascii_case("U") {
            Ok(Sunday)

        //
        } else {
            Err("Invalid weekday name.")
        }
    }
}
