// espera::looper
//
//! Loop manager with support for multiple rates.
//

use crate::{Duration, Instant};

use ahash::AHashMap;

use std::thread::sleep;

use sixbit::DecodeSixbit;
use sixbit::EncodeSixbit;

use crate::all::{EsperaResult, Rate, RateStats};

/// The status for the loop state machine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopStatus {
    Active,
    Asleep,
}

/// A loop manager state machine.
#[derive(Clone, Debug)]
pub struct Looper {
    /// The loop status.
    ///
    /// Forces to alternate between a single sleep period and an active period.
    status: LoopStatus,

    /// The root rate.
    root_rate: Rate,
    /// Stats for the root rate.
    root_stats: RateStats,

    /// Custom rates.
    rates: AHashMap<u128, Rate>,
    /// Stats for the custom rates.
    stats: AHashMap<u128, RateStats>,
}

impl Default for Looper {
    fn default() -> Self {
        Self {
            status: LoopStatus::Active,
            root_rate: Rate::default(),
            root_stats: RateStats::default(),
            rates: AHashMap::new(),
            stats: AHashMap::new(),
        }
    }
}

impl Looper {
    /// Returns a new default looper.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Looper {
    /// Returns the current loop state machine status.
    pub fn status(&self) -> LoopStatus {
        self.status
    }

    /// Takes a measure.
    ///
    /// Returns the *now* instant and the *delta* since the last measure was taken.
    ///
    /// # State machine rules
    /// On LoopStatus match:
    /// + `Asleep`:
    ///   - sets status to `Active`
    ///   - sets the last measure to *now*.
    /// + `Active`:
    ///   - Returns `None`.
    //
    // RETHINK REMOVING the state machine…
    //
    pub fn measure(&mut self) -> Option<(Instant, Duration)> {
        match self.status {
            LoopStatus::Asleep => {
                let (now, delta) = self.now_delta();

                self.root_rate.set_last_tick(now);
                self.root_rate.increment_ticks();
                self.status = LoopStatus::Active;

                /* root averages */

                let ns: u64 = delta.whole_nanoseconds() as u64;
                self.root_stats.add_ns(ns);
                self.root_stats.update(self.root_rate.ticks());

                Some((now, delta))
            }
            LoopStatus::Active => None,
        }
    }

    /// Returns the current instant and the delta duration since last measure,
    /// calculated using that instant.
    #[inline]
    pub fn now_delta(&self) -> (Instant, Duration) {
        let now = Instant::now();
        let delta = now - self.root_rate.last_tick();
        (now, delta)
    }

    // MAYBE:WIP
    // /// Returns the difference between the last tick and the ideal instant
    // /// it should have been according to the real time.
    // pub fn root_desviation(&self) -> Duration {
    //     self.root_rate.first_tick();
    // }

    /// Resets all the accumulated times and statistics.
    // TODO
    // MAYBE RENAME to reset_all?)
    pub fn reset(&mut self) {
        self.reset_root();
        // self.reset_rates(); // TODO
    }

    /// Resets the accumulated times and statistics.
    //
    // RETHINK: generalize reset for each rate.
    // MAYBE reset only the base, or all the rates? (separate function)
    // WIP
    #[inline]
    pub fn reset_root(&mut self) {
        self.status = LoopStatus::Asleep;
        self.root_rate.reset();
        self.root_stats.reset();
    }

    /// Resets the accumulated times and statistics for all rates.
    // TODO
    pub fn reset_rate(&mut self, _rate_name: &str) {
        todo![]
    }

    /* rates */

    /// Add new rate to the looper, with the specificied `duration` per tick,
    /// and with optional `stats`.
    ///
    /// If the `name` already exists, the previous rate will be returned,
    /// and the new `rate` will take its place.
    ///
    /// The rate's `name` must be a unique string with a maximum length of 21
    /// ASCII alphanumeric + underscore characters ([A-Za-z0-9_]).
    ///
    /// # Errors
    /// Returns an error if the `name` is not valid.
    ///
    /// # Examples
    /// ```
    /// use espera::all::{Looper, Rate};
    ///
    /// let mut l = Looper::new();
    /// assert![l.add_rate("ascii_name_max_length", Rate::with_tps(60.), false).is_ok()];
    /// ```
    #[inline(always)]
    pub fn add_rate(&mut self, name: &str, rate: Rate, stats: bool) -> EsperaResult<Option<Rate>> {
        let key = name.chars().encode_sixbit::<u128>()?;
        if stats {
            let _prev_stats = self.stats.insert(key, RateStats::new());
        }
        Ok(self.rates.insert(key, rate))
    }

    /// Returns a reference to the requested `name`d rate.
    #[inline]
    pub fn ref_rate(&self, name: &str) -> Option<&Rate> {
        if let Ok(key) = name.chars().encode_sixbit::<u128>() {
            self.rates.get(&key)
        } else {
            None
        }
    }

    /// Returns an exclusive reference to the requested `name`d rate.
    #[inline]
    pub fn mut_rate(&mut self, name: &str) -> Option<&mut Rate> {
        if let Ok(key) = name.chars().encode_sixbit::<u128>() {
            self.rates.get_mut(&key)
        } else {
            None
        }
    }

    // MAYBE TODO: set_rate?

    /// Returns the duration of the fastest rate.
    ///
    /// Returns `None` if there are no configured rates.
    #[inline]
    pub fn fastest_rate_duration(&mut self) -> Option<Duration> {
        self.rates
            .iter()
            .min_by(|(_, a), (_, b)| a.duration().cmp(&b.duration()))
            .map(|(_, r)| r.duration())
    }

    /// Returns a reference to the root rate.
    #[inline]
    pub fn ref_root_rate(&self) -> &Rate {
        &self.root_rate
    }

