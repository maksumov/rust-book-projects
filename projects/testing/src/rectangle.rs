// Listings 11-5/11-6: assert! takes a bool; on false it panics --
// which is exactly how a test fails. can_hold returns bool, making
// it a perfect assert! use case (the struct and method return from
// chapter 5's rectangles project).

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

// The BUG variant from the book (width compared with `<` instead
// of `>`): with it, larger_can_hold_smaller fails while
// smaller_cannot_hold_larger still passes -- the tests caught the
// bug. Uncomment and `cargo test` -- the panel below is an example
// of the failure output (counts, line numbers and the thread id
// reflect one particular run):
//
//     test rectangle::tests::larger_can_hold_smaller ... FAILED
//
//     failures:
//
//     ---- rectangle::tests::larger_can_hold_smaller stdout ----
//     thread 'rectangle::tests::larger_can_hold_smaller' (141878) panicked at src/rectangle.rs:49:9:
//     assertion failed: larger.can_hold(&smaller)
//     note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//
//     failures:
//         rectangle::tests::larger_can_hold_smaller
//
//     test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
//     error: test failed, to rerun pass `--lib`
//
// impl Rectangle {
//     pub fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width < other.width && self.height > other.height
//     }
// }
impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    // use super::* brings the outer module's items into the tests
    // module -- regular visibility rules apply (ch 7).
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
