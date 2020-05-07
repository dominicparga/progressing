use crate::Bar;
use log::warn;
use std::{cmp::min, fmt};

/// A progress-bar clamping values to `[0, 1]`.
///
///
/// # Mini-Example
///
/// ```
/// use progressing::Bar;
///
/// /// Printing value 0.3 clamped to [0, 1]
/// /// [=====>------------]
/// fn main() {
///     println!("Printing value 0.3 clamped to [0, 1]");
///     let mut progress_bar = progressing::ClampingBar::new();
///     progress_bar.set_len(20);
///     progress_bar.set(0.3);
///     println!("{}", progress_bar);
/// }
/// ```
///
/// It is only optimized for single-length-strings, but strings are more handy than chars and hence used as implementation.
#[derive(Debug)]
pub struct ClampingBar {
    bar_len: usize,
    style: String,
    progress: f64,
}

impl Default for ClampingBar {
    fn default() -> Self {
        ClampingBar {
            bar_len: 42,
            style: String::from("[=>-]"),
            progress: 0.0,
        }
    }
}

impl ClampingBar {
    pub fn new() -> ClampingBar {
        ClampingBar {
            ..Default::default()
        }
    }

    pub fn from(bar_len: usize) -> ClampingBar {
        ClampingBar {
            bar_len,
            ..Default::default()
        }
    }

    pub fn set_style<S>(&mut self, style: S)
    where
        S: Into<String>,
    {
        let style = style.into();

        if style.len() != 5 {
            warn!("The bar-style has to consist of 5 characters, e.g. [=>-]");
        };
        self.style = style;
    }

    fn inner_bar_len(&self) -> usize {
        self.len() - self.brackets_len()
    }

    fn brackets_len(&self) -> usize {
        self.left_bracket().len() + self.right_bracket().len()
    }

    fn left_bracket(&self) -> &str {
        &self.style[0..1]
    }

    fn line(&self) -> &str {
        &self.style[1..2]
    }

    fn hat(&self) -> &str {
        &self.style[2..3]
    }

    fn empty_line(&self) -> &str {
        &self.style[3..4]
    }

    fn right_bracket(&self) -> &str {
        &self.style[4..5]
    }
}

impl Bar for ClampingBar {
    type Progress = f64;

    fn len(&self) -> usize {
        self.bar_len
    }

    /// panics if length is `< 3`
    fn set_len(&mut self, new_bar_len: usize) {
        self.bar_len = new_bar_len;
    }

    fn progress(&self) -> f64 {
        self.progress
    }

    fn set<P>(&mut self, new_progress: P)
    where
        P: Into<f64>,
    {
        let mut new_progress = new_progress.into();

        if new_progress < 0.0 {
            new_progress = 0.0;
        }
        if 1.0 < new_progress {
            new_progress = 1.0;
        }
        self.progress = new_progress;
    }
}

impl fmt::Display for ClampingBar {
    /// Progress is clamped to `[0, 1]`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // calc progress
        // -> bar needs to be calculated
        // -> no brackets involved
        let reached = (self.progress * (self.inner_bar_len() as f64)) as usize;

        let line = self.line().repeat(reached);
        // crop hat if end of bar is reached
        let hat = &self.hat()[0..min(self.hat().len(), self.inner_bar_len() - reached)];
        // fill up rest with empty line
        let empty_line = self
            .empty_line()
            .repeat(self.inner_bar_len() - reached - hat.len());
        write!(f, "{}{}{}{}{}{}", self.len(), self.left_bracket(), line, hat, empty_line, self.right_bracket())
    }
}
