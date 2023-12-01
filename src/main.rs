pub mod one;


use core::panic;

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    exercise: String,
}


fn main() {
    let args = Cli::parse();
    match args.exercise.as_str() {
        "one" => one::run(),
        _ => {
            panic!("Unknown exercise {}", args.exercise);
        }
    }
}
