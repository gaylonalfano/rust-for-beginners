// === DEMO External Crates
// NOTES
// - Crates are ways to include other peoples' code in your program
// - When creating a new project with 'cargo init .' you get a Cargo.toml file.
// In this file there is a [dependencies] section to add crates e.g.:
// [dependencies]
// humantime = "2.1.0"

// use humantime::format_duration;
// use std::time::Duration;

// fn main() {
//     let d = Duration::from_secs(9876);
//     println!("{}", format_duration(d));
// }



// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

use chrono::prelude::*;

fn main() {
    let utc: DateTime<Utc> = Utc::now();
    // Now let's create a custom format for readability
    let fmt: &str = "%Y-%m-%d %H:%M:%S";
    println!("{}", utc.format(fmt).to_string());
}
