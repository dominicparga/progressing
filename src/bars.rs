//------------------------------------------------------------------------------------------------//
// other modules

use io::stdout;
use io::Write;
use std::cmp::min;
use std::io;

//------------------------------------------------------------------------------------------------//
// default bar clipping to [0; 1]

/// Only optimized for single-length-strings, but strings are more handy than chars.
#[derive(Debug)]
pub struct Bar {
    bar_len: usize,
    prefix: String,
    suffix: String,
    left_bracket: String,
    right_bracket: String,
    line: String,
    empty_line: String,
    hat: String,
}

impl Default for Bar {
    fn default() -> Self {
        Bar {
            bar_len: 72,
            prefix: String::from(""),
            suffix: String::from(""),
            left_bracket: String::from("["),
            right_bracket: String::from("]"),
            line: String::from("="),
            empty_line: String::from(" "),
            hat: String::from(">"),
        }
    }
}

impl Bar {
    pub fn new() -> Bar {
        Bar {
            ..Default::default()
        }
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn set_prefix<S: Into<String>>(&mut self, new_prefix: S) {
        self.prefix = new_prefix.into();
    }

    pub fn suffix(&self) -> &str {
        &self.suffix
    }

    pub fn set_suffix<S: Into<String>>(&mut self, new_suffix: S) {
        self.suffix = new_suffix.into();
    }

    pub fn bar_len(&self) -> usize {
        self.bar_len
    }

    pub fn set_bar_len(&mut self, new_bar_len: usize) {
        self.bar_len = new_bar_len;
    }

    fn inner_bar_len(&self) -> usize {
        self.bar_len() - self.brackets_len()
    }

    fn brackets_len(&self) -> usize {
        self.left_bracket.len() + self.right_bracket.len()
    }

    //--------------------------------------------------------------------------------------------//
    // print to stdout

    /// Progress is clipped to `[0, 1]`.
    ///
    /// Returns the printable progressbar.
    fn render(&self, progress: f32) -> String {
        // calc progress
        // -> bar needs to be calculated
        // -> no brackets involved
        let reached = {
            let reached = (progress * (self.inner_bar_len() as f32)) as usize;
            min(self.inner_bar_len(), reached)
        };

        let line = self.line.repeat(reached);
        // crop hat if end of bar is reached
        let hat = &self.hat[0..min(self.hat.len(), self.inner_bar_len() - reached)];
        // fill up rest with empty line
        let empty_line = self
            .empty_line
            .repeat(self.inner_bar_len() - reached - hat.len());
        let bar = format!("{}{}{}", line, hat, empty_line);
        format!(
            "{}{}{}{}{}",
            self.prefix, self.left_bracket, bar, self.right_bracket, self.suffix
        )
    }

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
    pub fn print(&self, progress: f32) -> Result<(), String> {
        let msg = format!("{}", self.render(progress));
        self.write_to_stdout(msg.as_ref())
    }

    /// Progress is clipped to `[0, 1]`.
    ///
    /// Prints a progressbar using given progress.
    /// In additon to `print(...)`, this function prints a new line.
    /// Use `reprintln(...)` for overwriting the current stdout-line.
    ///
    /// Returns error if writing to `stdout` throws an error.
    pub fn println(&self, progress: f32) -> Result<(), String> {
        let msg = format!("{}\n", self.render(progress));
        self.write_to_stdout(msg.as_ref())
    }

    /// Progress is clipped to `[0, 1]`.
    ///
    /// Prints the current line again with progressbar using given progress.
    /// Does not print a newline-character.
    /// Use `reprintln(...)` for reprinting with a newline-character.
    ///
    /// Returns error if writing to `stdout` throws an error.
    pub fn reprint(&self, progress: f32) -> Result<(), String> {
        let msg = format!("\r{}", self.render(progress));
        self.write_to_stdout(msg.as_ref())
    }

    /// Progress is clipped to `[0, 1]`.
    ///
    /// Prints the current line again with progressbar using given progress.
    /// In additon to `reprint(...)`, this function prints a new line.
    /// Use `println(...)` for always printing a newline-character.
    ///
    /// Returns error if writing to `stdout` throws an error.
    pub fn reprintln(&self, progress: f32) -> Result<(), String> {
        let msg = format!("\r{}\n", self.render(progress));
        self.write_to_stdout(msg.as_ref())
    }
}
