use crate::{mapping, timing, Baring};
use std::{
    fmt::{self, Display},
    ops::{Add, AddAssign, Div, Sub},
};

pub struct Config {}

/// A progress-bar counting successes (e.g. `42 out of 60`) and respective attempts (e.g. `130`).
///
/// # Mini-Example
///
/// ```
/// use progressing::{Baring, bernoulli::Bar as BernoulliBar};
///
/// /// Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)
/// /// [============>-----] (42 / 60 # 130)
/// fn main() {
///     println!("Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)");
///     let mut progress_bar = BernoulliBar::with_goal(60);
///     progress_bar.set_len(20);
///     progress_bar.set((42, 130));
///     println!("{}", progress_bar);
/// }
/// ```
#[derive(Debug)]
pub struct Bar {
    bar: mapping::Bar<usize>,
    attempts: usize,
}

impl Bar {
    pub fn with_goal(end: usize) -> Bar {
        Bar {
            bar: mapping::Bar::with_range(0, end),
            attempts: 0,
        }
    }

    pub fn timed(self) -> timing::Bar<Bar> {
        timing::Bar::with(self)
    }
}

impl Baring for Bar {
    type Progress = Progress;

    fn len(&self) -> usize {
        self.bar.len()
    }

    fn set_len(&mut self, new_bar_len: usize) {
        self.bar.set_len(new_bar_len)
    }

    fn progress(&self) -> Progress {
        Progress {
            successes: self.bar.progress(),
            attempts: self.attempts,
        }
    }

    fn set<P>(&mut self, outcome: P)
    where
        P: Into<Progress>,
    {
        let outcome = outcome.into();
        self.bar.set(outcome.successes);
        self.attempts = outcome.attempts;
    }

    fn start(&self) -> Progress {
        Progress {
            successes: self.bar.start(),
            attempts: 0,
        }
    }

    fn end(&self) -> Progress {
        Progress {
            successes: self.bar.end(),
            attempts: 1,
        }
    }

    fn has_progressed_significantly(&self) -> bool {
        self.bar.has_progressed_significantly()
    }

    fn remember_significant_progress(&mut self) {
        self.bar.remember_significant_progress()
    }
}

impl Display for Bar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} #{}", self.bar, self.attempts)
    }
}

/// Just a simple struct capsuling access to successes and attempts.
#[derive(Copy, Clone)]
pub struct Progress {
    pub successes: usize,
    pub attempts: usize,
}

impl From<(usize, usize)> for Progress {
    fn from((successes, attempts): (usize, usize)) -> Progress {
        Progress {
            successes,
            attempts,
        }
    }
}

impl From<usize> for Progress {
    fn from(successes: usize) -> Progress {
        Progress {
            successes,
            attempts: successes,
        }
    }
}

impl From<bool> for Progress {
    fn from(is_successful: bool) -> Progress {
        Progress {
            successes: if is_successful { 1 } else { 0 },
            attempts: 1,
        }
    }
}

impl Add for Progress {
    type Output = Progress;

    fn add(self, other: Progress) -> Progress {
        Progress {
            successes: self.successes + other.successes,
            attempts: self.attempts + other.attempts,
        }
    }
}

impl AddAssign for Progress {
    fn add_assign(&mut self, other: Progress) {
        *self = *self + other;
    }
}

impl Sub for Progress {
    type Output = Progress;

    fn sub(self, subtrahend: Progress) -> Progress {
        Progress {
            successes: self.successes - subtrahend.successes,
            attempts: self.attempts - subtrahend.attempts,
        }
    }
}

impl Div for Progress {
    type Output = f64;

    fn div(self, dividend: Progress) -> f64 {
        self.successes as f64 / (dividend.successes as f64)
    }
}
