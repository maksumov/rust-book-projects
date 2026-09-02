// Listings 9-10 + 9-12: where `?` can live. The broken variant:
//
//     fn main() {
//         let greeting_file = File::open("hello.txt")?;
//     }
//
// fails with E0277: "the `?` operator can only be used in a
// function that returns `Result` or `Option`" -- `?` performs an
// early return Err(..), and () cannot carry it. Listing 9-12's
// fix: main CAN return Result<(), Box<dyn Error>> -- the Termination
// trait maps Ok to exit code 0, Err to nonzero. `Box<dyn Error>`
// is a trait object: "any kind of error" (chapter 18 preview).
//
// This project's main stays an orchestrator, so the same pattern
// lives on an inner function: any Result-returning fn can host `?`.

fn open_greeting_file() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;

    let _greeting_file = File::open("hello.txt")?;
    Ok(())
}

pub fn demo() {
    println!("\n*** question_in_main demo: where `?` can live ***");

    println!("inner fn result: {:?}", open_greeting_file());
}
