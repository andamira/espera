// espera::calendar::month
//
//!
//

use core::{fmt, str::FromStr};
use Month::*;

/// The months.
///
/// Goes from 0 (January) to 11 (December).
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum Month {
    January = 0,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

/// # 3 letter abbreviations.
impl Month {
    pub const Jan: Month = Month::January;
    pub const Feb: Month = Month::February;
    pub const Mar: Month = Month::March;
    pub const Apr: Month = Month::April;
    pub const May: Month = Month::May;
    pub const Jun: Month = Month::June;
    pub const Jul: Month = Month::July;
    pub const Aug: Month = Month::August;
    pub const Sep: Month = Month::September;
    pub const Oct: Month = Month::October;
    pub const Nov: Month = Month::November;
    pub const Dec: Month = Month::December;
}

/// # 2 letter abbreviations.
impl Month {
    pub const JA: Month = Month::January;
    pub const FE: Month = Month::February;
    pub const MR: Month = Month::March;
    pub const AP: Month = Month::April;
    pub const MY: Month = Month::May;
    pub const JN: Month = Month::June;
    pub const JL: Month = Month::July;
    pub const AU: Month = Month::August;
    pub const SE: Month = Month::September;
    pub const OC: Month = Month::October;
    pub const NV: Month = Month::November;
    pub const DE: Month = Month::December;
}

impl Month {
    /// Returns the number of days of each month.
    ///
    /// It doesn't take into account leap years.
    pub const fn days(&self) -> u8 {
        match self {
            January => 31,
            February => 28, // leap year 29 days
            March => 31,
            April => 30,
            May => 31,
            June => 30,
            July => 31,
            August => 31,
            September => 30,
            October => 31,
            November => 30,
            December => 31,
        }
    }

    /// Returns the previous month.
    #[inline]
    pub const fn previous(self) -> Self {
        match self {
            January => December,
            February => January,
            March => February,
            April => March,
            May => April,
            June => May,
            July => June,
            August => July,
            September => August,
            October => September,
            November => October,
            December => November,
        }
    }

    /// Returns the next month.
    #[inline]
    pub const fn next(self) -> Self {
        match self {
            January => February,
            February => March,
            March => April,
            April => May,
            May => June,
            June => July,
            July => August,
            August => September,
            September => October,
            October => November,
            November => December,
            December => January,
        }
    }

    /* numbers */

    /// Returns the month number from January being 1.
    #[inline(always)]
    pub const fn number(self) -> u8 {
        self.index() + 1
    }

    /// Returns the month index from January being 0.
    #[inline]
    pub const fn index(self) -> u8 {
        self as _
    }

    /// Returns a month from a month number (between 1 and 12).
    pub const fn from_number(n: u8) -> Result<Self, &'static str> {
        match n {
            1 => Ok(January),
            2 => Ok(February),
            3 => Ok(March),
            4 => Ok(April),
            5 => Ok(May),
            6 => Ok(June),
            7 => Ok(July),
            8 => Ok(August),
            9 => Ok(September),
            10 => Ok(October),
            11 => Ok(November),
            12 => Ok(December),
            _ => Err("The month number must be between 1 and 12."),
        }
    }

    /// Returns a month from a month index (between 0 and 11).
    pub const fn from_index(n: u8) -> Result<Self, &'static str> {
        match n {
            0 => Ok(January),
            1 => Ok(February),
            2 => Ok(March),
            3 => Ok(April),
            4 => Ok(May),
            5 => Ok(June),
            6 => Ok(July),
            7 => Ok(August),
            8 => Ok(September),
            9 => Ok(October),
            10 => Ok(November),
            11 => Ok(December),
            _ => Err("The month index must be between 0 and 11."),
        }
    }

    /* abbreviations */

    /// Returns the 3-letter abbreviated month name, in ASCII, UpperCamelCase.
    pub const fn abbr3(&self) -> &'static str {
        match self {
            January => "Jan",
            February => "Feb",
            March => "Mar",
            April => "Apr",
            May => "May",
            June => "Jun",
            July => "Jul",
            August => "Aug",
            September => "Sep",
            October => "Oct",
            November => "Nov",
            December => "Dec",
        }
    }

    /// Returns the 2-letter abbreviated month name, in ASCII, uppercase.
    pub const fn abbr2(&self) -> &'static str {
        match self {
            January => "JA",
            February => "FE",
            March => "MR",
            April => "AP",
            May => "MY",
            June => "JN",
            July => "JL",
            August => "AU",
            September => "SE",
            October => "OC",
            November => "NV",
            December => "DE",
        }
    }

    /// Returns the 1-letter abbreviated month name, in ASCII, uppercase.
    pub const fn abbr1(&self) -> &'static str {
        match self {
            January => "J",
            February => "F",
            March => "M",
            April => "P",
            May => "Y",
            June => "N",
            July => "L",
            August => "U",
            September => "S",
            October => "O",
            November => "N",
            December => "D",
        }
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            January => "January",
            February => "February",
            March => "March",
            April => "April",
            May => "May",
            June => "June",
            July => "July",
            August => "August",
            September => "September",
            October => "October",
            November => "November",
            December => "December",
        })
    }
}

