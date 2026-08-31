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

// Borrow conflict (listing 8-4): holding `&v[0]` while pushing is
// rejected -- `push` may reallocate the buffer and dangle `first`
// (chapter 4: no & and &mut at the same time, even if this push
// wouldn't actually reallocate -- the compiler can't know that).
pub fn vectors_borrow_conflict_demo() {
    println!("\n*** Vector borrow conflict demo ***");
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    // The next line fails with "cannot borrow `v` as mutable because
    // it is also borrowed as immutable": `first` is still used below.
    // v.push(6);
    println!("The first element is: {first}");

    // Fine now: `first`'s last use was the println above (NLL),
    // so the borrow ends before push.
    v.push(6);
    println!("After push: {v:?}");
}

// Iterating: `for i in &v` yields references (read-only); the
// mutable variant (`&mut v`) yields &mut and needs `*` -- the
// dereference operator -- to modify the element in place.
pub fn vectors_iteration_demo() {
    println!("\n*** Vector iteration demo ***");
    let mut v = vec![100, 200, 300];
    println!("After creation: {v:?}");

    // Immutable iteration: i is &i32
    for i in &v {
        println!("{i}");
    }

    // Mutable iteration: i is &mut i32, `*i` writes through the reference
    for i in &mut v {
        *i += 50;
    }
    println!("After mutable iteration: {v:?}");
}

// Vectors are homogeneous: one element type. To store "different"
// values, define an enum whose variants carry each type -- every
// element is then the SAME enum type (chapter 6 at work). A match
// on the variant unpacks the value with its type back.
#[derive(Debug)]
enum Cell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn vectors_multiple_types_demo() {
    println!("\n*** Vector multiple types demo ***");

    let row = vec![
        Cell::Int(3),
        Cell::Text(String::from("blue")),
        Cell::Float(10.12),
    ];

    for cell in &row {
        match cell {
            Cell::Int(i) => println!("Int cell: {i}"),
            Cell::Float(f) => println!("Float cell: {f}"),
            Cell::Text(s) => println!("Text cell: {s}"),
        }
    }
}
