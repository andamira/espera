// espera::rate::rate
//
//!
//

use crate::all::{Duration, Instant};

/// A rate allows to control a periodic repetition in time.
///
// Note that when duration is ZERO it will be ignored in practice.
//
// Size: 60 Bytes = 16 + 16 + 16 + 8 + 4
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rate {
    /// Target duration per tick.
    ///
    // If zero, it will be treated as not important to enforce.
    duration: Duration,
    /// the instant of the last tick.
    last_tick: Instant,
    /// The instant of the first tick.
    first_tick: Instant,
    /// Number of ticks recorded.
    ticks: u64,

    /// Saves the difference of the delta time against the target duration,
    /// allowing to automatically compensate lag.
    ///
    // Storing the nanoseconds in a i32 allows us to store
    // up to 2 seconds of either positive or negative lag.
    delta_rem: i32,
    // MAYBE: Whether to allocate and manage associated stats.
    // stats: bool,
    // ...

    // 4 bytes more to reach 64B
    // e.g. 2xbool 1xu16
    // e.g. 1xu32
}

impl Default for Rate {
    /// Returns a new `Rate` with zero duration per tick.
    fn default() -> Self {
        Self {
            duration: Duration::ZERO,
            first_tick: Instant::now(),
            last_tick: Instant::now(),
            ticks: 0,
            delta_rem: 0,
        }
    }
}

impl Rate {
    /// Returns a new `Rate` with the given `duration_per_tick`.
    ///
    /// # Examples
    /// ```
    /// use espera::all::{Duration, Rate};
    ///
    /// let r = Rate::new(Duration::milliseconds(25));
    /// ```
    pub fn new(duration_per_tick: Duration) -> Self {
        Self {
            duration: duration_per_tick,
            first_tick: Instant::now(),
            last_tick: Instant::now(),
            ticks: 0,
            delta_rem: 0,
        }
    }

    /// Returns a new `Rate` with the given `seconds_per_tick`.
    ///
    /// # Examples
    /// ```
    /// use espera::all::Rate;
    ///
    /// let r = Rate::with_seconds(0.025);
    /// ```
    pub fn with_seconds(seconds_per_tick: f64) -> Self {
        Self::new(Duration::seconds_f64(seconds_per_tick))
    }

    /// Returns a new `Rate` with the given `ticks_per_second`.
    ///
    /// # Examples
    /// ```
    /// use espera::all::Rate;
    ///
    /// let r = Rate::with_tps(40.0);
    /// ```
    pub fn with_tps(ticks_per_second: f64) -> Self {
        Self::new(Duration::seconds_f64(1.0 / ticks_per_second))
    }

    //

    /// Resets the number of ticks to 0, and the first and last ticks to now.
    ///
    /// # Examples
    /// ```
    /// use espera::all::Rate;
    ///
    /// let mut r = Rate::default();
    /// r.increment_ticks();
    /// r.reset();
    /// assert_eq![0, r.ticks()];
    /// ```
    #[inline(always)]
    pub fn reset(&mut self) {
        self.ticks = 0;
        self.first_tick = Instant::now();
        self.last_tick = Instant::now();
        self.delta_rem = 0;
    }

    //

    /// Returns the current number of ticks.
    ///
    /// # Examples
    /// ```
    /// use espera::all::Rate;
    ///
    /// let r = Rate::default();
    /// assert_eq![0, r.ticks()];
    /// ```
    #[inline(always)]
    pub const fn ticks(&self) -> u64 {
        self.ticks
    }

    /// Increments the current number of ticks by 1.
    ///
    /// # Examples
    /// ```
    /// use espera::all::Rate;
    ///
    /// let mut r = Rate::default();
    /// r.increment_ticks();
    /// assert_eq![1, r.ticks()];
    /// ```
    #[inline(always)]
    pub fn increment_ticks(&mut self) {
        self.ticks += 1;
    }

    //

    /// Returns the instant of the first tick.
    ///
    /// # Examples
    /// ```
    /// use espera::all::{Instant, Rate};
    ///
    /// let r = Rate::default();
    /// assert![r.first_tick() < Instant::now()];
    /// ```
    #[inline(always)]
    pub const fn first_tick(&self) -> Instant {
        self.first_tick
    }

    /// Sets the `instant` of the first tick.
    ///
    /// # Examples
    /// ```
    /// use espera::all::{Instant, Rate};
    ///
    /// let mut r = Rate::default();
    /// let now = Instant::now();
    /// r.set_first_tick(now);
    /// assert_eq![now, r.first_tick()];
    /// ```
    #[inline(always)]
    pub fn set_first_tick(&mut self, instant: Instant) {
        self.first_tick = instant;
    }

