// The same logic as error_kind_match, without any `match`:
// `unwrap_or_else` takes a CLOSURE that runs only on the Err
// branch. Closures are chapter 13 -- bookmark this and revisit
// (the book's own advice); for now, read `|error| { ... }` as
// an inline function receiving the error.

pub fn demo() {
    println!("\n*** unwrap_or_else demo: the 9-5 logic without any match ***");

    use std::fs::File;
    use std::io::ErrorKind;

    let filename = "hello.txt";

    let greeting_file = File::open(filename).unwrap_or_else(|error| {
        println!("Err branch: {error:?}");
        if error.kind() == ErrorKind::NotFound {
            println!("NotFound -- creating the file");
            File::create(filename).unwrap_or_else(|error| {
                panic!("Problem creating the file {filename:?}: {error:?}");
            })
        } else {
            panic!("Problem opening the file {filename:?}: {error:?}");
        }
    });

    println!("opened: {greeting_file:?}");
}
