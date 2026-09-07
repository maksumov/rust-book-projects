// The cargo-generated test template (listing 11-1) -- the chapter's
// playground. #[cfg(test)] compiles the module only under
// `cargo test`; #[test] marks functions for the test runner.

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// Listing 11-7: assert_eq! compares with == and, on failure,
// prints BOTH values -- unlike assert!, which only says the
// condition was false. Rust names the arguments left/right (order
// doesn't matter), not expected/actual. Requires PartialEq + Debug
// on the compared type (both derivable; u64 has them). The mirror
// macro assert_ne! asserts "definitely not this value".
pub fn add_two(a: u64) -> u64 {
    a + 2
}

// The BUG variant from the book (+ 3 instead of + 2): it_adds_two
// fails, and assert_eq! shows exactly WHY -- both values in the
// panel. Uncomment and `cargo test` -- the panel below is an
// example of the failure output (counts, line numbers and the
// thread id reflect one particular run):
//
//     ---- adder::tests::it_adds_two stdout ----
//     thread 'adder::tests::it_adds_two' (163435) panicked at src/adder.rs:76:9:
//     assertion `left == right` failed
//       left: 5
//      right: 4
//     note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//
// pub fn add_two(a: u64) -> u64 {
//     a + 3
// }

#[cfg(test)]
mod tests {
    // use super::* brings the outer module's items into the tests
    // module -- regular visibility rules apply (ch 7).
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // Listing 11-3: a DELIBERATELY failing test. Each test runs in
    // its own thread; a panicking thread marks the test FAILED.
    // Note the module-qualified name: after the refactoring into
    // modules the runner reports adder::tests::another, not
    // tests::another as in the flat lib.rs (listing 11-4).
    // Uncomment and `cargo test` -- the panel below is an example
    // of the failure output (counts, line numbers and the thread
    // id reflect one particular run):
    //
    //     test adder::tests::another ... FAILED
    //
    //     failures:
    //
    //     ---- adder::tests::another stdout ----
    //     thread 'adder::tests::another' (139149) panicked at src/adder.rs:44:9:
    //     Make this test fail
    //     note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    //
    //     failures:
    //         adder::tests::another
    //
    //     test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    //
    //     error: test failed, to rerun pass `--lib`
    //
    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    // The Result form: Ok(()) passes, Err fails -- and the ?
    // operator becomes usable in the body (any failing operation
    // aborts the test). The harness accepts it via the same
    // Termination trait as main() -> Result (error-handling
    // project, question_in_main). #[should_panic] does NOT combine
    // with Result; assert an Err via assert!(value.is_err()).
    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
