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

    println!();
    println!("A bar running from -1 % to 120 % clipped to [0, 1]");

    // create bar
    let mut progressbar = progressing::ClippingBar::new();
    progressbar.set_bar_len(30);
    progressbar.set_prefix("  Progress: ");

    // do the job and show progress
    for value in -20..121 {
        // show clipping explicitely
        if value < 0 || 100 < value {
            progressbar.set_suffix(&format!(" ({} %)", value));
        } else {
            progressbar.set_suffix("");
        }
        progressbar.reprint_with(value as f32 / 100.0)?;

        // sleep for visual effects ;)
        thread::sleep(time::Duration::from_millis(50));
    }
    // add new line to finished progressbar
    progressbar.reprintln()?;

    Ok(())
}
