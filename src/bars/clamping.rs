//------------------------------------------------------------------------------------------------//
// other modules

use super::Bar;
use std::cmp::min;
use std::fmt;

//------------------------------------------------------------------------------------------------//
// default bar clamping to [0; 1]

/// A progressbar clamping values to `[0, 1]`.
///
///
/// # Mini-Example
///
/// ```
/// use progressing::Bar;
///
/// /// Printing value 0.3 clamped to [0, 1]
/// /// [=====>------------]
/// fn main() -> Result<(), String> {
///     println!("Printing value 0.3 clamped to [0, 1]");
///     let mut progressbar = progressing::ClampingBar::new();
///     progressbar.set_bar_len(20);
///     progressbar.set(0.3).reprintln()
/// }
/// ```
///
/// It is only optimized for single-length-strings, but strings are more handy than chars and hence used as implementation.
#[derive(Debug)]
pub struct ClampingBar {
    bar_len: usize,
    prefix: String,
    left_bracket: String,
    right_bracket: String,
    line: String,
    empty_line: String,
    hat: String,
    progress: f32,
}

impl Default for ClampingBar {
    fn default() -> Self {
        ClampingBar {
            bar_len: 42,
            prefix: String::from(""),
            left_bracket: String::from("["),
            right_bracket: String::from("]"),
            line: String::from("="),
            empty_line: String::from("-"),
            hat: String::from(">"),
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

    pub fn from(bar_len: usize, prefix: String) -> ClampingBar {
        ClampingBar {
            bar_len,
            prefix,
            ..Default::default()
        }
    }

    fn inner_bar_len(&self) -> usize {
        self.bar_len() - self.brackets_len()
    }

    fn brackets_len(&self) -> usize {
        self.left_bracket.len() + self.right_bracket.len()
    }
}

impl Bar for ClampingBar {
    type Progress = f32;

    fn bar_len(&self) -> usize {
        self.bar_len
    }

    /// panics if length is `< 3`
    fn set_bar_len(&mut self, new_bar_len: usize) {
        assert!(new_bar_len > self.brackets_len());
        self.bar_len = new_bar_len;
    }

    fn progress(&self) -> f32 {
        self.progress
    }

    fn set<P: Into<f32>>(&mut self, new_progress: P) -> &mut Self {
        let mut new_progress = new_progress.into();

        if new_progress < 0.0 {
            new_progress = 0.0;
        }
        if 1.0 < new_progress {
            new_progress = 1.0;
        }
        self.progress = new_progress;
        self
    }

    fn add<P: Into<f32>>(&mut self, delta: P) -> &mut Self {
        self.set(self.progress + delta.into())
    }
}

impl fmt::Display for ClampingBar {
    /// Progress is clamped to `[0, 1]`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // calc progress
        // -> bar needs to be calculated
        // -> no brackets involved
        let reached = (self.progress * (self.inner_bar_len() as f32)) as usize;

        let line = self.line.repeat(reached);
        // crop hat if end of bar is reached
        let hat = &self.hat[0..min(self.hat.len(), self.inner_bar_len() - reached)];
        // fill up rest with empty line
        let empty_line = self
            .empty_line
            .repeat(self.inner_bar_len() - reached - hat.len());
        let bar = format!("{}{}{}", line, hat, empty_line);
        write!(
            f,
            "{}{}{}{}",
            self.prefix, self.left_bracket, bar, self.right_bracket
        )
    }
}
