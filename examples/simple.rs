use progressing::{self, Bar};

fn main() {
    // different examples for different use-cases
    clamped();
    println!();
    mapped();
    println!();
    bernoulli();
    println!();
    styles();
}

/// Printing value 0.3 clamped to [0, 1]
/// [=====>------------]
fn clamped() {
    println!("Printing value 0.3 clamped to [0, 1]");
    let mut progress_bar = progressing::ClampingBar::new();
    progress_bar.set_len(20);
    progress_bar.set(0.3);
    println!("{}", progress_bar);
}

/// Mapping from [-9, 5] to [0, 1]
/// [================>-] (4 / 5)
fn mapped() {
    println!("Mapping from [-9, 5] to [0, 1]");
    let mut progress_bar = progressing::MappingBar::new(-9, 5);
    progress_bar.set_len(20);
    progress_bar.set(4);
    println!("{}", progress_bar);
}

/// Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)
/// [============>-----] (42 / 60 # 130)
fn bernoulli() {
    println!("Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)");
    let mut progress_bar = progressing::BernoulliBar::from_goal(60);
    progress_bar.set_len(20);
    progress_bar.set((42, 130));
    println!("{}", progress_bar);
}

// clamped-example, but with other styles
fn styles() {
    println!("Custom styles");
    let mut progress_bar = progressing::ClampingBar::new();
    progress_bar.set_len(20);
    progress_bar.set(0.3);

    progress_bar.set_style("(->.)");
    println!("{}", progress_bar);

    progress_bar.set_style("[#  ]");
    println!("{}", progress_bar);

    progress_bar.set_style("(#--)");
    println!("{}", progress_bar);
}
