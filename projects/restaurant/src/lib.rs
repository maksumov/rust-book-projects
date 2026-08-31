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
use crate::front_of_house::hosting;

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
