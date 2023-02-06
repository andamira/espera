// espera::sleeper
//
//!
//

use spin_sleep::{SpinSleeper, SpinStrategy};

use crate::{Duration, Instant};

/// Sleep control structure.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sleeper {
    //
    sleeper: SpinSleeper,
    // /// Accuracy in nanoseconds, of the native sleep function.
    // ///
    // /// The maximum supported accuracy is 4294 ms for [`u32::MAX`] nanoseconds.
    // accuracy: u32,
}

// impl Default for Sleeper {
//     fn default() -> Self {
//         Self {
//             sleeper: SpinSleeper::default(),
//             // accuracy: 100_000,
//         }
//     }
// }

impl Sleeper {
    /// Returns a new sleeper with the provided accuracy
    ///
    /// # Arguments
    /// - `accuracy`: the accuracy of native sleep, in nanoseconds.
    /// - `do_spin`: if true, after native sleep spin loops up to its accuracy.
    pub fn new(accuracy: u32, do_spin: bool) -> Self {
        Self {
            sleeper: Self::new_inner_sleeper(accuracy, do_spin),
            // accuracy,
        }
    }

    /// Sleeps for a given positive `duration`.
    ///
    /// Does nothing if duration is not positive.
    pub fn sleep(&self, duration: Duration) {
        if duration.is_positive() {
            self.sleeper.sleep(duration.unsigned_abs());
        }
    }

    /// Returns the accuracy of the native yielding sleep method.
    pub fn accuracy(&self) -> Duration {
        return Duration::nanoseconds(self.sleeper.native_accuracy_ns().into());

        #[cfg(feature = "wasm")]
        todo![]
    }
    /// Returns the accuracy of the native yielding sleep method, in nanoseconds.
    pub fn accuracy_ns(&self) -> u32 {
        return self.sleeper.native_accuracy_ns();

        #[cfg(feature = "wasm")]
        todo![]
    }

    // Convenience constructor for the platform-dependant inner sleeper.
    fn new_inner_sleeper(accuracy: u32, do_spin: bool) -> SpinSleeper {
        if do_spin {
            SpinSleeper::new(accuracy).with_spin_strategy(SpinStrategy::SpinLoopHint)
        } else {
            SpinSleeper::new(accuracy).with_spin_strategy(SpinStrategy::YieldThread)
        }
    }
}

impl Sleeper {
    /// Measures the accuracy of native sleep by averaging multiple samples.
    ///
    /// ## Arguments
    /// num_samples: the number of samples needed to calculate the mean duration.
    /// extra_nanos: the extra nanoseconds to add to the final value.
    #[inline]
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn calculate_accuracy(&mut self, num_samples: u32, extra_nanos: u32) {
        let mut durations = vec![];

        for _ in 0..num_samples {
            let d = Self::sample_sleep_accuracy();
            durations.push(d.whole_nanoseconds() as u32);
        }
        let mean_accuracy = Self::mean(durations.as_slice());

        self.sleeper = Self::new_inner_sleeper(mean_accuracy.saturating_add(extra_nanos), true);
    }

    // Returns the real time duration passed after trying to sleep the minimum
    // amount of time possible, using native sleep function.
    #[inline]
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    fn sample_sleep_accuracy() -> Duration {
        let start = Instant::now();
        std::thread::sleep(Duration::MICROSECOND.unsigned_abs());
        let end = Instant::now();
        end - start
    }

    // Calculates the mean of a slice of u32 values.
    #[inline]
    fn mean(list: &[u32]) -> u32 {
        let sum: u32 = Iterator::sum(list.iter());
        (f64::from(sum) / (list.len() as f64)) as u32
    }

    /// Returns the size of the type, in bytes.
    #[inline]
    pub fn size() -> usize {
        core::mem::size_of::<Self>()
    }
}
