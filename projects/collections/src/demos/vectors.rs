// Two ways to create: `Vec::new()` gives an empty vector (the type
// annotation is required -- nothing to infer from), while the
// `vec!` macro creates one with initial values (type inferred).
pub fn vectors_creation_demo() {
    println!("\n*** Vectors creation demo ***");

    let v: Vec<i32> = Vec::new();

    println!("Empty vector initialized via Vec::new(): {v:?}");

    let v = vec![1, 2, 3];

    println!("New vector containing values: {v:?}");
}

// Mutation requires `mut`. `push` takes ownership of the value;
// when the vector goes out of scope, all of its elements are
// dropped along with it (chapter 4 ownership at work).
pub fn vectors_updating_demo() {
    println!("\n*** Vector updating demo ***");

    let mut v: Vec<u8> = Vec::new();
    println!("Vector right after creation: {v:?}");

    for value in 5..=8 {
        v.push(value);
        println!("Vector after pushing {value} into it: {v:?}");
    }
}
