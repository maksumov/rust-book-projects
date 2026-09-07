// #[should_panic] INVERTS the rule "a panicking test fails":
// the test passes only if the code panics. Imprecise by default
// (ANY panic counts), so `expected = "..."` narrows it: the panic
// message must CONTAIN the given substring -- the book refines a
// single-message Guess (listing 9-13) into this two-message
// version to make the substring unique.
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }

        Guess { value }
    }

    // The getter from listing 9-13: keeps the private field readable
    // (and silences the field-never-read warning).
    pub fn value(&self) -> i32 {
        self.value
    }
}

// BUG variant A (the > 100 branch removed): Guess::new(200) no
// longer panics, and the test fails with the least informative
// message possible. Uncomment (replacing the new above) and
// `cargo test` -- the panel below is an example of the failure
// output (counts, line numbers and the thread id reflect one
// particular run):
//
//     ---- guess::tests::greater_than_100 stdout ----
//     note: test did not panic as expected at src/guess.rs:69:8
//
// pub fn new(value: i32) -> Guess {
//     if value < 1 {
//         panic!("Guess value must be greater than or equal to 1, got {value}.");
//     }
//
//     Guess { value }
// }

// BUG variant B (panic bodies swapped): the code DOES panic, but
// with the other message -- `expected` catches the mismatch.
// Uncomment (replacing the new above) and `cargo test` -- the
// panel below is an example of the failure output:
//
//     ---- guess::tests::greater_than_100 stdout ----
//     thread 'guess::tests::greater_than_100' (185328) panicked at src/guess.rs:16:13:
//     Guess value must be greater than or equal to 1, got 200.
//     note: panic did not contain expected string
//       panic message: "Guess value must be greater than or equal to 1, got 200."
//      expected substring: "less than or equal to 100"
//
// pub fn new(value: i32) -> Guess {
//     if value < 1 {
//         panic!("Guess value must be less than or equal to 100, got {value}.");
//     } else if value > 100 {
//         panic!("Guess value must be greater than or equal to 1, got {value}.");
//     }
//
//     Guess { value }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // The panic message must CONTAIN this substring (not match
    // exactly) -- enough to pin the > 100 branch specifically:
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
