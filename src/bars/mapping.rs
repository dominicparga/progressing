//------------------------------------------------------------------------------------------------//
// other modules

use super::Bar;
use super::ClampingBar;
use std::fmt;
use std::ops;

//------------------------------------------------------------------------------------------------//
// own modules

pub fn inner_bar<N>(mapping_bar: &MappingBar<N>) -> &ClampingBar {
    &mapping_bar.bar
}

pub fn inner_k<N>(mapping_bar: &MappingBar<N>) -> &N {
    &mapping_bar.k
}

//------------------------------------------------------------------------------------------------//
// bar mapping [min, max] to [0, 1]

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
///     progress_bar.set_bar_len(20);
///     println!("{}", progress_bar.set(4));
/// }
/// ```
#[derive(Debug)]
pub struct MappingBar<N> {
    bar: ClampingBar,
    range: ops::RangeInclusive<N>,
    k: N,
}

impl<N> MappingBar<N>
where
    MappingBar<N>: Default,
{
    pub fn new(range: ops::RangeInclusive<N>) -> MappingBar<N> {
        MappingBar {
            range,
            ..Default::default()
        }
    }
}

//------------------------------------------------------------------------------------------------//
// impl for usize

impl Default for MappingBar<usize> {
    fn default() -> Self {
        MappingBar {
            bar: ClampingBar::default(),
            range: 0..=100,
            k: 0,
        }
    }
}

impl MappingBar<usize> {
    pub fn start(&self) -> usize {
        *(self.range.start())
    }

    pub fn end(&self) -> usize {
        *(self.range.end())
    }
}

impl Bar for MappingBar<usize> {
    type Progress = usize;

    fn bar_len(&self) -> usize {
        self.bar.bar_len()
    }

    fn set_bar_len(&mut self, new_bar_len: usize) {
        self.bar.set_bar_len(new_bar_len)
    }

    fn progress(&self) -> usize {
        self.k
    }

    fn set<P: Into<usize>>(&mut self, new_progress: P) -> &mut Self {
        let new_progress = new_progress.into();
        self.k = new_progress;

        // calculate new progress
        let k_min = self.start() as f32;
        let k_max = self.end() as f32;
        let k = new_progress as f32;
        self.bar.set((k - k_min) / (k_max - k_min));

        // return self
        self
    }

    fn add<P: Into<usize>>(&mut self, delta: P) -> &mut Self {
        self.set(self.k + delta.into())
    }
}

impl fmt::Display for MappingBar<usize> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} / {})", self.bar, self.k, self.end())
    }
}

//------------------------------------------------------------------------------------------------//
// impl for u32

impl Default for MappingBar<u32> {
    fn default() -> Self {
        MappingBar {
            bar: ClampingBar::default(),
            range: 0..=100,
            k: 0,
        }
    }
}

impl MappingBar<u32> {
    pub fn start(&self) -> u32 {
        *(self.range.start())
    }

    pub fn end(&self) -> u32 {
        *(self.range.end())
    }
}

impl Bar for MappingBar<u32> {
    type Progress = u32;

    fn bar_len(&self) -> usize {
        self.bar.bar_len()
    }

    fn set_bar_len(&mut self, new_bar_len: usize) {
        self.bar.set_bar_len(new_bar_len)
    }

    fn progress(&self) -> u32 {
        self.k
    }

    fn set<P: Into<u32>>(&mut self, new_progress: P) -> &mut Self {
        let new_progress = new_progress.into();
        self.k = new_progress;

        // calculate new progress
        let k_min = self.start() as f32;
        let k_max = self.end() as f32;
        let k = new_progress as f32;
        self.bar.set((k - k_min) / (k_max - k_min));

        // return self
        self
    }

    fn add<P: Into<u32>>(&mut self, delta: P) -> &mut Self {
        self.set(self.k + delta.into())
    }
}

impl fmt::Display for MappingBar<u32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} / {})", self.bar, self.k, self.end())
    }
}

//------------------------------------------------------------------------------------------------//
// impl for i32

impl Default for MappingBar<i32> {
    fn default() -> Self {
        MappingBar {
            bar: ClampingBar::default(),
            range: 0..=100,
            k: 0,
        }
    }
}

impl MappingBar<i32> {
    pub fn start(&self) -> i32 {
        *(self.range.start())
    }

    pub fn end(&self) -> i32 {
        *(self.range.end())
    }
}

impl Bar for MappingBar<i32> {
    type Progress = i32;

    fn bar_len(&self) -> usize {
        self.bar.bar_len()
    }

    fn set_bar_len(&mut self, new_bar_len: usize) {
        self.bar.set_bar_len(new_bar_len)
    }

    fn progress(&self) -> i32 {
        self.k
    }

    fn set<P: Into<i32>>(&mut self, new_progress: P) -> &mut Self {
        let new_progress = new_progress.into();
        self.k = new_progress;

        // calculate new progress
        let k_min = self.start() as f32;
        let k_max = self.end() as f32;
        let k = new_progress as f32;
        self.bar.set((k - k_min) / (k_max - k_min));

        // return self
        self
    }

    fn add<P: Into<i32>>(&mut self, delta: P) -> &mut Self {
        self.set(self.k + delta.into())
    }
}

impl fmt::Display for MappingBar<i32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} / {})", self.bar, self.k, self.end())
    }
}
