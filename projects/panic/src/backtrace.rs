// Listing 9-2: the panic here comes not from our macro but from
// the standard library detecting a bug -- indexing out of bounds:
// "the len is 3 but the index is 99". Run as
//     RUST_BACKTRACE=1 cargo run
// (with the panic_macro call in main commented out) to see the
// full call-stack trace of the failure (debug builds have the
// symbols; release strips them by default).

pub fn demo() {
    let v = vec![1, 2, 3];

    v[99];
}
