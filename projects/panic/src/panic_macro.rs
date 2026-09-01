// `panic!` aborts immediately: the message is printed to stderr,
// the stack unwinds (or aborts, see profile settings), and the
// process exits with a failure code. Run with RUST_BACKTRACE=1
// to see the backtrace (listing 9-1's follow-up).

pub fn demo() {
    panic!("crash and burn");
}
