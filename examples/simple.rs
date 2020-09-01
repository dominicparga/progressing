use progressing::{
    bernoulli::Bar as BernoulliBar, clamping::Bar as ClampingBar, mapping::Bar as MappingBar,
    Baring,
};

fn main() {
    // different examples for different use-cases
    clamped();
    println!();
    mapped();
    println!();
    timed_mapped();
    println!();
    bernoulli();
    println!();
    styles();
    println!();
    remember_progress();
}

/// Printing value 0.3 clamped to [0, 1]
/// [=====>------------]
fn clamped() {
    println!("Printing value 0.3 clamped to [0, 1]");
    let mut progress_bar = ClampingBar::new();
    progress_bar.set_len(20);
    progress_bar.set(0.3);
    println!("{}", progress_bar);
}

/// Mapping from [-9, 5] to [0, 1]
/// [================>-] (4 / 5)
fn mapped() {
    println!("Mapping from [-9, 5] to [0, 1]");
    let mut progress_bar = MappingBar::with_range(-9, 5);
    progress_bar.set_len(20);
    progress_bar.set(4);
    println!("{}", progress_bar);
}

/// Mapping from [-9, 5] to [0, 1], but with time-approximation
/// [================>-] (4 / 5) ~ 2 min
fn timed_mapped() {
    println!("Mapping from [-9, 5] to [0, 1], but with time-approximation");
    let mut progress_bar = MappingBar::with_range(-9, 5).timed();
    progress_bar.set_len(20);
    progress_bar.set(4);
    println!("{}", progress_bar);
}

/// Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)
/// [============>-----] (42 / 60 # 130)
fn bernoulli() {
    println!("Bernoulli-Bar counting successes (42 / 60) and attempts (# 130)");
    let mut progress_bar = BernoulliBar::with_goal(60);
    progress_bar.set_len(20);
    progress_bar.set((42, 130));
    progress_bar.add(true);
    println!("{}", progress_bar);

    let is_successful = true;
    if is_successful {
        // Does increase both 42 and 130
        progress_bar.add(true);
    } else {
        // Does increase 130 only
        progress_bar.add(false);
    }
}

/// clamped-example, but with other styles
fn styles() {
    println!("Custom styles");
    let mut progress_bar = ClampingBar::new();
    progress_bar.set_len(20);
    progress_bar.set(0.3);

    // different custom styles are possible

    // prints (----->............)
    progress_bar.set_style("(->.)");
    println!("{}", progress_bar);

    // prints [#####             ]
    progress_bar.set_style("[#  ]");
    println!("{}", progress_bar);

    // prints [#####-------------]
    progress_bar.set_style("(#--)");
    println!("{}", progress_bar);
}

fn remember_progress() {
    println!("Looped progress");

    // create bar
    let mut progress_bar = BernoulliBar::with_goal(100).timed();
    progress_bar.set_len(20);
    progress_bar.set(13);

    // do the job and show progress
    for _ in 0..100 {
        progress_bar.add(true);
        if progress_bar.has_progressed_significantly() {
            progress_bar.remember_significant_progress();
            println!("{}", progress_bar);
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    println!("{}", progress_bar);
}