    /// Returns the instant of the last tick.
    ///
    /// # Examples
    /// ```
    /// use espera::all::Rate;
    ///
    /// let r = Rate::default();
    /// assert![r.last_tick() >= r.first_tick()];
    /// ```
    #[inline(always)]
    pub const fn last_tick(&self) -> Instant {
        self.last_tick
    }

    /// Sets the `instant` of the last tick.
    ///
    /// # Examples
    /// ```
    /// use espera::all::{Instant, Rate};
    ///
    /// let mut r = Rate::default();
    /// let now = Instant::now();
    /// r.set_last_tick(now);
    /// assert_eq![now, r.last_tick()];
    /// ```
    #[inline(always)]
    pub fn set_last_tick(&mut self, instant: Instant) {
        self.last_tick = instant;
    }

    //

    /// Returns the duration per tick.
    ///
    /// # Examples
    /// ```
    /// use espera::all::{Duration, Rate};
    ///
    /// let r = Rate::with_tps(40.0);
    /// assert_eq![Duration::milliseconds(25), r.duration()];
    /// ```
    #[inline(always)]
    pub const fn duration(&self) -> Duration {
        self.duration
    }

    /// Returns the ticks per second.
    ///
    /// # Examples
    /// ```
    /// use espera::all::{Duration, Rate};
    ///
    /// let r = Rate::new(Duration::milliseconds(25));
    /// assert_eq![40.0, r.tps()];
    /// ```
    #[inline(always)]
    pub fn tps(&self) -> f64 {
        1. / self.duration.as_seconds_f64()
    }

    /// Sets the `duration_per_tick`.
    ///
    /// # Examples
    /// ```
    /// use espera::all::{Duration, Rate};
    ///
    /// let mut r = Rate::default();
    /// r.set_duration(Duration::milliseconds(25));
    /// ```
    #[inline(always)]
    pub fn set_duration(&mut self, duration_per_tick: Duration) {
        self.duration = duration_per_tick;
    }

    /// Sets the `seconds_per_tick`.
    ///
    /// # Examples
    /// ```
    /// use espera::all::Rate;
    ///
    /// let mut r = Rate::default();
    /// r.set_seconds(0.025);
    /// ```
    #[inline(always)]
    pub fn set_seconds(&mut self, seconds_per_tick: f64) {
        self.duration = Duration::seconds_f64(seconds_per_tick);
    }

    /// Sets the `ticks_per_second`.
    ///
    /// # Examples
    /// ```
    /// use espera::all::Rate;
    ///
    /// let mut r = Rate::default();
    /// r.set_tps(40.0);
    /// ```
    #[inline(always)]
    pub fn set_tps(&mut self, ticks_per_second: f64) {
        self.duration = Duration::seconds_f64(1. / ticks_per_second);
    }

    //

    /// Returns the duration between the [`last_tick`][Self::last_tick] and
    /// the given `instant`, as long as the duration is non-negative.
    ///
    /// In which case the [`ticks`][Self::ticks] counter is incremented
    /// and the instant of the last tick is replaced with the given `instant`.
    ///
    /// Otherwise, if the time difference is negative `None` is returned.
    ///
    /// # Precision
    /// This version should give a much more precise average frame rate than
    /// [`do_tick_fast`][Self::do_tick_fast], because it takes into account the
    /// accumulated lag, at the cost of being a little less performant.
    ///
    /// The maximum accumulated lag is ± 2.1 s (±[`i32::MAX`] ns).
    /// # Examples
    /// ```
    /// use espera::all::{Instant, Rate};
    ///
    /// let mut r = Rate::with_seconds(0.025);
    /// for _ in 0..2 {
    ///     let d = r.do_tick(Instant::now());
    /// }
    /// ```
    #[inline]
    pub fn do_tick(&mut self, instant: Instant) -> Option<Duration> {
        let delta = self.last_elapsed(instant);
        if (delta + Duration::new(0, self.delta_rem)) >= self.duration {
            let lag: i128 = (delta - self.duration).whole_nanoseconds();
            let lag_clamped = lag.clamp(i32::MIN as i128, i32::MAX as i128);

            // FIX overflows!
            // log::debug![
            //     ">> Δ:{delta} + Δ_acc:{} ...",
            //     self.delta_rem
            // ];
            // self.delta_rem += lag_clamped as i32;
            self.delta_rem = self.delta_rem.saturating_add(lag_clamped as i32);

            //     ">> Δ:{delta} + Δ_acc:{} >= dur (lag:{lag_clamped})",
            //     self.delta_rem
            // ];
            self.increment_ticks();
            self.set_last_tick(instant);
            Some(delta)
        } else {
            None
        }
    }
    /// Calls [`do_tick`][Self::do_tick] with `Instant::now()`.
    #[inline(always)]
    pub fn do_tick_now(&mut self) -> Option<Duration> {
        self.do_tick(Instant::now())
    }

