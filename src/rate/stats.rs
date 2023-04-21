// espera::rate::stats
//
//!
//

use crate::{all::Rate, Duration};
use arraydeque::{ArrayDeque, Wrapping};
use core::cmp;

// /// The max size of the ring buffer that stores measures.
// const RATE_RING_LEN: usize = 1024;

// TEMP (to print function?)
// const S_TO_NS: f64 = 1e+9;
const NS_TO_S: f64 = 1e-9;
// const S_TO_US: f64 = 1e+6;
// const US_TO_S: f64 = 1e-6;
// const S_TO_MS: f64 = 1e+3;
// const MS_TO_S: f64 = 1e-3;

/// Statistics for a given Rate.
///
/// Average
#[derive(Clone, Debug)]
pub struct RateStats {
    /// A collection of measures in a ring.
    avg_ring: ArrayDeque<[u64; 1024], Wrapping>,

    // diferent window sizes
    avg_16: f64,
    avg_128: f64,
    avg_1024: f64,
    //
    max_ns_16: u64,
    max_ns_128: u64,
    max_ns_1024: u64,
}

impl Default for RateStats {
    fn default() -> Self {
        Self {
            avg_ring: ArrayDeque::new(),
            avg_16: 0.0,
            avg_128: 0.0,
            avg_1024: 0.0,

            max_ns_16: 0,
            max_ns_128: 0,
            max_ns_1024: 0,
        }
    }
}

impl RateStats {
    /// Returns a new `RateStats`.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new `duration` to the stats.
    #[inline]
    pub fn add(&mut self, duration: Duration) {
        self.avg_ring
            .push_back(cmp::max(0_i128, duration.whole_nanoseconds()) as u64);
    }
    /// Adds a new `nanoseconds` value to the stats.
    #[inline]
    pub fn add_ns(&mut self, nanoseconds: u64) {
        self.avg_ring.push_back(nanoseconds);
    }

    /// Updates the statistics for each time window that aligns with
    /// the provided tick count.
    pub fn update(&mut self, tick_count: u64) {
        // IMPROVE IDEA we may have a little performance gain
        // if we could reuse %16 for %128 and that for %1024

        if tick_count % 16 == 0 {
            let mut avg_accumulator = 0_u64;
            self.max_ns_16 = 0;
            let mut i = self.avg_ring.iter();
            for _ in 0..16 {
                let val = i.next_back().unwrap_or(&0);
                avg_accumulator += i.next_back().unwrap_or(&0);
                self.max_ns_16 = cmp::max(self.max_ns_16, *val);
            }
            self.avg_16 = avg_accumulator as f64 / 16.;
        }

        if tick_count % 128 == 0 {
            let mut avg_accumulator = 0_u64;
            self.max_ns_128 = 0;
            let mut i = self.avg_ring.iter();
            for _ in 0..128 {
                let val = i.next_back().unwrap_or(&0);
                avg_accumulator += val;
                self.max_ns_128 = cmp::max(self.max_ns_128, *val);
            }
            self.avg_128 = avg_accumulator as f64 / 128.;
        }

        if tick_count % 1024 == 0 {
            let mut avg_accumulator = 0_u64;
            let mut i = self.avg_ring.iter();

            self.max_ns_1024 = 0;
            for _ in 0..1024 {
                let val = i.next_back().unwrap_or(&0);
                avg_accumulator += val;
                self.max_ns_1024 = cmp::max(self.max_ns_1024, *val);
            }
            self.avg_1024 = avg_accumulator as f64 / 1024.;
        }
    }

    /// Resets the stats.
    pub fn reset(&mut self) {
        self.avg_16 = 0.0;
        self.avg_128 = 0.0;
        self.avg_1024 = 0.0;

        self.max_ns_128 = 0;
        self.max_ns_1024 = 0;
    }

    /// Logs the recorded stats, with the provided `name`, and the optional
    /// `rate` for comparison.
    //
    // - MAYBE print how much time can base_ticks continue at current rate
    //
    // - IMPROVE
    pub fn log(&self, name: &str, rate: Option<&Rate>) {
        // average tps for each window
        let avg_16 = 1. / (self.avg_16 * NS_TO_S);
        let avg_128 = 1. / (self.avg_128 * NS_TO_S);
        let avg_1024 = 1. / (self.avg_1024 * NS_TO_S);
        // minimum tps for each window
        let min_16 = 1. / (self.max_ns_16 as f64 * NS_TO_S);
        let min_128 = 1. / (self.max_ns_128 as f64 * NS_TO_S);
        let min_1024 = 1. / (self.max_ns_1024 as f64 * NS_TO_S);

        // show % against rate's tps if avaiable
        if let Some(rate) = rate {
            let d = rate.duration();

            let tps = rate.tps();
            // % deviations from base for averages
            let pcta_16 = avg_16 / tps * 100.;
            let pcta_128 = avg_128 / tps * 100.;
            let pcta_1024 = avg_1024 / tps * 100.;
            // % deviations from base for minimums
            let pctm_16 = min_16 / tps * 100.;
            let pctm_128 = min_128 / tps * 100.;
            let pctm_1024 = min_1024 / tps * 100.;

            log::trace![
                "[window]avg(%)|min(%) rate tps:{tps:.2} dpt:{d} \"{name}\":
[16]{avg_16:.2}({pcta_16:.1}%)|{min_16:.2}({pctm_16:.1}%) \
[128]{avg_128:.2}({pcta_128:.1}%)|{min_128:.2}({pctm_128:.1}%) \
[1024]{avg_1024:.2}({pcta_1024:.1}%)|{min_1024:.2}({pctm_1024:.1}%)
"
            ];
        // or don't
        } else {
            // % deviations from average for minimums
            let pctm_16 = min_16 / avg_16 * 100.;
            let pctm_128 = min_128 / avg_128 * 100.;
            let pctm_1024 = min_1024 / avg_1024 * 100.;

            log::trace![
                "[window]avg|min rate \"{name}\":
[16]{avg_16:.2}|{min_16:.2}({pctm_16:.1}%) \
[128]{avg_128:.2}|{min_128:.2}({pctm_128:.1}%) \
[1024]{avg_1024:.2}|{min_1024:.2}({pctm_1024:.1}%)
"
            ];
        }
        //         // no percentages
        //         } else {
        //             // % deviations from base for minimums
        //             let pctm_16 = min_16 / tps * 100.;
        //             let pctm_128 = min_128 / tps * 100.;
        //             let pctm_1024 = min_1024 / tps * 100.;
        //
        //         log::trace!["[window]avg|min rate dpt:{d} \"{name}\":
        // [16]{avg_16:.2}|{min_16:.2} \
        // [128]{avg_128:.2}|{min_128:.2} \
        // [1024]{avg_1024:.2}|{min_1024:.2}
        // "];
        //         }
    }
}
