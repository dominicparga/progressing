//------------------------------------------------------------------------------------------------//
// other modules

use io::stdout;
use io::Write;
use std::cmp::min;
use std::io;

//------------------------------------------------------------------------------------------------//

pub trait Bar {
    /// Returns the printable progressbar.
    fn render(&self) -> String;

    fn write_to_stdout(&self, msg: &str) -> Result<(), String> {
        let mut output = stdout();
        match output.write(msg.as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(format!("{}", e)),
        };
        match output.flush() {
            Ok(_) => (),
            Err(e) => return Err(format!("{}", e)),
        };

        Ok(())
    }

    /// Progress is clipped to `[0, 1]`.
    ///
    /// Prints a progressbar using given progress.
    /// Does not print a newline-character.
    /// Use `println(...)` for printing a newline-character.
    /// Use `reprint(...)` for overwriting the current stdout-line.
    ///
    /// Returns error if writing to `stdout` throws an error.
    fn print(&self) -> Result<(), String> {
        let msg = format!("{}", self.render());
        self.write_to_stdout(msg.as_ref())
    }

    /// Progress is clipped to `[0, 1]`.
    ///
    /// Prints a progressbar using given progress.
    /// In additon to `print(...)`, this function prints a new line.
    /// Use `reprintln(...)` for overwriting the current stdout-line.
    ///
    /// Returns error if writing to `stdout` throws an error.
    fn println(&self) -> Result<(), String> {
        let msg = format!("{}\n", self.render());
        self.write_to_stdout(msg.as_ref())
    }

    /// Progress is clipped to `[0, 1]`.
    ///
    /// Prints the current line again with progressbar using given progress.
    /// Does not print a newline-character.
    /// Use `reprintln(...)` for reprinting with a newline-character.
    ///
    /// Returns error if writing to `stdout` throws an error.
    fn reprint(&self) -> Result<(), String> {
        let msg = format!("\r{}", self.render());
        self.write_to_stdout(msg.as_ref())
    }

    /// Progress is clipped to `[0, 1]`.
    ///
    /// Prints the current line again with progressbar using given progress.
    /// In additon to `reprint(...)`, this function prints a new line.
    /// Use `println(...)` for always printing a newline-character.
    ///
    /// Returns error if writing to `stdout` throws an error.
    fn reprintln(&self) -> Result<(), String> {
        let msg = format!("\r{}\n", self.render());
        self.write_to_stdout(msg.as_ref())
    }
}

//------------------------------------------------------------------------------------------------//
// default bar clipping to [0; 1]

/// Only optimized for single-length-strings, but strings are more handy than chars.
#[derive(Debug)]
pub struct ClippingBar {
    bar_len: usize,
    prefix: String,
    left_bracket: String,
    right_bracket: String,
    line: String,
    empty_line: String,
    hat: String,
    progress: f32,
}

impl Default for ClippingBar {
    fn default() -> Self {
        ClippingBar {
            bar_len: 72,
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

impl ClippingBar {
    pub fn new() -> ClippingBar {
        ClippingBar {
            ..Default::default()
        }
    }

    pub fn from(bar_len: usize, prefix: String) -> ClippingBar {
        ClippingBar {
            bar_len,
            prefix,
            ..Default::default()
        }
    }

    fn bar_len(&self) -> usize {
        self.bar_len
    }

    fn inner_bar_len(&self) -> usize {
        self.bar_len() - self.brackets_len()
    }

    fn brackets_len(&self) -> usize {
        self.left_bracket.len() + self.right_bracket.len()
    }

    pub fn update_progress(&mut self, mut new_progress: f32) -> &mut Self {
        if new_progress < 0.0 {
            new_progress = 0.0;
        }
        if 1.0 < new_progress {
            new_progress = 1.0;
        }
        self.progress = new_progress;
        self
    }

    //--------------------------------------------------------------------------------------------//
    // imitate trait-methods

    pub fn render_with(&mut self, new_progress: f32) -> String {
        self.update_progress(new_progress).render()
    }

    pub fn print_with(&mut self, new_progress: f32) -> Result<(), String> {
        self.update_progress(new_progress).print()
    }

    pub fn println_with(&mut self, new_progress: f32) -> Result<(), String> {
        self.update_progress(new_progress).println()
    }

    pub fn reprint_with(&mut self, new_progress: f32) -> Result<(), String> {
        self.update_progress(new_progress).reprint()
    }

    pub fn reprintln_with(&mut self, new_progress: f32) -> Result<(), String> {
        self.update_progress(new_progress).reprintln()
    }
}

impl Bar for ClippingBar {
    /// Progress is clipped to `[0, 1]`.
    fn render(&self) -> String {
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
        format!(
            "{}{}{}{}",
            self.prefix, self.left_bracket, bar, self.right_bracket
        )
    }
}

//------------------------------------------------------------------------------------------------//
// bar mapping [min, max] to [0, 1]

#[derive(Debug)]
pub struct MappingBar {
    bar: ClippingBar,
    k_min: i32,
    k_max: i32,
}

impl Default for MappingBar {
    fn default() -> Self {
        MappingBar {
            bar: ClippingBar::new(),
            k_min: 0,
            k_max: 100,
        }
    }
}

impl MappingBar {
    pub fn new() -> MappingBar {
        MappingBar {
            ..Default::default()
        }
    }

    pub fn from(k_min: i32, k_max: i32) -> MappingBar {
        if k_max < k_min {
            MappingBar {
                bar: ClippingBar::new(),
                k_min: k_max,
                k_max: k_min,
            }
        } else {
            MappingBar {
                bar: ClippingBar::new(),
                k_min,
                k_max,
            }
        }
    }

    pub fn from_all(bar: ClippingBar, k_min: i32, k_max: i32) -> MappingBar {
        if k_max < k_min {
            MappingBar {
                bar,
                k_min: k_max,
                k_max: k_min,
            }
        } else {
            MappingBar { bar, k_min, k_max }
        }
    }

    //--------------------------------------------------------------------------------------------//
    // imitate methods from ClippingBar

    pub fn update_progress(&mut self, mut k: i32) -> &mut Self {
        // clip to [0, 1]
        if k < self.k_min {
            k = self.k_min;
        }
        if self.k_max < k {
            k = self.k_max;
        }

        // calculate new progress
        let k_min = self.k_min as f32;
        let k_max = self.k_max as f32;
        let k = k as f32;
        self.bar.progress = (k - k_min) / (k_max - k_min);

        // return self
        self
    }

    pub fn render_with(&mut self, k: i32) -> String {
        self.update_progress(k).render()
    }

    pub fn print_with(&mut self, k: i32) -> Result<(), String> {
        self.update_progress(k).print()
    }

    pub fn println_with(&mut self, k: i32) -> Result<(), String> {
        self.update_progress(k).println()
    }

    pub fn reprint_with(&mut self, k: i32) -> Result<(), String> {
        self.update_progress(k).reprint()
    }

    pub fn reprintln_with(&mut self, k: i32) -> Result<(), String> {
        self.update_progress(k).reprintln()
    }
}

impl Bar for MappingBar {
    /// Progress is clipped to `[0, 1]`.
    fn render(&self) -> String {
        self.bar.render()
    }
}
