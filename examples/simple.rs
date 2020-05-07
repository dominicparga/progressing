use progressing::{self, Bar};

fn main() {
    // different examples for different use-cases
    clamped();
    println!();
    mapped();
    println!();
    bernoulli();
}

/// Printing value 0.3 clamped to [0, 1]
/// [=====>------------]
fn clamped() {
    println!("Printing value 0.3 clamped to [0, 1]");
    let mut progress_bar = progressing::ClampingBar::new();
    progress_bar.set_bar_len(20);
    println!("{}", progress_bar.set(0.3));
}

/// Mapping from [-9, 5] to [0, 1]
/// [================>-] (4 / 5)
fn mapped() {
    println!("Mapping from [-9, 5] to [0, 1]");
    let mut progress_bar = progressing::MappingBar::new(-9..=5);
    progress_bar.set_bar_len(20);
    println!("{}", progress_bar.set(4));
}

/// Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)
/// [============>-----] (42 / 60 # 130)
fn bernoulli() {
    println!("Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)");
    let mut progress_bar = progressing::BernoulliBar::from_goal(60);
    progress_bar.set_bar_len(20);
    println!("{}", progress_bar.set((42, 130)));
}
