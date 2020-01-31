//------------------------------------------------------------------------------------------------//
// other modules

use super::mapping;
use super::Bar;
use super::MappingBar;
use std::fmt;
use std::ops;

//------------------------------------------------------------------------------------------------//
// bar counting success and failures

#[derive(Copy, Clone)]
pub struct BernoulliProgress {
    pub successes: u32,
    pub attempts: u32,
}

impl BernoulliProgress {
    pub fn new() -> Self {
        BernoulliProgress {
            successes: 0,
            attempts: 0,
        }
    }
}

impl From<(u32, u32)> for BernoulliProgress {
    fn from((successes, attempts): (u32, u32)) -> Self {
        BernoulliProgress {
            successes,
            attempts,
        }
    }
}

impl From<u32> for BernoulliProgress {
    fn from(successes: u32) -> Self {
        BernoulliProgress {
            successes,
            attempts: successes,
        }
    }
}

impl From<bool> for BernoulliProgress {
    fn from(is_successful: bool) -> Self {
        let successes = if is_successful { 1 } else { 0 };
        BernoulliProgress {
            successes,
            attempts: 1,
        }
    }
}

impl ops::Add for BernoulliProgress {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            successes: self.successes + other.successes,
            attempts: self.attempts + other.attempts,
        }
    }
}

impl ops::AddAssign for BernoulliProgress {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

//------------------------------------------------------------------------------------------------//

/// Examples
///
/// ```
/// // Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)
/// // [============>     ] (42 / 60 # 130)
/// let mut progressbar = progressing::BernoulliBar::from_goal(60);
/// progressbar.set_bar_len(20);
/// progressbar.set((42, 130)).reprintln()
/// ```
#[derive(Debug)]
pub struct BernoulliBar {
    bar: MappingBar<u32>,
    attempts: u32,
}

impl Default for BernoulliBar {
    fn default() -> Self {
        BernoulliBar {
            bar: MappingBar::default(),
            attempts: 0,
        }
    }
}

impl BernoulliBar {
    pub fn from_goal(n: u32) -> BernoulliBar {
        BernoulliBar {
            bar: MappingBar::new(0..=n),
            ..Default::default()
        }
    }
}

impl Bar for BernoulliBar {
    type Progress = BernoulliProgress;

    fn bar_len(&self) -> usize {
        self.bar.bar_len()
    }

    fn set_bar_len(&mut self, new_bar_len: usize) {
        self.bar.set_bar_len(new_bar_len)
    }

    fn progress(&self) -> BernoulliProgress {
        BernoulliProgress {
            successes: self.bar.progress(),
            attempts: self.attempts,
        }
    }

    fn set<P: Into<BernoulliProgress>>(&mut self, outcome: P) -> &mut Self {
        let outcome = outcome.into();
        self.bar.set(outcome.successes);
        self.attempts = outcome.attempts;
        self
    }

    fn add<P: Into<BernoulliProgress>>(&mut self, outcome: P) -> &mut Self {
        self.set(self.progress() + outcome.into())
    }
}

impl fmt::Display for BernoulliBar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({} / {} # {})",
            mapping::inner_bar(&self.bar),
            mapping::inner_k(&self.bar),
            self.bar.end(),
            self.attempts
        )
    }
}