    /// Returns the duration between the last tick and the provided `instant`,
    /// as long as the duration is non-negative.
    ///
    /// If the duration is non-negative, the ticks counter is incremented
    /// and the instant of the last tick is replaced with the given `instant`.
    ///
    /// Otherwise, if the time difference is negative then `None` is returned.
    ///
    /// # Precision
    /// This version is less precise than [`do_tick`][Self::do_tick],
    /// because it doesn't try to compensate accumulated lag. It will probably
    /// lag a little behind the target rate, but should also be a little faster.
    #[inline]
    pub fn do_tick_fast(&mut self, instant: Instant) -> Option<Duration> {
        let delta = self.last_elapsed(instant);
        if delta >= self.duration {
            self.increment_ticks();
            self.set_last_tick(instant);
            Some(delta)
        } else {
            None
        }
    }
    /// Calls [`do_tick_fast`][Self::do_tick_fast] with `Instant::now()`.
    #[inline(always)]
    pub fn do_tick_fast_now(&mut self) -> Option<Duration> {
        self.do_tick_fast(Instant::now())
    }

    //

    /// Returns the elapsed time between the given `instant` and the first
    /// recorded tick.
    #[inline(always)]
    pub fn first_elapsed(&self, instant: Instant) -> Duration {
        instant - self.first_tick
    }

    /// Returns the elapsed time between the given `instant` and the last
    /// recorded tick.
    #[inline(always)]
    pub fn last_elapsed(&self, instant: Instant) -> Duration {
        instant - self.last_tick
    }

    /// Returns the elapsed time between the given `instant` and `tick` number.
    /// according to the [`duration`][Self::duration] per tick and the
    /// [`first_tick`][Self::first_tick].
    ///
    /// If the `tick` is in the future the duration will be negative.
    /// or the negative duration of when it will come to pass.
    // TEST
    #[inline(always)]
    pub fn tick_elapsed(&self, tick: u64, instant: Instant) -> Duration {
        instant - self.instant_tick(tick)
    }

    /// Returns the total duration for the provided number of `ticks`.
    ///
    /// Note that the maximum representable duration is i64::MAX seconds (+2.14).
    #[inline(always)]
    pub fn duration_ticks(&self, ticks: u64) -> Duration {
        Duration::seconds_f64(self.duration.as_seconds_f64() * ticks as f64)
    }

    /// Returns the instant the provided `tick` should happen, according to the
    /// [`duration`][Self::duration] per tick, and the instant of the
    /// [`first_tick`][Self::first_tick].
    ///
    /// # Panics
    /// Panics on overflow which, for example, in linux can happen at
    /// [`u64::MAX`] ticks at 500ms per-tick, [`i64::MAX`] ticks at 1s per-tick,
    /// or [`u32::MAX`] ticks at ([`i32::MAX`] + 2)s per-tick...
    #[inline(always)]
    pub fn instant_tick(&self, tick: u64) -> Instant {
        self.first_tick + self.duration_ticks(tick)
    }

    /// Returns the instant the provided `tick` should happen, according to the
    /// [`duration`][Self::duration] per tick, and the instant of the
    /// [`first_tick`][Self::first_tick].
    ///
    /// Like [`instant_tick`][Self::instant_tick] but returns `None` instead of
    /// panicking.
    #[inline(always)]
    pub fn instant_tick_checked(&self, tick: u64) -> Option<Instant> {
        self.first_tick.checked_add(self.duration_ticks(tick))
    }

    //

    // MAYBE
    // /// Pretty print...?
    // pub fn to_string_with_delta(&self, delta: Duration) -> String {
    //
    // }
}

mod core_impl {
    use super::Rate;
    use core::fmt;

    impl fmt::Display for Rate {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "#{:05}, tps:{}, Δdiff:{}",
                self.ticks, self.duration, self.delta_rem,
            )
        }
    }
}
