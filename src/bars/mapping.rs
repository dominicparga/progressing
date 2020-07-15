use crate::{Bar, ClampingBar};
use kissunits::quantity::Promille;
use std::fmt::{self, Display};

pub fn inner_bar<N>(mapping_bar: &MappingBar<N>) -> &ClampingBar {
    &mapping_bar.bar
}

pub fn inner_k<N>(mapping_bar: &MappingBar<N>) -> &N {
    &mapping_bar.k
}

/// A progress-bar mapping values from `[a, b]` (e.g. `[-9, 5]`) to `[0, 1]`.
///
/// ```
/// use progressing::Bar;
///
/// /// Mapping from [-9, 5] to [0, 1]
/// /// [================>-] (4 / 5)
/// fn main() {
///     println!("Mapping from [-9, 5] to [0, 1]");
///     let mut progress_bar = progressing::MappingBar::new(-9, 5);
///     progress_bar.set_len(20);
///     progress_bar.set(4);
///     println!("{}", progress_bar);
/// }
/// ```
#[derive(Debug)]
pub struct MappingBar<N> {
    bar: ClampingBar,
    min_k: N,
    max_k: N,
    k: N,
}

impl<N> MappingBar<N>
where
    N: Copy,
{
    pub fn start(&self) -> N {
        self.min_k
    }

    pub fn end(&self) -> N {
        self.max_k
    }
}

impl<N> Display for MappingBar<N>
where
    N: Display + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} / {})", self.bar, self.k, self.end())
    }
}

impl<N> MappingBar<N>
where
    N: Default,
{
    pub fn new(min_k: N, max_k: N) -> MappingBar<N> {
        MappingBar {
            bar: ClampingBar::new(),
            min_k: min_k,
            max_k: max_k,
            k: N::default(),
        }
    }
}

impl Bar for MappingBar<usize> {
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
        self.bar.set(Promille::from_div(delta, max_delta));
    }
}

impl Bar for MappingBar<i64> {
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
        self.bar.set(Promille::from_div(delta, max_delta));
    }
}

impl Bar for MappingBar<i32> {
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
        self.bar.set(Promille::from_div(delta, max_delta));
    }
}
