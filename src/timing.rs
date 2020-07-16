use crate::{bernoulli::Bar as BernoulliBar, clamping, mapping, Baring};
use std::{
    fmt::{self, Display},
    time::Instant,
};

#[derive(Debug)]
pub struct Bar<B>
where
    B: Baring,
{
    bar: B,
    now: Instant,
    is_remembering_progress: bool,
}

impl<B> Bar<B>
where
    B: Baring,
{
    pub(crate) fn with(bar: B) -> Bar<B> {
        Bar {
            bar,
            now: Instant::now(),
            is_remembering_progress: false,
        }
    }
}

impl<B> Baring for Bar<B>
where
    B: Baring,
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

    fn start(&self) -> Self::Progress {
        self.bar.start()
    }

    fn end(&self) -> Self::Progress {
        self.bar.end()
    }

    fn has_progressed_significantly(&self) -> bool {
        self.bar.has_progressed_significantly()
            || (!self.is_remembering_progress && (self.now.elapsed().as_millis() > 60_000))
    }

    fn remember_significant_progress(&mut self) {
        self.bar.remember_significant_progress();
        self.is_remembering_progress = true;
    }
}

//------------------------------------------------------------------------------------------------//
// displaying time

impl Bar<clamping::Bar> {
    fn approx_time(&self) -> String {
        let progress = self.progress();
        if progress > self.start() {
            let scale = (self.end() - progress) / progress;

            let elapsed_ms = self.now.elapsed().as_millis();
            let elapsed_s = elapsed_ms as f64 / 1_000.0;
            let mut elapsed = (elapsed_s * scale) as usize;
            let mut unit = "s";

            // update unit
            if elapsed > 3_600 {
                elapsed /= 3_600;
                unit = "h";
            } else if elapsed > 60 {
                elapsed /= 60;
                unit = "min";
            }

            format!("{} {}", elapsed, unit)
        } else {
            String::from("inf s")
        }
    }
}

impl Display for Bar<clamping::Bar> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ~ {}", self.bar, self.approx_time())
    }
}

impl Bar<mapping::Bar<usize>>
where
    mapping::Bar<usize>: Baring,
{
    fn approx_time(&self) -> String {
        let progress = self.progress();
        if progress > self.start() {
            let scale = ((self.end() - progress) as f64) / (progress as f64);

            let elapsed_ms = self.now.elapsed().as_millis();
            let elapsed_s = elapsed_ms as f64 / 1_000.0;
            let mut elapsed = (elapsed_s * scale) as usize;
            let mut unit = "s";

            // update unit
            if elapsed > 3_600 {
                elapsed /= 3_600;
                unit = "h";
            } else if elapsed > 60 {
                elapsed /= 60;
                unit = "min";
            }

            format!("{} {}", elapsed, unit)
        } else {
            String::from("inf s")
        }
    }
}

impl Display for Bar<mapping::Bar<usize>>
where
    mapping::Bar<usize>: Baring,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ~ {}", self.bar, self.approx_time())
    }
}

impl Bar<mapping::Bar<i64>>
where
    mapping::Bar<i64>: Baring,
{
    fn approx_time(&self) -> String {
        let progress = self.progress();
        if progress > self.start() {
            let scale = ((self.end() - progress) as f64) / (progress as f64);

            let elapsed_ms = self.now.elapsed().as_millis();
            let elapsed_s = elapsed_ms as f64 / 1_000.0;
            let mut elapsed = (elapsed_s * scale) as usize;
            let mut unit = "s";

            // update unit
            if elapsed > 3_600 {
                elapsed /= 3_600;
                unit = "h";
            } else if elapsed > 60 {
                elapsed /= 60;
                unit = "min";
            }

            format!("{} {}", elapsed, unit)
        } else {
            String::from("inf s")
        }
    }
}

impl Display for Bar<mapping::Bar<i64>>
where
    mapping::Bar<i64>: Baring,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ~ {}", self.bar, self.approx_time())
    }
}

impl Bar<mapping::Bar<i32>>
where
    mapping::Bar<i32>: Baring,
{
    fn approx_time(&self) -> String {
        let progress = self.progress();
        if progress > self.start() {
            let scale = ((self.end() - progress) as f64) / (progress as f64);

            let elapsed_ms = self.now.elapsed().as_millis();
            let elapsed_s = elapsed_ms as f64 / 1_000.0;
            let mut elapsed = (elapsed_s * scale) as usize;
            let mut unit = "s";

            // update unit
            if elapsed > 3_600 {
                elapsed /= 3_600;
                unit = "h";
            } else if elapsed > 60 {
                elapsed /= 60;
                unit = "min";
            }

            format!("{} {}", elapsed, unit)
        } else {
            String::from("inf s")
        }
    }
}

impl Display for Bar<mapping::Bar<i32>>
where
    mapping::Bar<i32>: Baring,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ~ {}", self.bar, self.approx_time())
    }
}

impl Bar<BernoulliBar>
where
    BernoulliBar: Baring,
{
    fn approx_time(&self) -> String {
        let progress = self.progress();
        if progress.successes > self.start().successes {
            let scale =
                ((self.end().successes - progress.successes) as f64) / (progress.successes as f64);

            let elapsed_ms = self.now.elapsed().as_millis();
            let elapsed_s = elapsed_ms as f64 / 1_000.0;
            let mut elapsed = (elapsed_s * scale) as usize;
            let mut unit = "s";

            // update unit
            if elapsed > 3_600 {
                elapsed /= 3_600;
                unit = "h";
            } else if elapsed > 60 {
                elapsed /= 60;
                unit = "min";
            }

            format!("{} {}", elapsed, unit)
        } else {
            String::from("inf s")
        }
    }
}

impl Display for Bar<BernoulliBar>
where
    BernoulliBar: Baring,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ~ {}", self.bar, self.approx_time())
    }
}
