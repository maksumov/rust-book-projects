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

// `use` brings a path into scope once; after that, short paths suffice.
// Idiom: import the MODULE and call through it (hosting::add_to_waitlist),
// not the function itself -- the call site stays clear about where
// the item comes from. (For structs and enums, import the item directly.)

// `pub` turns this import into a RE-EXPORT: the shortcut itself
// becomes part of the public API (external path:
// restaurant::hosting), while the private internal structure
// stays hidden -- refactoring `front_of_house` won't break
// consumers of the public path.
pub use crate::front_of_house::hosting;

// Importing the function itself (listing 7-13) -- legal, but less
// idiomatic: a bare `add_to_waitlist()` call doesn't show where the
// item comes from (locally defined? imported?). Importing the module
// (the `hosting` use above) keeps the call site self-explanatory.
use crate::front_of_house::hosting::add_to_waitlist;

// Listing 7-14 idiom: for structs and enums, specify the FULL path to
// the type itself (`std::collections::HashMap`), unlike functions where
// importing the module and calling through it is preferred. The book
// demonstrates this in a binary crate's main.rs; the idiom is identical
// in the library root.
use std::collections::HashMap;

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

    // Path via `use` shortcut
    hosting::add_to_waitlist();

    // Via directly imported function -- works, but the origin is
    // unclear at a glance (non-idiomatic, see the comment above)
    add_to_waitlist();

    // HashMap via full-path import (listing 7-14 idiom demo)
    let mut table_map: HashMap<i32, i32> = HashMap::new();
    table_map.insert(1, 2);
    println!("{table_map:?}");

    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!("Order1 is {order1:?} and order2 is {order2:?}");
}

mod customer {
    pub fn eat_at_restaurant() {
        // The next line won't compile if we uncomment it: `use` binds
        // a name only within the module where it appears -- the
        // crate-root `use ... hosting;` is not visible inside `customer`.

        // hosting::add_to_waitlist();
    }
}

fn deliver_order() {}

mod back_of_house {
    // `pub` on a struct makes the TYPE public, not its fields:
    // fields stay private unless each is marked `pub` individually
    // (`toast` public, `seasonal_fruit` private -- the chef decides).
    // A private field means no struct literal is possible outside
    // this module, so a public associated constructor (`summer`)
    // is the only way to build an instance.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // `pub` on an enum makes ALL its variants public automatically --
    // a deliberate contrast with structs (listing 7-9): an enum whose
    // variants were private by default would be nearly useless.
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // `super` starts a relative path from the parent module -- like `..`
    // in a filesystem path. Useful when the item is closely related to
    // its parent: if the tree is reorganized and both move together,
    // the path stays valid. Note the privacy contrast: children CAN see
    // their ancestors' private items (`deliver_order` needs no `pub`).
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

// fmt and io are imported as MODULES for the listing 7-15 workaround:
// the name-clashing Result types are qualified at the call sites
// (fmt::Result, io::Result).
// Nested paths merge imports sharing a prefix: the two separate
// `use` lines become one; same names in scope.
use std::{fmt, io};

// The formatting Result, via module qualification
fn function1() -> fmt::Result {
    Ok(())
}

// The I/O Result with () success type, likewise qualified
fn function2() -> io::Result<()> {
    Ok(())
}

// `as` renames an import (listing 7-16): both Result types are now
// imported directly under distinct local names -- no qualification
// needed at the call sites.
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

// The formatting Result, via alias
fn function3() -> FmtResult {
    Ok(())
}

// The I/O Result, via alias
fn function4() -> IoResult<()> {
    Ok(())
}

// External packages: paths start with the crate NAME, not `crate::`
// (that prefix is only for the current crate). The version in
// Cargo.toml ("0.9.5") is shorthand for ^0.9.5: any compatible
// 0.9.x. `std` is the implicit dependency -- no Cargo.toml entry.
use rand::Rng;

pub fn lucky_table() {
    let table: u32 = rand::rng().random_range(1..=10);
    println!("You're seated at table {table}");
}

// `self` inside a nested path imports the prefix module itself:
// `use std::io::{self, Write};` brings BOTH `io` and the `Write`
// trait into scope in one line. Scoped inside a module on purpose:
// the crate root already imports `io` via the group above, and two
// imports of the same name in one scope would clash (E0252).
mod seat_writer {
    use std::io::{self, Write};

    // Uses both: `io` as a module qualifier, and `write_all` --
    // a Write-trait method on Stdout
    pub fn function5() -> io::Result<()> {
        io::stdout().write_all(b"table ready\n")
    }
}
