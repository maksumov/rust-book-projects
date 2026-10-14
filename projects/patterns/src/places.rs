// ch 19.1: all the places patterns can be used (listings 19-1..19-7).
// The revelation: `let x = 5;` was ALWAYS a pattern - the variable
// name is just the simplest one. Patterns also live in match arms,
// if let / else if let chains, while let, for loops, and function
// parameters.

use std::sync::mpsc;

// Listing 19-6/19-7: function parameters are patterns too - here a
// reference-to-tuple is destructured right in the signature.
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}

pub fn demo_background() {
    println!("\n*** demo of patterns: the places they live ***");

    // Listing 19-1: let PATTERN = EXPRESSION - destructuring a tuple.
    // The variable name in `let x = 5` is the simplest pattern; this
    // one nests three of them:
    let (x, y, z) = (1, 2, 3);
    println!("destructured: {x}, {y}, {z}");

    // Listing 19-2: the pattern must match the shape - two names for
    // three elements does not compile:
    //
    // // let (x, y) = (1, 2, 3);
    //
    // error[E0308]: mismatched types
    //   = note: expected tuple `({integer}, {integer}, {integer})`
    //              found tuple `(_, _)`

    // Listing 19-3: if let / else if / else if let / else mixed - the
    // arms need not relate to each other (unlike a match on one value).
    // PREDICT before reading on: what color wins here?
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        // note the shadowing: a NEW age, the u8 inside Ok - which is
        // why the > 30 check must live INSIDE this block (the book's
        // point: `if let Ok(age) = age && age > 30` cannot work).
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // Listing 19-4: while let - runs as long as the pattern keeps
    // matching. A ch 16 channel revisited: recv gives Ok while the
    // sender lives, Err when it drops - the loop ends BY PATTERN.
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });
    while let Ok(value) = rx.recv() {
        println!("received {value}");
    }

    // Listing 19-5: the `for x in y` - x is a pattern (enumerate
    // tuples, as in minigrep). Listing 19-6/19-7 below: parameters.
    print_coordinates(&(3, 5));
}
