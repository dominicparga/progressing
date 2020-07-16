use crate::{clamping, timing, Baring};
use std::fmt::{self, Display};

pub struct Config<N> {
    pub bar_len: usize,
    pub style: String,
    pub interesting_progress_step: f64,
    pub min_k: N,
    pub max_k: N,
}

impl<N> Config<N> {
    pub fn with(min_k: N, max_k: N) -> Config<N> {
        // get defaults
        let cfg = clamping::Config::new();

        Config {
            bar_len: cfg.bar_len,
            style: cfg.style,
            interesting_progress_step: cfg.interesting_progress_step,
            min_k,
            max_k,
        }
    }
}

/// A progress-bar mapping values from `[a, b]` (e.g. `[-9, 5]`) to `[0, 1]`.
///
/// ```
/// use progressing::{mapping::Bar as MappingBar, Baring};
///
/// /// Mapping from [-9, 5] to [0, 1]
/// /// [================>-] (4 / 5)
/// fn main() {
///     println!("Mapping from [-9, 5] to [0, 1]");
///     let mut progress_bar = MappingBar::with_range(-9, 5);
///     progress_bar.set_len(20);
///     progress_bar.set(4);
///     println!("{}", progress_bar);
/// }
/// ```
#[derive(Debug)]
pub struct Bar<N> {
    pub(crate) bar: clamping::Bar,
    min_k: N,
    max_k: N,
    k: N,
}

impl<N> Display for Bar<N>
where
    N: Display,
    Bar<N>: Baring,
    <Bar<N> as Baring>::Progress: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}/{})", self.bar, self.k, self.end())
    }
}

impl<N> Bar<N>
where
    N: Clone,
{
    pub fn with_range(from: N, to: N) -> Bar<N> {
        Bar {
            bar: clamping::Bar::new(),
            min_k: from.clone(),
            max_k: to,
            k: from,
        }
    }

    pub fn with(cfg: Config<N>) -> Bar<N> {
        Bar {
            bar: clamping::Bar::with(clamping::Config {
                bar_len: cfg.bar_len,
                style: cfg.style,
                interesting_progress_step: cfg.interesting_progress_step,
            }),
            min_k: cfg.min_k.clone(),
            max_k: cfg.max_k,
            k: cfg.min_k,
        }
    }

    pub fn timed(self) -> timing::Bar<Bar<N>>
    where
        Bar<N>: Baring,
    {
        timing::Bar::with(self)
    }
}

impl Baring for Bar<usize> {
    type Progress = usize;

    fn len(&self) -> usize {
        self.bar.len()
    }

    fn set_len(&mut self, new_bar_len: usize) {
        self.bar.set_len(new_bar_len)
    }

    fn progress(&self) -> usize {
        self.k
    }

    fn set<P>(&mut self, new_progress: P)
    where
        P: Into<usize>,
    {
        let new_progress = new_progress.into();
        self.k = new_progress;

        // calculate new progress
        let delta = new_progress - self.start();
        let max_delta = self.end() - self.start();
        self.bar.set(delta as f64 / (max_delta as f64));
    }

    fn start(&self) -> usize {
        self.min_k
    }

    fn end(&self) -> usize {
        self.max_k
    }

    fn has_progressed_significantly(&self) -> bool {
        self.bar.has_progressed_significantly()
    }

    fn remember_significant_progress(&mut self) {
        self.bar.remember_significant_progress()
    }
}

impl Baring for Bar<i64> {
    type Progress = i64;

    fn len(&self) -> usize {
        self.bar.len()
    }

    fn set_len(&mut self, new_bar_len: usize) {
        self.bar.set_len(new_bar_len)
    }

    fn progress(&self) -> i64 {
        self.k
    }

    fn set<P>(&mut self, new_progress: P)
    where
        P: Into<i64>,
    {
        let new_progress = new_progress.into();
        self.k = new_progress;

        // calculate new progress
        // guaranteed to be positive or 0
        let delta = (new_progress - self.start()) as usize;
        let max_delta = (self.end() - self.start()) as usize;
        self.bar.set(delta as f64 / (max_delta as f64));
    }

    fn start(&self) -> i64 {
        self.min_k
    }

    fn end(&self) -> i64 {
        self.max_k
    }

    fn has_progressed_significantly(&self) -> bool {
        self.bar.has_progressed_significantly()
    }

    fn remember_significant_progress(&mut self) {
        self.bar.remember_significant_progress()
    }
}

impl Baring for Bar<i32> {
    type Progress = i32;

    fn len(&self) -> usize {
        self.bar.len()
    }

    fn set_len(&mut self, new_bar_len: usize) {
        self.bar.set_len(new_bar_len)
    }

    fn progress(&self) -> i32 {
        self.k
    }

    fn set<P>(&mut self, new_progress: P)
    where
        P: Into<i32>,
    {
        let new_progress = new_progress.into();
        self.k = new_progress;

        // calculate new progress
        // guaranteed to be positive or 0
        let delta = (new_progress - self.start()) as usize;
        let max_delta = (self.end() - self.start()) as usize;
        self.bar.set(delta as f64 / (max_delta as f64));
    }

    fn start(&self) -> i32 {
        self.min_k
    }

    fn end(&self) -> i32 {
        self.max_k
    }

    fn has_progressed_significantly(&self) -> bool {
        self.bar.has_progressed_significantly()
    }

    fn remember_significant_progress(&mut self) {
        self.bar.remember_significant_progress()
    }
}
