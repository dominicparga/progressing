//------------------------------------------------------------------------------------------------//
// other modules

mod bernoulli;
pub use bernoulli::BernoulliBar;
pub use bernoulli::BernoulliProgress;
mod clamping;
pub use clamping::ClampingBar;
mod mapping;
use io::stdout;
use io::Write;
pub use mapping::MappingBar;
use std::fmt;
use std::io;

//------------------------------------------------------------------------------------------------//

/// A trait describing basic functionality for simple text-based progress-bars.
///
///
/// # Mini-Examples
///
/// ```
/// use progressing::Bar;
///
/// /// Printing value 0.3 clamped to [0, 1]
/// /// [=====>------------]
/// fn clamped() -> Result<(), String> {
///     println!("Printing value 0.3 clamped to [0, 1]");
///     let mut progressbar = progressing::ClampingBar::new();
///     progressbar.set_bar_len(20);
///     progressbar.set(0.3).reprintln()
/// }
///
/// /// Mapping from [-9, 5] to [0, 1]
/// /// [================>-] (4 / 5)
/// fn mapped() -> Result<(), String> {
///     println!("Mapping from [-9, 5] to [0, 1]");
///     let mut progressbar = progressing::MappingBar::new(-9..=5);
///     progressbar.set_bar_len(20);
///     progressbar.set(4).reprintln()
/// }
///
/// /// Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)
/// /// [============>-----] (42 / 60 # 130)
/// fn bernoulli() -> Result<(), String> {
///     println!("Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)");
///     let mut progressbar = progressing::BernoulliBar::from_goal(60);
///     progressbar.set_bar_len(20);
///     progressbar.set((42, 130)).reprintln()
/// }
///
/// fn main() -> Result<(), String> {
///     clamped()?;
///     println!();
///     mapped()?;
///     println!();
///     bernoulli()?;
///
///     Ok(())
/// }
/// ```
pub trait Bar: fmt::Display {
    fn bar_len(&self) -> usize;

    /// Do not shorten the length before reprinting since the line will be overwritten, not cleared.
    ///
    /// `[========>-]` becomes `[====>]==>-]` instead of `[====>]     `.
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
