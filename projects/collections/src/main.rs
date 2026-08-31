// Chapter 8 demos, one file per collection under demos/:
// `mod demos;` loads demos.rs, its children live in demos/.
mod demos;

use crate::demos::vectors;

fn main() {
    vectors::vectors_creation_demo();
    vectors::vectors_updating_demo();
}
