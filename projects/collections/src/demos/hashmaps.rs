use std::collections::HashMap;

// Creating a HashMap: `new` + `insert` (insert takes OWNERSHIP of
// the key and value -- see the ownership demo later). Not in the
// prelude: the least used of the three collections, needs an
// explicit `use` (unlike Vec and String).
pub fn hashmaps_creation_demo() {
    println!("\n*** Hashmaps creation demo ***");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores: {scores:?}");
}

