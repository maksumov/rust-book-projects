use std::collections::HashMap;

// Creating a HashMap: `new` + `insert` (insert takes OWNERSHIP of
// the key and value -- see the ownership demo later). Not in the
// prelude: the least used of the three collections, needs an
// explicit `use` (unlike Vec and String).
pub fn hashmaps_creation_demo() {
    println!("\n*** Hash maps creation demo ***");

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
    println!("\n*** Hash maps creation via collect demo ***");

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
    println!("\n*** Hash maps accessing values demo ***");

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
    println!("\n*** Hash maps iteration demo ***");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

// `insert` takes ownership: owned keys/values (String) are MOVED
// into the map and become invalid at the call site; Copy types
// (i32) are copied and stay usable. References can't just be
// inserted -- their data must outlive the map (lifetimes, ch 10).
pub fn hashmaps_managing_ownership_demo() {
    println!("\n*** Hash maps managing ownership demo ***");

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // The next line fails with "borrow of moved value: `field_name`"
    // (the book invites you to try it):
    // println!("{field_name:?}");

    // The next line fails with "borrow of moved value: `field_value`"
    // (the book invites you to try it):
    // println!("{field_value:?}");

    // The data lives in the map now:
    println!("map: {map:?}");

    // Copy types are copied, not moved -- still usable after insert:
    let mut counts = HashMap::new();
    let number = 10;
    counts.insert("Blue", number);
    println!("number is still valid: {number}");
}

// Updating strategy 1 of 3: `insert` on an existing key REPLACES
// the value outright -- the old value is dropped.
pub fn hashmaps_overwriting_value_demo() {
    println!("\n*** Hash maps overwriting value demo ***");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    println!("HashMap with original value: {scores:?}");

    let old = scores.insert(String::from("Blue"), 25);
    println!("insert returned the old value: {old:?}"); // Some(10)
    println!("HashMap with overwritten value: {scores:?}");
}

// Updating strategy 2 of 3: insert only if the key is ABSENT.
//
// Why a dedicated API exists -- `get` vs `entry`:
//
//   get:   fn get(&self, k: &Q) -> Option<&V>
//     -- immutable borrow of the map, read-only lookup: it cannot
//        insert anything, and the Option<&V> it returns cannot be
//        mutated through.
//
//   entry: fn entry(&mut self, key: K) -> Entry<K, V>
//     -- mutable borrow, and the key is taken by VALUE (it may be
//        needed for the insert). Returns an Occupied/Vacant "slot",
//        and or_insert(default) returns &mut V in BOTH cases --
//        inserting the default for a vacant key, keeping the old
//        value for an occupied one.
//
// A manual check-then-insert would take two lookups and trips the
// borrow checker: the check borrows the map immutably, the insert
// needs it mutably. `entry` fuses check-and-insert into one call --
// "plays more nicely with the borrow checker" (the book's words).
pub fn hashmaps_insert_if_absent_demo() {
    println!("\n*** Hash maps insert if absent demo ***");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    println!("HashMap with original value: {scores:?}");

    scores.entry(String::from("Yellow")).or_insert(50);
    println!("HashMap with the additional entry: {scores:?}");

    scores.entry(String::from("Blue")).or_insert(50);
    println!("HashMap didn't change: {scores:?}");
}
