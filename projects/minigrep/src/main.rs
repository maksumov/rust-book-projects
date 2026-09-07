// The CLI contract: two arguments expected --
//     cargo run -- <search-string> <file-path>
// e.g. cargo run -- thebody poem.txt

use std::env;
use std::fs;

fn main() {
    // Listing 12-1: std::env::args -- an iterator over the CLI
    // arguments; collect() materializes it. Note: args[0] is the
    // binary path itself, the user's arguments start at index 1.
    let args: Vec<String> = env::args().collect();

    // Listing 12-2: arguments saved into variables (borrowed by
    // index -- book listing). My adaptation: {:?} instead of the
    // book's {} -- the Debug quotes make stray whitespace visible.
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query:?}");
    println!("In file {file_path:?}");

    // Listing 12-4: fs::read_to_string -- the std one-shot shortcut
    // (opens, reads to String, returns io::Result<String>; expect
    // panics on Err).
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("\nWith text:\n{contents}");
}
