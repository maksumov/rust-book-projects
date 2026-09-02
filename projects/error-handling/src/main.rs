// `#![...]` is an INNER attribute: it applies to the enclosing
// item -- here, the whole crate (a plain `#[...]` is OUTER and
// attaches to the next item only, e.g. a single `mod` line).
//
// Disabled on purpose: an active `#![allow(dead_code)]` silences
// the warnings caused by orchestrating demos via commented-out
// calls, but it also hides the IDE's "unused function" hint --
// which is what catches forgotten demo() calls. Uncomment for a
// warning-free experimentation session if ever needed.
// #![allow(dead_code)]

// The demos open "hello.txt" in the CURRENT directory and PANIC by
// design on the Err paths, so only the FIRST call ever runs --
// comment out the calls above to observe the later demos. hello.txt
// is not committed; note that error_kind_match CREATES it as a side
// effect of the NotFound branch.

mod error_kind_match;
mod open_match;
mod unwrap_expect;
mod unwrap_or_else;

fn main() {
    // panics first when hello.txt is missing -- blocks the demos below:
    open_match::demo();

    // The next two demos implement the SAME open-or-create logic:
    // a nested-match primitive vs a combinator chain. Idiomatic
    // Rust prefers combinators -- small closure-taking methods
    // (unwrap_or_else & friends) that compose flat pipelines
    // instead of match pyramids; match stays for genuinely
    // multi-way logic. Closures themselves arrive in chapter 13.
    error_kind_match::demo();
    unwrap_or_else::demo();

    // unwrap/expect: shorthands for the open_match pattern -- the
    // Ok value comes out, Err panics; expect lets the panic carry
    // your own message.
    unwrap_expect::demo();
}
