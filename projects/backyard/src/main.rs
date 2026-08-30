// The module tree maps to files: `pub mod garden;` -> src/garden.rs,
// and the nested `pub mod vegetables;` -> src/garden/vegetables.rs.
// `pub` is required at every level: an ancestor (the crate root here)
// cannot see private items of its descendants -- privacy is default.
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
