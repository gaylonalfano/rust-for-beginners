// NOTE: REF: https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html
// NOTE: !! Separation of concerns (main.rs vs. lib.rs) for binary projects.
// --- The responsibilities of main.rs:
// - Calling the command line parsing logic with the argument values
// - Setting up any other configuration
// - Calling a run function in lib.rs
// - Handling the error if run returns an error
// --- The responsibilities of lib.rs:
// - Handle program logic of the task

use minigrep::Config;

use std::{env, process};

fn main() {
    // -- With iterators
    let config = Config::build_with_iter(env::args()).unwrap_or_else(|err| {
        // NOTE: The inner 'err' value is "not enough arguments provided"
        // println!("Problem parsing arguments: {err}");
        // NOTE: A nonzero exit status is a convention to signal
        // to the process that called our program that
        // the program exited with an error state.
        // NOTE: !! println!() is stdout & eprintln!() is stderr
        // $ cargo run > output.txt
        // $ cargo run -- to poem.txt > output.txt
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    // -- Without using iterators
    // let args: Vec<String> = env::args().collect();
    //
    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     // NOTE: The inner 'err' value is "not enough arguments provided"
    //     // println!("Problem parsing arguments: {err}");
    //     // NOTE: A nonzero exit status is a convention to signal
    //     // to the process that called our program that
    //     // the program exited with an error state.
    //     // NOTE: !! println!() is stdout & eprintln!() is stderr
    //     // $ cargo run > output.txt
    //     // $ cargo run -- to poem.txt > output.txt
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1)
    // });
    //
    println!("Search term: {}", config.query);
    println!("File path: {}", config.file_path);

    // NOTE: if let Err(e) vs. unwrap_or_else(|e|..)
    // The run() doesn't return a value that we want to unwrap,
    // so we just want to detect an error.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
