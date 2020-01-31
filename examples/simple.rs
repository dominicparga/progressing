use progressing;
use progressing::Bar;

/// Printing value 0.3 clamped to [0, 1]
/// [=====>------------]
fn clamped() -> Result<(), String> {
    println!("Printing value 0.3 clamped to [0, 1]");
    let mut progressbar = progressing::ClampingBar::new();
    progressbar.set_bar_len(20);
    progressbar.set(0.3).reprintln()
}

/// Mapping from [-9, 5] to [0, 1]
/// [================>-] (4 / 5)
fn mapped() -> Result<(), String> {
    println!("Mapping from [-9, 5] to [0, 1]");
    let mut progressbar = progressing::MappingBar::new(-9..=5);
    progressbar.set_bar_len(20);
    progressbar.set(4).reprintln()
}

/// Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)
/// [============>-----] (42 / 60 # 130)
fn bernoulli() -> Result<(), String> {
    println!("Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)");
    let mut progressbar = progressing::BernoulliBar::from_goal(60);
    progressbar.set_bar_len(20);
    progressbar.set((42, 130)).reprintln()
}

fn main() -> Result<(), String> {
    clamped()?;
    println!();
    mapped()?;
    println!();
    bernoulli()?;

    Ok(())
}
