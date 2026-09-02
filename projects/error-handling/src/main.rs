// `#![...]` is an INNER attribute: it applies to the enclosing
// item -- here, the whole crate (a plain `#[...]` is OUTER and
// attaches to the next item only, e.g. a single `mod` line).
//
// Demos are orchestrated manually: commenting out a call leaves
// its module unused -- expected here, not real dead code.
#![allow(dead_code)]

// The demos open "hello.txt" in the CURRENT directory and PANIC by
// design on the Err paths, so only the FIRST call ever runs --
// comment out the calls above to observe the later demos. hello.txt
// is not committed; note that error_kind_match CREATES it as a side
// effect of the NotFound branch.

mod error_kind_match;
mod open_match;

fn main() {
    // panics first when hello.txt is missing -- blocks the demos below:
    open_match::demo();

    error_kind_match::demo();
}
