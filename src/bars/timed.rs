use crate::Bar;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct TimedBar<B>
where
    B: Bar,
{
    bar: B,
}

impl<B> TimedBar<B>
where
    B: Bar,
{
    pub fn new(bar: B) -> TimedBar<B> {
        TimedBar { bar }
    }
}

impl<B> Display for TimedBar<B>
where
    B: Bar + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ~ {} {}", self.bar, 3, "s")
    }
}

impl<B> Bar for TimedBar<B>
where
    B: Bar,
{
    type Progress = B::Progress;

    fn len(&self) -> usize {
        self.bar.len()
    }

    /// panics if length is `< 3`
    fn set_len(&mut self, new_bar_len: usize) {
        self.bar.set_len(new_bar_len);
    }

    fn progress(&self) -> Self::Progress {
        self.bar.progress()
    }

    fn set<P>(&mut self, new_progress: P)
    where
        P: Into<Self::Progress>,
    {
        self.bar.set(new_progress)
    }
}
