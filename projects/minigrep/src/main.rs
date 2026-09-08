// The CLI contract: two arguments expected --
//     cargo run -- <search-string> <file-path>
// e.g. cargo run -- thebody poem.txt

use std::env;
use std::fs;
use std::process;

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

    // Build errors are usage errors: a friendly message + a nonzero
    // exit code, not a panic (the chapter 9 guidelines).
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

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
    // build (not new) idiom: new is expected to never fail.
    // Cloning is deliberate: simplicity over performance
    // (iterator-based parsing arrives in chapter 13). Errors on
    // missing arguments; main converts Err into a friendly
    // message + a nonzero exit code.
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // 3 = binary path + query + file_path; fewer means usage error
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
