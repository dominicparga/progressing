use crate::{Bar, BernoulliBar, ClampingBar, MappingBar};
use std::{
    fmt::{self, Display},
    time::Instant,
};

#[derive(Debug)]
pub struct TimedBar<B>
where
    B: Bar,
{
    bar: B,
    now: Instant,
    is_remembering_progress: bool,
}

impl<B> TimedBar<B>
where
    B: Bar,
{
    pub fn new(bar: B) -> TimedBar<B> {
        TimedBar {
            bar,
            now: Instant::now(),
            is_remembering_progress: false,
        }
    }
}

impl<B> Bar for TimedBar<B>
where
    B: Bar,
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

    fn has_progressed_much(&self) -> bool {
        self.bar.has_progressed_much()
            || (!self.is_remembering_progress && (self.now.elapsed().as_millis() > 60_000))
    }

    fn remember_progress(&mut self) {
        self.bar.remember_progress();
        self.is_remembering_progress = true;
    }
}

//------------------------------------------------------------------------------------------------//
// displaying time

impl TimedBar<ClampingBar> {
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

impl Display for TimedBar<ClampingBar> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ~ {}", self.bar, self.approx_time())
    }
}

impl TimedBar<MappingBar<usize>>
where
    MappingBar<usize>: Bar,
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

impl Display for TimedBar<MappingBar<usize>>
where
    MappingBar<usize>: Bar,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ~ {}", self.bar, self.approx_time())
    }
}

impl TimedBar<MappingBar<i64>>
where
    MappingBar<i64>: Bar,
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

impl Display for TimedBar<MappingBar<i64>>
where
    MappingBar<i64>: Bar,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ~ {}", self.bar, self.approx_time())
    }
}

impl TimedBar<MappingBar<i32>>
where
    MappingBar<i32>: Bar,
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

impl Display for TimedBar<MappingBar<i32>>
where
    MappingBar<i32>: Bar,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ~ {}", self.bar, self.approx_time())
    }
}

impl TimedBar<BernoulliBar>
where
    BernoulliBar: Bar,
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

impl Display for TimedBar<BernoulliBar>
where
    BernoulliBar: Bar,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ~ {}", self.bar, self.approx_time())
    }
}
