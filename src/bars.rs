//------------------------------------------------------------------------------------------------//
// other modules

use io::stdout;
use io::Write;
use std::cmp::min;
use std::fmt;
use std::io;
use std::ops;

//------------------------------------------------------------------------------------------------//

pub trait Bar: fmt::Display {
    fn bar_len(&self) -> usize;

    /// Do not shorten the length before reprinting since the line will be overwritten, not cleared.
    ///
    /// `[========> ]` becomes `[====>]==> ]` instead of `[====>]     `.
    fn set_bar_len(&mut self, new_bar_len: usize);

    /// Returns the printable progressbar.
    fn display(&self) -> String {
        format!("{}", self)
    }

    fn write_to_stdout<S: Into<String>>(&self, msg: S) -> Result<(), String> {
        let mut output = stdout();
        match output.write(msg.into().as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(format!("{}", e)),
        };
        match output.flush() {
            Ok(_) => (),
            Err(e) => return Err(format!("{}", e)),
        };

        Ok(())
    }

    /// Prints a progressbar using given progress.
    /// Does not print a newline-character.
    /// Use `println(...)` for printing a newline-character.
    /// Use `reprint(...)` for overwriting the current stdout-line.
    ///
    /// Will return error if writing to `stdout` throws an error.
    fn print(&self) -> Result<(), String> {
        self.write_to_stdout(format!("{}", self))
    }

    /// Prints a progressbar using given progress.
    /// In additon to `print(...)`, this function prints a new line.
    /// Use `reprintln(...)` for overwriting the current stdout-line.
    ///
    /// Will return error if writing to `stdout` throws an error.
    fn println(&self) -> Result<(), String> {
        self.write_to_stdout(format!("{}\n", self))
    }

    /// Prints the current line again with progressbar using given progress.
    /// Does not print a newline-character.
    /// Use `reprintln(...)` for reprinting with a newline-character.
    ///
    /// Will return error if writing to `stdout` throws an error.
    fn reprint(&self) -> Result<(), String> {
        self.write_to_stdout(format!("\r{}", self))
    }

    /// Prints the current line again with progressbar using given progress.
    /// In additon to `reprint(...)`, this function prints a new line.
    /// Use `println(...)` for always printing a newline-character.
    ///
    /// Will return error if writing to `stdout` throws an error.
    fn reprintln(&self) -> Result<(), String> {
        self.write_to_stdout(format!("\r{}\n", self))
    }

    //--------------------------------------------------------------------------------------------//

    type Progress;

    /// Sets the progress to the given value
    fn set(&mut self, new_progress: Self::Progress) -> &mut Self;

    /// Adds the given progress to the current progress
    fn add(&mut self, delta: Self::Progress) -> &mut Self;
}

//------------------------------------------------------------------------------------------------//
// default bar clamping to [0; 1]

/// Only optimized for single-length-strings, but strings are more handy than chars.
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
            empty_line: String::from(" "),
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

    fn set(&mut self, mut new_progress: f32) -> &mut Self {
        if new_progress < 0.0 {
            new_progress = 0.0;
        }
        if 1.0 < new_progress {
            new_progress = 1.0;
        }
        self.progress = new_progress;
        self
    }

    fn add(&mut self, delta: f32) -> &mut Self {
        self.set(self.progress + delta)
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

//------------------------------------------------------------------------------------------------//
// bar mapping [min, max] to [0, 1]

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
    fn start(&self) -> u32 {
        *(self.range.start())
    }

    fn end(&self) -> u32 {
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

    fn set(&mut self, new_progress: u32) -> &mut Self {
        self.k = new_progress;

        // calculate new progress
        let k_min = self.start() as f32;
        let k_max = self.end() as f32;
        let k = new_progress as f32;
        self.bar.set((k - k_min) / (k_max - k_min));

        // return self
        self
    }

    fn add(&mut self, delta: u32) -> &mut Self {
        self.set(self.k + delta)
    }
}

impl fmt::Display for MappingBar<u32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} / {})", self.bar, self.k, self.end())
    }
}

//------------------------------------------------------------------------------------------------//

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
    fn start(&self) -> i32 {
        *(self.range.start())
    }

    fn end(&self) -> i32 {
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

    fn set(&mut self, new_progress: i32) -> &mut Self {
        self.k = new_progress;

        // calculate new progress
        let k_min = self.start() as f32;
        let k_max = self.end() as f32;
        let k = new_progress as f32;
        self.bar.set((k - k_min) / (k_max - k_min));

        // return self
        self
    }

    fn add(&mut self, delta: i32) -> &mut Self {
        self.set(self.k + delta)
    }
}

impl fmt::Display for MappingBar<i32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} / {})", self.bar, self.k, self.end())
    }
}

//------------------------------------------------------------------------------------------------//
// bar counting success and failures

pub struct BernoulliProgress {
    pub successes: u32,
    pub attempts: u32,
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

//------------------------------------------------------------------------------------------------//

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

    fn set(&mut self, outcome: BernoulliProgress) -> &mut Self {
        self.bar.set(outcome.successes);
        self.attempts = outcome.attempts;
        self
    }

    fn add(&mut self, outcome: BernoulliProgress) -> &mut Self {
        let new_progress = BernoulliProgress {
            successes: self.bar.k + outcome.successes,
            attempts: self.attempts + outcome.attempts,
        };
        self.set(new_progress)
    }
}

impl fmt::Display for BernoulliBar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({} / {} # {})",
            self.bar.bar,
            self.bar.k,
            self.bar.end(),
            self.attempts
        )
    }
}
