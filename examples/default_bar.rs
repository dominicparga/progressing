//------------------------------------------------------------------------------------------------//
// other modules

use std::thread;
use std::time;

//------------------------------------------------------------------------------------------------//
// own modules

use progressing;
use progressing::Bar;

//------------------------------------------------------------------------------------------------//

fn main() -> Result<(), String> {
    //--------------------------------------------------------------------------------------------//

    let min_value = -20;
    let max_value = 120;

    println!();
    println!(
        "A bar running from {} % to {} % clipped to [0, 1]",
        min_value, max_value
    );

    // create bar
    let mut progressbar = progressing::ClippingBar::new();

    // do the job and show progress
    for value in min_value..(max_value + 1) {
        progressbar.reprint_with(value as f32 / 100.0)?;

        // sleep for visual effects ;)
        thread::sleep(time::Duration::from_millis(50));
    }
    // add new line to finished progressbar
    progressbar.reprintln()?;

    Ok(())
}
