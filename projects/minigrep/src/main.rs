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
    // User arguments start at index 1 (args[0] is the binary path).
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {:?}", config.query);
    println!("In file {:?}", config.file_path);

    // The std one-shot file read: io::Result<String>;
    // expect panics on Err (replaced later in this chapter).
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("\nWith text:\n{contents}");
}

// Program configuration: the parsed CLI arguments as owned data.
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // Maps CLI arguments to a Config. An associated fn -- the
    // String::new idiom for constructors. Cloning is deliberate:
    // simplicity over performance (iterator-based parsing arrives
    // in chapter 13). Panics on missing arguments (fixed later
    // in this chapter).
    fn new(args: &[String]) -> Config {
        // 3 = binary path + query + file_path; fewer means usage error
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
