// The CLI contract: two arguments expected --
//     cargo run -- <search-string> <file-path>
// e.g. cargo run -- thebody poem.txt
// The argument parsing, validation and error messages arrive
// with the next listings (Config::build in ch 12.2).

// Listing 12-1: std::env::args -- an iterator over the CLI
// arguments; collect() materializes it. Note: args[0] is the
// binary path itself, the user's arguments start at index 1.

// Listing 12-2: arguments saved into variables (borrowed by
// index -- book listing). My adaptation: {:?} instead of the
// book's {} -- the Debug quotes make stray whitespace visible.

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query:?}");
    println!("In file {file_path:?}");
}
