mod bernoulli;
pub use bernoulli::{BernoulliBar, BernoulliProgress};
mod clamping;
pub use clamping::ClampingBar;
mod mapping;
pub use mapping::MappingBar;

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
///     progress_bar.set_bar_len(20);
///     println!("{}", progress_bar.set(0.3));
/// }
///
/// /// Mapping from [-9, 5] to [0, 1]
/// /// [================>-] (4 / 5)
/// fn mapped() {
///     println!("Mapping from [-9, 5] to [0, 1]");
///     let mut progress_bar = progressing::MappingBar::new(-9..=5);
///     progress_bar.set_bar_len(20);
///     println!("{}", progress_bar.set(4));
/// }
///
/// /// Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)
/// /// [============>-----] (42 / 60 # 130)
/// fn bernoulli() {
///     println!("Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)");
///     let mut progress_bar = progressing::BernoulliBar::from_goal(60);
///     progress_bar.set_bar_len(20);
///     println!("{}", progress_bar.set((42, 130)));
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
    fn bar_len(&self) -> usize;

    /// Do not shorten the length before reprinting since the line will be overwritten, not cleared.
    ///
    /// `[========>-]` becomes `[====>]==>-]` instead of `[====>]     `.
    fn set_bar_len(&mut self, new_bar_len: usize);

    type Progress;

    fn progress(&self) -> Self::Progress;

    /// Sets the progress to the given value
    fn set<P: Into<Self::Progress>>(&mut self, new_progress: P) -> &mut Self;

    /// Adds the given progress to the current progress
    fn add<P: Into<Self::Progress>>(&mut self, delta: P) -> &mut Self;
}
