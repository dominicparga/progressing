use crate::{Bar, ClampingBar, Progress};
use std::{
    fmt::{self, Display},
    ops::{Add, RangeInclusive, Sub},
};

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
///     let mut progress_bar = progressing::MappingBar::new(-9..=5);
///     progress_bar.set_len(20);
///     progress_bar.set(4);
///     println!("{}", progress_bar);
/// }
/// ```
#[derive(Debug)]
pub struct MappingBar<N> {
    bar: ClampingBar,
    range: RangeInclusive<N>,
    k: N,
}

impl<N> MappingBar<N>
where
    N: Copy,
{
    pub fn start(&self) -> N {
        *(self.range.start())
    }

    pub fn end(&self) -> N {
        *(self.range.end())
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
    pub fn new(range: RangeInclusive<N>) -> MappingBar<N> {
        MappingBar {
            bar: ClampingBar::new(),
            range,
            k: N::default(),
        }
    }
}

impl<N> Bar for MappingBar<N>
where
    N: Progress + Copy + Add<Output = N> + Sub<Output = N>,
{
    type Progress = N;

    fn len(&self) -> usize {
        self.bar.len()
    }

    fn set_len(&mut self, new_bar_len: usize) {
        self.bar.set_len(new_bar_len)
    }

    fn progress(&self) -> N {
        self.k
    }

    fn set<P>(&mut self, new_progress: P)
    where
        P: Into<N>,
    {
        let new_progress = new_progress.into();
        self.k = new_progress;

        // calculate new progress
        let k_min = self.start();
        let k_max = self.end();
        let k = new_progress;
        self.bar.set((k - k_min).div(k_max - k_min));
    }
}
