// An immutable capture coexists with other readers of `list`.
pub fn demo_immutable() {
    println!("\n*** capturing references: immutable reference ***");

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}

// The mutable borrow spans from the closure definition to its
// last use, so no other borrows are allowed in between.
pub fn demo_mutable() {
    println!("\n*** capturing references: mutable reference ***");

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    // Uncomment next line to see the error:
    // println!("Before calling closure: {list:?}");
    //
    // error[E0502]: cannot borrow `list` as immutable because it is also borrowed as mutable
    borrows_mutably();
    println!("After calling closure: {list:?}");
}

// The spawned thread may outlive this function, so a borrowed
// `list` could dangle; `move` transfers ownership to the thread.
pub fn demo_moving_ownership() {
    use std::thread;

    println!("\n*** capturing references: moving ownership ***");

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // without `move` in the next line, we receive the following error:
    // error[E0373]: closure may outlive the current function, but it
    // borrows `list`, which is owned by the current function
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
