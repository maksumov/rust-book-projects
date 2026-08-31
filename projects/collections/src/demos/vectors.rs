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

// Two ways to read: `&v[i]` panics on an invalid index, while
// `v.get(i)` returns Option<&T> -- the failure mode is the choice.
pub fn vectors_reading_elements_demo() {
    println!("\n*** Vector reading elements demo ***");
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Indexing with [] panics on out-of-bounds: uncommenting the next
    // line would abort with "index out of bounds: the len is 5 but
    // the index is 100".
    // let does_not_exist = &v[100];

    // `.get` returns Option<&T> instead: None for a missing index --
    // the safe choice when the index may be invalid.
    let does_not_exist = v.get(100);
    println!("{does_not_exist:?} when trying to get an element outside the vector");
}
