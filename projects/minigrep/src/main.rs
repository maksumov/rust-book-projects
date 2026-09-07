// The CLI contract: two arguments expected --
//     cargo run -- <search-string> <file-path>
// e.g. cargo run -- thebody poem.txt
// The argument parsing, validation and error messages arrive
// with the next listings (Config::build in ch 12.2).

// Listing 12-1: std::env::args -- an iterator over the CLI
// arguments; collect() materializes it. Note: args[0] is the
// binary path itself, the user's arguments start at index 1.
// dbg! prints to stderr with file:line -- a placeholder until
// the argument parsing arrives (next listings).

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
