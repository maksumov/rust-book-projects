#![allow(dead_code)] // skeleton crate: functions get used from chapter 7.3 on

// A library crate: the root is src/lib.rs (no main); the artifact
// is a library for other crates to consume.
// Unlike backyard's `mod x;` files, these modules are inline --
// the whole tree lives in one file (listing 7-2):
//
// crate
//  └── front_of_house
//      ├── hosting (add_to_waitlist, seat_at_table)
//      └── serving (take_order, serve_order, take_payment)
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// Still non-compiling (listing 7-5): only `hosting` is `pub`, and that's
// enough for the path -- `front_of_house` stays private but is accessed
// from its own module (the crate root, where this function lives;
// items in the same module see each other). The error is now
// "function `add_to_waitlist` is private" (listing 7-6); the final
// step is `pub fn` (listing 7-7).
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
