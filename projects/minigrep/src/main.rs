// The CLI contract: two arguments expected --
//     cargo run -- <search-string> <file-path>
// e.g. cargo run -- thebody poem.txt

use minigrep::{search, search_case_insensitive};
use std::env;
use std::error::Error;
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

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

// Program configuration: the parsed CLI arguments as owned data,
// plus process-environment state (ignore_case).
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
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

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// The program's logic: everything except config parsing and
// error handling (main stays a thin orchestrator).
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // The std one-shot file read: io::Result<String>;
    // the ? operator propagates any error up to the caller.
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
