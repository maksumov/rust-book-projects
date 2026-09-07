// Integration tests (listing 11-13): entirely external to the
// library -- this file is compiled as its OWN crate and uses the
// public API exactly like any outside consumer would. Hence the
// use statement (unit tests need none: they live inside) and no
// #[cfg(test)] (Cargo compiles tests/ only under cargo test).
// Only pub items are reachable from here -- the private
// internal_adder is invisible (contrast: adder::tests::internal).

use testing::adder::add_two;

#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}
