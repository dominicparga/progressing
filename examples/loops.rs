//------------------------------------------------------------------------------------------------//
// other modules

use std::thread;
use std::time;

//------------------------------------------------------------------------------------------------//
// own modules

use progressing;
use progressing::Bar;

//------------------------------------------------------------------------------------------------//
// different examples for different use-cases

fn clamped() -> Result<(), String> {
    let min_value = -40;
    let max_value = 140;

    println!();
    println!(
        "The bar is running from {} % to {} % clamping to [0, 1].",
        min_value, max_value
    );
    println!("Note the respective pause at the beginning and the end.");

    // create bar
    let mut progressbar = progressing::ClampingBar::new();

    // do the job and show progress
    for value in min_value..(max_value + 1) {
        progressbar.set(value as f32 / 100.0).reprint()?;

        // sleep for visual effects ;)
        thread::sleep(time::Duration::from_millis(30));
    }
    // add new line to finished progressbar
    progressbar.reprintln()
}

//------------------------------------------------------------------------------------------------//

fn mapped() -> Result<(), String> {
    let min_value = -10;
    let max_value = 100;

    println!();
    println!(
        "The bar is running from {} to {} mapping [-40, 140] to [0, 1].",
        min_value, max_value
    );
    println!("Note that the bar doesn't start/end at the bar-borders.");

    // create bar
    let mut progressbar = progressing::MappingBar::new(-40..=140);

    // do the job and show progress
    for value in min_value..(max_value + 1) {
        progressbar.set(value).reprint()?;

        // sleep for visual effects ;)
        thread::sleep(time::Duration::from_millis(30));
    }
    // add new line to finished progressbar
    progressbar.reprintln()
}

//------------------------------------------------------------------------------------------------//

fn bernoulli() -> Result<(), String> {
    let min_value = -50;
    let max_value = 120;

    println!();
    println!(
        "The bar is running from {} to {} counting successes (if value is even) and attempts ({}).",
        min_value,
        max_value,
        (max_value - min_value) + 1
    );
    println!("Note that the bar expects less successes than provided.");

    // create bar
    let mut progressbar = progressing::BernoulliBar::from_goal(60);
    // you can reset the lenght of it
    progressbar.set_bar_len(60);

    // do the job and show progress
    for value in min_value..(max_value + 1) {
        // job is successful if value is even
        let is_successful = value % 2 == 0;
        progressbar.add(is_successful.into()).reprint()?;

        // sleep for visual effects ;)
        thread::sleep(time::Duration::from_millis(30));
    }
    // add new line to finished progressbar
    progressbar.reprintln()
}

//------------------------------------------------------------------------------------------------//

fn main() -> Result<(), String> {
    clamped()?;
    mapped()?;
    bernoulli()?;

    Ok(())
}