impl From<Month> for u8 {
    fn from(month: Month) -> Self {
        month as _
    }
}

impl FromStr for Month {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("January") {
            Ok(January)
        } else if s.eq_ignore_ascii_case("February") {
            Ok(February)
        } else if s.eq_ignore_ascii_case("March") {
            Ok(March)
        } else if s.eq_ignore_ascii_case("April") {
            Ok(April)
        } else if s.eq_ignore_ascii_case("May") {
            Ok(May)
        } else if s.eq_ignore_ascii_case("June") {
            Ok(June)
        } else if s.eq_ignore_ascii_case("July") {
            Ok(July)
        } else if s.eq_ignore_ascii_case("August") {
            Ok(August)
        } else if s.eq_ignore_ascii_case("September") {
            Ok(September)
        } else if s.eq_ignore_ascii_case("October") {
            Ok(October)
        } else if s.eq_ignore_ascii_case("November") {
            Ok(November)
        } else if s.eq_ignore_ascii_case("December") {
            Ok(December)
        //
        } else if s.eq_ignore_ascii_case("Jan") {
            Ok(January)
        } else if s.eq_ignore_ascii_case("Feb") {
            Ok(February)
        } else if s.eq_ignore_ascii_case("Mar") {
            Ok(March)
        } else if s.eq_ignore_ascii_case("Apr") {
            Ok(April)
        // } else if s.eq_ignore_ascii_case("May") {
        //     Ok(May)
        } else if s.eq_ignore_ascii_case("Jun") {
            Ok(June)
        } else if s.eq_ignore_ascii_case("Jul") {
            Ok(July)
        } else if s.eq_ignore_ascii_case("Aug") {
            Ok(August)
        } else if s.eq_ignore_ascii_case("Sep") {
            Ok(September)
        } else if s.eq_ignore_ascii_case("Oct") {
            Ok(October)
        } else if s.eq_ignore_ascii_case("Nov") {
            Ok(November)
        } else if s.eq_ignore_ascii_case("Dec") {
            Ok(December)
        // abbr2
        } else if s.eq_ignore_ascii_case("JA") {
            Ok(January)
        } else if s.eq_ignore_ascii_case("FE") {
            Ok(February)
        } else if s.eq_ignore_ascii_case("MR") {
            Ok(March)
        } else if s.eq_ignore_ascii_case("AP") {
            Ok(April)
        } else if s.eq_ignore_ascii_case("MY") {
            Ok(May)
        } else if s.eq_ignore_ascii_case("JN") {
            Ok(June)
        } else if s.eq_ignore_ascii_case("JL") {
            Ok(July)
        } else if s.eq_ignore_ascii_case("AU") {
            Ok(August)
        } else if s.eq_ignore_ascii_case("SE") {
            Ok(September)
        } else if s.eq_ignore_ascii_case("OC") {
            Ok(October)
        } else if s.eq_ignore_ascii_case("NV") {
            Ok(November)
        } else if s.eq_ignore_ascii_case("DE") {
            Ok(December)
        // abbr1
        } else if s.eq_ignore_ascii_case("J") {
            Ok(January)
        } else if s.eq_ignore_ascii_case("F") {
            Ok(February)
        } else if s.eq_ignore_ascii_case("M") {
            Ok(March)
        } else if s.eq_ignore_ascii_case("P") {
            Ok(April)
        } else if s.eq_ignore_ascii_case("Y") {
            Ok(May)
        } else if s.eq_ignore_ascii_case("N") {
            Ok(June)
        } else if s.eq_ignore_ascii_case("L") {
            Ok(July)
        } else if s.eq_ignore_ascii_case("U") {
            Ok(August)
        } else if s.eq_ignore_ascii_case("S") {
            Ok(September)
        } else if s.eq_ignore_ascii_case("O") {
            Ok(October)
        } else if s.eq_ignore_ascii_case("N") {
            Ok(November)
        } else if s.eq_ignore_ascii_case("D") {
            Ok(December)

        //
        } else {
            Err("Invalid month name.")
        }
    }
}