    /// Returns an exclusive reference to the root rate.
    #[inline]
    pub fn mut_root_rate(&mut self) -> &mut Rate {
        &mut self.root_rate
    }

    /* ticks */

    /// Returns the duration between the last tick of the `name`d rate,
    /// and the provided `instant`, as long as the duration is non-negative.
    ///
    /// If the duration is non-negative, the ticks counter is also incremented
    /// and the instant of the last tick is replaced with the given `instant`.
    ///
    /// Returns `None` if the either the rate is not found in the rates list,
    /// or if the time difference is negative.
    ///
    /// # Precision
    /// This version should give a much more precise average frame rate than
    /// [`do_tick_fast`][Self::do_tick_fast], because it takes into accout the
    /// accumulated lag, at the cost of being a little less performant.
    ///
    /// The maximum lag taken into account is ± 2.1 s (±[`i32::MAX`] ns).
    pub fn do_tick(&mut self, now: Instant, name: &str) -> Option<Duration> {
        if let Ok(key) = name.chars().encode_sixbit::<u128>() {
            if let Some(rate) = self.rates.get_mut(&key) {
                if let Some(delta) = rate.do_tick(now) {
                    // stats
                    if let Some(stats) = self.stats.get_mut(&key) {
                        let ns: u64 = delta.whole_nanoseconds() as u64;
                        stats.add_ns(ns);
                        stats.update(rate.ticks());
                    }

                    // log::trace![
                    //     "{name:10}{rate} || Δ:{delta:.2}, TPS:{:.2}",
                    //     1.0 / delta.as_seconds_f64() ];

                    Some(delta)
                } else {
                    None // not yet tick time
                }
            } else {
                None // rate name not found
            }
        } else {
            None // invalid rate name
        }
    }
    /// Calls [`do_tick`][Self::do_tick] with `Instant::now()`.
    #[inline(always)]
    pub fn do_tick_now(&mut self, name: &str) -> Option<Duration> {
        self.do_tick(Instant::now(), name)
    }

    /// Returns the duration between the last tick of the `name`d rate,
    /// and the provided `instant`, as long as the duration is non-negative.
    ///
    /// If the duration is non-negative, the ticks counter is also incremented
    /// and the instant of the last tick is replaced with the given `instant`.
    ///
    /// Returns `None` if the either the rate is not found in the rates list,
    /// or if the time difference is negative.
    ///
    /// # Precision
    /// This version is less precise than [`do_tick`][Self::do_tick],
    /// because it doesn't try to compensate accumulated lag. It will probably
    /// lag a little behind the target rate, but should also be a little faster.
    pub fn do_tick_fast(&mut self, now: Instant, name: &str) -> Option<Duration> {
        if let Ok(key) = name.chars().encode_sixbit::<u128>() {
            if let Some(rate) = self.rates.get_mut(&key) {
                if let Some(delta) = rate.do_tick_fast(now) {
                    // stats
                    if let Some(stats) = self.stats.get_mut(&key) {
                        let ns: u64 = delta.whole_nanoseconds() as u64;
                        stats.add_ns(ns);
                        stats.update(rate.ticks());
                    }

                    // log::trace![
                    //     "{name:10} {rate} || Δ:{delta:.2}, TPS:{:.2}",
                    //     1.0 / delta.as_seconds_f64() ];

                    Some(delta)
                } else {
                    None // not yet tick time
                }
            } else {
                None // rate name not found
            }
        } else {
            None // invalid rate name
        }
    }
    /// Calls [`do_tick_fast`][Self::do_tick_fast] with `Instant::now()`.
    #[inline(always)]
    pub fn do_tick_fast_now(&mut self, name: &str) -> Option<Duration> {
        self.do_tick_fast(Instant::now(), name)
    }

    /* logging */

    /// Logs the stats of the root rate.
    #[inline]
    pub fn log_root_rate(&self) {
        // don't send
        self.root_stats.log("ROOT", None);
    }

    /// Logs the stats of a given rate.
    #[inline]
    pub fn log_rate(&self, name: &str) {
        if let Ok(key) = name.chars().encode_sixbit::<u128>() {
            if let Some(stats) = self.stats.get(&key) {
                let name = &key.decode_sixbit().collect::<String>();
                let rate = self.rates.get(&key);
                stats.log(name, rate);
            }
        }
    }

    /// Logs the stats of all rates.
    #[inline]
    pub fn log_all_rates(&self) {
        self.log_root_rate();
        for (key, _) in self.rates.iter() {
            if let Some(stats) = self.stats.get(key) {
                let name = &key.decode_sixbit().collect::<String>();
                let rate = self.rates.get(key);
                stats.log(name, rate);
            }
        }
    }

    /* sleep */

    /// Request to sleep for the requested positive `duration`.
    ///
    /// # State machine rules
    /// On LoopStatus match:
    /// + `Active`:
    ///   - sets status to `Sleep`
    ///   - sleeps for requested duration.
    /// + `Sleep`:
    ///   - Returns `None`.
    //
    // IMPROVE: check minimum resolution?
    pub fn sleep(&mut self, duration: Duration) {
        if let LoopStatus::Active = self.status {
            self.status = LoopStatus::Asleep;
            if duration.is_positive() {
                // log::debug!["sleep: {duration}"];
                sleep(duration.unsigned_abs());
            }
        }
    }

    // MAYBE
    // /// Sleeps enough time to stabilize as closest as possible to
    // //
    // // returns the delta
    // pub fn sleep_target(&mut self, target: Duration) -> Option<Duration> {
    //     // let target
    //     // self.sleep(target)
    //     todo![];
    // }
}
