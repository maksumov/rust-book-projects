// beyond the book
//
// The three iteration forms mirror the ownership triad: immutable
// borrow (iter), mutable borrow (iter_mut), move (into_iter).
// Strings instead of Copy types make the differences visible.

pub fn demo() {
    println!("\n*** iterators: iter / iter_mut / into_iter ***");

    // iter(): borrows immutably -- read-only pass; the owner is
    // untouched and still usable afterwards.
    let words = vec![String::from("read"), String::from("only")];
    let total: usize = words.iter().map(String::len).sum();
    println!("iter: total chars in {words:?} = {total}");

    // iter_mut(): borrows mutably -- transform in place; the same
    // collection, no reallocation.
    let mut words = vec![String::from("shout"), String::from("here")];
    for w in words.iter_mut() {
        w.push_str("!");
    }
    println!("iter_mut: {words:?}");

    // into_iter(): takes ownership -- each String is moved out;
    // the vector dies with the loop.
    let words = vec![String::from("gone"), String::from("soon")];
    for w in words.into_iter() {
        println!("into_iter: owned {w}");
    }
    // println!("{words:?}");
    //
    // error[E0382]: borrow of moved value: `words`
    //    --> src/iteration_forms.rs:30:16
    //     |
    //  26 |     let words = vec![String::from("gone"), String::from("soon")];
    //     |         ----- move occurs because `words` has type `Vec<String>`, which does not implement the `Copy` trait
    //  27 |     for w in words.into_iter() {
    //     |                    ----------- `words` moved due to this method call
    // ...
    //  30 |     println!("{words:?}");
    //     |                ^^^^^ value borrowed here after move
    //     |
    // note: `into_iter` takes ownership of the receiver `self`, which moves `words`
    //    --> .../library/core/src/iter/traits/collect.rs:312:18
    //     |
    // 312 |     fn into_iter(self) -> Self::IntoIter;
    //     |                  ^^^^
    // help: you can `clone` the value and consume it, but this might not be your desired behavior
    //     |
    //  27 |     for w in words.clone().into_iter() {
    //     |                   ++++++++
}
