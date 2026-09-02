// Shortcuts for panic on error: `unwrap` panics with a generic
// default message; `expect(msg)` panics with YOUR message. The
// book recommends expect in production code: a good message says
// why success was expected -- when the assumption breaks, it
// points straight at the bug.

pub fn demo() {
    use std::fs::File;

    println!("\n*** unwrap_expect demo: panic-on-error shortcuts ***");

    let filename = "hello.txt";

    // Uncomment (and delete hello.txt) to see the DEFAULT panic
    // message -- note it carries no context of WHY the file was
    // expected: "called `Result::unwrap()` on an `Err` value:
    // Os { code: 2, kind: NotFound, message: \"No such file or directory\" }"
    // let greeting_file = File::open(filename).unwrap();

    // `expect` panics with YOUR message instead -- the book's
    // recommendation for production code. When it fires, the message
    // reads: `"hello.txt" should be included in this project:
    // Os { code: 2, ... }` -- your text first, the error appended
    // after a colon.
    //
    // Note: expect takes a plain &str -- no format machinery, so
    // "{filename:?}" would print literally; interpolate via
    // &format!(...) if ever needed (the book hardcodes the name).
    let greeting_file =
        File::open(filename).expect(&format!("{filename:?} should be included in this project"));

    println!("opened: {greeting_file:?}");
}
