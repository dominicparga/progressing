//------------------------------------------------------------------------------------------------//
// other modules

mod mapping;
pub use mapping::MappingBar;
mod bernoulli;
pub use bernoulli::BernoulliBar;
pub use bernoulli::BernoulliProgress;
mod clamping;
pub use clamping::ClampingBar;
use io::stdout;
use io::Write;
use std::fmt;
use std::io;

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

    fn progress(&self) -> Self::Progress;

    /// Sets the progress to the given value
    fn set<P: Into<Self::Progress>>(&mut self, new_progress: P) -> &mut Self;

    /// Adds the given progress to the current progress
    fn add<P: Into<Self::Progress>>(&mut self, delta: P) -> &mut Self;
}
