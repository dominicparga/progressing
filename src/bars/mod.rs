mod bernoulli;
pub use bernoulli::{BernoulliBar, BernoulliProgress};
mod clamping;
pub use clamping::ClampingBar;
mod mapping;
pub use mapping::MappingBar;

use std::ops::Add;

/// A trait describing basic functionality for simple text-based progress-bars.
///
///
/// # Mini-Examples
///
/// ```
/// use progressing::Bar;
///
/// /// Printing value 0.3 clamped to [0, 1]
/// /// [=====>------------]
/// fn clamped() {
///     println!("Printing value 0.3 clamped to [0, 1]");
///     let mut progress_bar = progressing::ClampingBar::new();
///     progress_bar.set_len(20);
///     progress_bar.set(0.3);
///     println!("{}", progress_bar);
/// }
///
/// /// Mapping from [-9, 5] to [0, 1]
/// /// [================>-] (4 / 5)
/// fn mapped() {
///     println!("Mapping from [-9, 5] to [0, 1]");
///     let mut progress_bar = progressing::MappingBar::new(-9..=5);
///     progress_bar.set_len(20);
///     progress_bar.set(4);
///     println!("{}", progress_bar);
/// }
///
/// /// Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)
/// /// [============>-----] (42 / 60 # 130)
/// fn bernoulli() {
///     println!("Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)");
///     let mut progress_bar = progressing::BernoulliBar::from_goal(60);
///     progress_bar.set_len(20);
///     progress_bar.set((42, 130));
///     println!("{}", progress_bar);
/// }
///
/// fn main() {
///     clamped();
///     println!();
///     mapped();
///     println!();
///     bernoulli();
/// }
/// ```
pub trait Bar {
    type Progress: Progress + Add<Output = Self::Progress>;

    fn len(&self) -> usize;

    /// Do not shorten the length before reprinting ("\r") since the line will be overwritten, not cleared.
    ///
    /// `[========>-]` becomes `[====>]==>-]` instead of `[====>]     `.
    fn set_len(&mut self, new_bar_len: usize);

    fn progress(&self) -> Self::Progress;

    /// Sets the progress to the given value
    fn set<P>(&mut self, new_progress: P)
    where
        P: Into<Self::Progress>;

    /// Adds the given progress to the current progress
    fn add<P>(&mut self, delta: P)
    where
        P: Into<Self::Progress>,
    {
        self.set(self.progress() + delta.into());
    }
}

pub trait Progress<Rhs = Self> {
    fn div(self, divisor: Rhs) -> f64;
}

impl Progress for f64 {
    fn div(self, divisor: f64) -> f64 {
        self / divisor
    }
}

impl Progress for usize {
    fn div(self, divisor: usize) -> f64 {
        (self as f64) / (divisor as f64)
    }
}

impl Progress for u64 {
    fn div(self, divisor: u64) -> f64 {
        (self as f64) / (divisor as f64)
    }
}

impl Progress for u32 {
    fn div(self, divisor: u32) -> f64 {
        (self as f64) / (divisor as f64)
    }
}

impl Progress for i64 {
    fn div(self, divisor: i64) -> f64 {
        (self as f64) / (divisor as f64)
    }
}

impl Progress for i32 {
    fn div(self, divisor: i32) -> f64 {
        (self as f64) / (divisor as f64)
    }
}
