// The library half of minigrep: everything testable lives here
// (the lib+bin pattern; the binary in main.rs is a thin consumer).

// Returns every line of contents containing the query. The 'a
// lifetime ties the returned lines to CONTENTS (each returned
// &str is a slice of it) -- not to query: nothing borrowed from
// query escapes the function. Stub until chapter 12.4 fills it.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    unimplemented!();
}
