// The cargo-generated test template (listing 11-1) -- the chapter's
// playground. #[cfg(test)] compiles the module only under
// `cargo test`; #[test] marks functions for the test runner.

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
