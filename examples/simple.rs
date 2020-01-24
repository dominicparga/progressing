use progressing;
use progressing::Bar;

fn clamped() -> Result<(), String> {
    println!("Printing value 0.3 clamped to [0, 1]");
    let mut progressbar = progressing::ClampingBar::new();
    progressbar.set_bar_len(20);
    progressbar.set(0.3).reprintln()
}

fn mapped() -> Result<(), String> {
    println!("Mapping from [-9, 5] to [0, 1]");
    let mut progressbar = progressing::MappingBar::new(-9..=5);
    progressbar.set_bar_len(20);
    progressbar.set(4).reprintln()
}

fn bernoulli() -> Result<(), String> {
    println!("Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)");
    let mut progressbar = progressing::BernoulliBar::from_goal(60);
    progressbar.set_bar_len(20);
    progressbar.set((42, 130).into()).reprintln()
}

fn main() -> Result<(), String> {
    clamped()?;
    mapped()?;
    bernoulli()?;

    Ok(())
}
