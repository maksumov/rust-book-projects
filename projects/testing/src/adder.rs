// The cargo-generated test template (listing 11-1) -- the chapter's
// playground. #[cfg(test)] compiles the module only under
// `cargo test`; #[test] marks functions for the test runner.

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

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
    // Uncomment and `cargo test` to see the panel below (the
    // thread id in parentheses varies per run):
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
    //     test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    //
    //     error: test failed, to rerun pass `--lib`
    //
    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }
}
