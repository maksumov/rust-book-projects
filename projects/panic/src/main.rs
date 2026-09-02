// Both demos panic by design, so only the FIRST call ever runs.
// Comment one out to see the other; backtrace also wants
// RUST_BACKTRACE=1 (see backtrace.rs).
//
// Chapter 9.3 pairing: panic! suits unrecoverable states (bugs,
// examples/prototypes/tests); expected failures -- files, input,
// network -- should return Result instead: see the error-handling
// project for that side of the split.
mod backtrace;
mod panic_macro;

fn main() {
    panic_macro::demo();
    backtrace::demo();
}
