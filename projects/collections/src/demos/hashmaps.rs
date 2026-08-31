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

// NOT in the current book edition (older editions had it): a
// HashMap can also be built with `collect` from an iterator of
// (key, value) tuples. The `HashMap<_, _>` annotation is needed:
// `collect` is generic over the target collection.
pub fn hashmaps_creation_via_collect_demo() {
    println!("\n*** Hashmaps creation via collect demo ***");

    // collect from tuples: zip pairs the teams with initial scores
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("scores via collect: {scores:?}");
}
