// Two ways to create: `Vec::new()` gives an empty vector (the type
// annotation is required -- nothing to infer from), while the
// `vec!` macro creates one with initial values (type inferred).
pub fn vectors_creation_demo() {
    println!("*** Vectors creation demo ***");

    let v: Vec<i32> = Vec::new();

    println!("Empty vector initialized via Vec::new(): {v:?}");

    let v = vec![1, 2, 3];

    println!("New vector containing values: {v:?}");
}
