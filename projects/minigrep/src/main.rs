// The CLI contract: two arguments expected --
//     cargo run -- <search-string> <file-path>
// e.g. cargo run -- thebody poem.txt

use std::env;
use std::fs;

// main's target responsibilities (the book's checklist, by the
// end of the chapter 12.3 refactor):
//   1. call the parsing logic;
//   2. set up any remaining config;
//   3. call run() from lib;
//   4. handle run's errors.
// Everything else moves to lib.rs.

fn main() {
    // Listing 12-1: std::env::args -- an iterator over the CLI
    // arguments; collect() materializes it. Note: args[0] is the
    // binary path itself, the user's arguments start at index 1.
    let args: Vec<String> = env::args().collect();

    // Listing 12-5: parsing extracted from main -- it no longer
    // decides which argument goes where. The {:?} adaptation stays
    // (Debug quotes make stray whitespace visible).
    let (query, file_path) = parse_config(&args);

    println!("Searching for {query:?}");
    println!("In file {file_path:?}");

    // Listing 12-4: fs::read_to_string -- the std one-shot shortcut
    // (opens, reads to String, returns io::Result<String>; expect
    // panics on Err).
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("\nWith text:\n{contents}");
}

// Listing 12-5: the args -> (query, file_path) mapping lives here
// now. Still borrowing by index; ownership questions surface with
// the Config struct (next listings). Panics on missing arguments
// -- fixed later in this section.
fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
