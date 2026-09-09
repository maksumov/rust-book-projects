// The essential difference between functions and closures:
// closures capture their defining environment, functions cannot.

pub fn demo() {
    println!("\n*** function vs closure: environment capture ***");

    let threshold = 10;

    // The closure uses `threshold` from the enclosing scope --
    // it is captured (here: by immutable reference).
    let is_above = |x: i32| x > threshold;
    println!("threshold is {threshold}");
    println!("13 above threshold: {}", is_above(13));

    // A function can SEE `threshold` but cannot capture it -- fn
    // items have no environment. Uncomment to see the error:
    //
    // fn is_above_fn(x: i32) -> bool {
    //     x > threshold
    // }
    //
    // error[E0434]: can't capture dynamic environment in a fn item
    //   --> src/function_vs_closure.rs:18:13
    //    |
    // 18 |         x > threshold
    //    |             ^^^^^^^^^
    //    |
    //    = help: use the `|| { ... }` closure form instead
    //
    // For more information about this error, try `rustc --explain E0434`.
    // error: could not compile `closures` (bin "closures") due to 1 previous error
}
