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

// `get` looks up a key and returns Option<&V> -- the safe form:
// hash maps have no indexing syntax, and a missing key is None,
// not a panic. The copied/unwrap_or chain below handles absence.
pub fn hashmaps_accessing_values_demo() {
    println!("\n*** Hashmaps accessing values demo ***");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // `get` returns Option<&V>: copied() -> Option<V>, then
    // unwrap_or(default) -> V. "Green" is deliberately absent from
    // the map, so the loop demonstrates BOTH paths: Some -> the
    // stored value, None -> the default.
    for team_name in ["Blue", "Green"] {
        // Lookup by &str directly: `get` is generic over Q where
        // K: Borrow<Q>, and String: Borrow<str> -- no allocation,
        // unlike building a String key per lookup.
        let score = scores.get(team_name).copied().unwrap_or(0);
        println!("The {team_name:?} have {score} points");
    }
}

// Iterating yields (&K, &V) pairs in ARBITRARY order -- the hash
// function decides the layout, not insertion order.
pub fn hashmaps_iteration_demo() {
    println!("\n*** Hashmaps iteration demo ***");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
