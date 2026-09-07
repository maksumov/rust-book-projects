// Custom failure messages: any extra arguments after the required
// ones go to format! -- works with assert!, assert_eq! and
// assert_ne!. Useful to document what an assertion MEANS: when it
// fails, the message carries the actual value (here: the greeting).

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

// The BUG variant from the book (name ignored): the test fails,
// and the custom message pays off -- it prints the actual value.
// Uncomment and `cargo test` -- the panel below is an example of
// the failure output (counts, line numbers and the thread id
// reflect one particular run):
//
//     ---- greeter::tests::greeting_contains_name stdout ----
//     thread 'greeter::tests::greeting_contains_name' (171509) panicked at src/greeter.rs:34:9:
//     Greeting did not contain name, value was `Hello!`
//     note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//
// Without the custom message the same failure would print only
// the default line: assertion failed: result.contains("Carol")
// -- no actual value, harder to debug.
//
// pub fn greeting(name: &str) -> String {
//     String::from("Hello!")
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // The second argument is a custom failure message (format!):
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }
}
