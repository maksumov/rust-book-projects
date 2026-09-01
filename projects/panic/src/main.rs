// Both demos panic by design, so only the FIRST call ever runs.
// Comment one out to see the other; backtrace also wants
// RUST_BACKTRACE=1 (see backtrace.rs).
mod backtrace;
mod panic_macro;

fn main() {
    panic_macro::demo();
    backtrace::demo();
}
