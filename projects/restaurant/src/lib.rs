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
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// Compiles now (listing 7-7): the chain `pub mod hosting` +
// `pub fn add_to_waitlist` opens the path. `front_of_house` stays
// private and that's fine: it's accessed from its own module (the
// crate root, where this function lives -- items in the same
// module see each other).
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
