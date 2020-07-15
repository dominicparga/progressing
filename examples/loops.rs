use progressing::{self, Bar};
use std::{thread, time};

const SLEEP_MS: u64 = 20;

fn main() {
    // different examples for different use-cases
    clamped();
    println!();
    mapped();
    println!();
    bernoulli();
}

fn clamped() {
    let min_value = -80;
    let max_value = 180;

    println!(
        "The bar is running from {} % to {} % clamping to [0, 1].",
        min_value, max_value
    );
    println!(
        "{}{}",
        "Note the respective pause at the beginning and the end, ",
        "which causes the approximated time to be too high."
    );

    // create bar
    let progress_bar = progressing::ClampingBar::new();
    let mut progress_bar = progressing::TimedBar::new(progress_bar);

    // do the job and show progress
    for value in min_value..(max_value + 1) {
        progress_bar.set(value as f32 / 100.0);
        if progress_bar.has_progressed_much() {
            progress_bar.remember_progress();
            println!("{}", progress_bar);
        }

        // sleep for visual effects ;)
        thread::sleep(time::Duration::from_millis(SLEEP_MS));
    }
    // add new line to finished progress-bar
    println!("{}", progress_bar);
}

fn mapped() {
    let min_value = -10;
    let max_value = 100;
    let min_bar_border = -40;
    let max_bar_border = 140;

    println!(
        "The bar is running from {} to {}, but maps [{}, {}] to [0, 1].",
        min_value, max_value, min_bar_border, max_bar_border
    );
    println!("Note that the bar neither starts nor ends at the bar-borders.");

    // create bar
    let mut progress_bar = progressing::MappingBar::new(min_bar_border, max_bar_border);

    // do the job and show progress
    for value in min_value..(max_value + 1) {
        progress_bar.set(value);
        if progress_bar.has_progressed_much() {
            progress_bar.remember_progress();
            println!("{}", progress_bar);
        }

        // sleep for visual effects ;)
        thread::sleep(time::Duration::from_millis(SLEEP_MS));
    }
    // add new line to finished progress-bar
    println!("{}", progress_bar);
}

fn bernoulli() {
    let min_value = -50;
    let max_value = 120;

    println!(
        "The bar is running from {} to {} counting successes (if value is even) and attempts ({}).",
        min_value,
        max_value,
        (max_value - min_value) + 1
    );
    println!("Note that the bar expects less successes than provided .");

    // create bar
    let mut progress_bar = progressing::BernoulliBar::from_goal(60);
    // you can reset the length of it
    progress_bar.set_len(60);

    // do the job and show progress
    for value in min_value..(max_value + 1) {
        // job is successful if value is even
        let is_successful = value % 2 == 0;
        progress_bar.add(is_successful);
        if progress_bar.has_progressed_much() {
            progress_bar.remember_progress();
            println!("{}", progress_bar);
        }

        // sleep for visual effects ;)
        thread::sleep(time::Duration::from_millis(SLEEP_MS));
    }
    // add new line to finished progress-bar
    println!("{}", progress_bar);
}
