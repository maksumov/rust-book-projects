// Creating an iterator does nothing by itself -- iterators are
// lazy; the for loop consumes it here, calling next under the hood.
pub fn demo() {
    println!("\n*** iterators usage demo ***");

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }
}
