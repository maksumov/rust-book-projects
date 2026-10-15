// ch 19.2: refutability - whether a pattern might fail to match
// (listings 19-8..19-10). Irrefutable: matches ANY value (x, a
// struct with all-named fields). Refutable: can fail (Some(x),
// literal 5). let/for/fn-params demand irrefutable; if let / while
// let / let...else accept refutable - that is their POINT.

pub fn demo_let_else() {
    println!("\n*** demo of refutability: let...else gives the out ***");

    // Listing 19-8: a refutable pattern in plain let - rejected:
    //
    // // let some_option_value: Option<i32> = None;
    // // let Some(x) = some_option_value;
    //
    // error[E0005]: refutable pattern in local binding
    // 3 |     let Some(x) = some_option_value;
    //   |         ^^^^^^^ pattern `None` not covered
    //   = note: `let` bindings require an "irrefutable pattern"
    // help: you might want to use `let...else`

    // Listing 19-9: let...else - the refutable pattern gets an exit.
    // The else block must diverge (return, break, panic...). Two
    // sequential cases: the first takes the happy path, the second
    // takes the ELSE - observed, and it ends the demo right there.
    let some_option_value: Option<i32> = Some(5);
    let Some(x) = some_option_value else {
        return;
    };
    println!("bound {x} - straight-line, no unwrap");

    let none_option_value: Option<i32> = None;
    let Some(_y) = none_option_value else {
        println!("else taken - the demo ends here");
        return;
    };
    println!("never printed: {_y}");
}

// Listing 19-10: the reverse mismatch - an IRREFUTABLE pattern in
// let...else - compiles with a warning, the else is unreachable:
//
// // let x = 5 else {
// //     return;
// // };
//
// warning: unreachable `else` clause
//   = note: this pattern always matches, so the else clause is
//   unreachable
