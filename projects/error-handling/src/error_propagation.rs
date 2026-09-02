// Note the `Read` import: read_to_string is a TRAIT method -- it
// only exists on File with the trait in scope (unlike inherent
// methods like File::open). Traits are chapter 10.
use std::fs::{self, File};
use std::io::{self, Read};

// Propagating errors: instead of handling (or panicking), the
// function RETURNS the error to the caller, who has more context.
// The same read_username_from_file evolves through four versions
// (listings 9-6 .. 9-9) -- read top to bottom to see the approach
// shrink from ~15 lines to one.

const FILENAME: &str = "hello.txt";

// Listing 9-6: manual match -- Err(e) => return Err(e) is the
// early return `?` will replace.
fn read_username_match() -> Result<String, io::Error> {
    let username_file_result = File::open(FILENAME);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Listing 9-7: the `?` operator -- sugar for exactly that match.
fn read_username_question() -> Result<String, io::Error> {
    let mut username_file = File::open(FILENAME)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Listing 9-8: `?` yields the value, so calls can be chained.
fn read_username_chained() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open(FILENAME)?.read_to_string(&mut username)?;

    Ok(username)
}

// Listing 9-9: the whole pattern is common enough for a std API.
fn read_username_fs() -> Result<String, io::Error> {
    fs::read_to_string(FILENAME)
}

pub fn demo() {
    println!("\n*** error_propagation demo: one function, four versions ***");

    println!("match (9-6):       {:?}", read_username_match());
    println!("? (9-7):           {:?}", read_username_question());
    println!("chained ? (9-8):   {:?}", read_username_chained());
    println!("fs shortcut (9-9): {:?}", read_username_fs());
}
