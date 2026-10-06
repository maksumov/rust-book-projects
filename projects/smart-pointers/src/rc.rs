// ch 15.4: Rc<T> -- the reference-counted smart pointer (listings
// 15-17..15-19). Multiple owners of one value; it is dropped when the
// LAST owner goes away. Single-threaded only (the multithreaded sibling
// is Arc, ch 16). Rc is not in the prelude -- hence the use below.

use std::rc::Rc;

use List::{Cons, Nil};

// Listing 15-18: the tail is Rc<List> -- cloning bumps the reference
// counter instead of moving or copying the list.
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn demo_list() {
    println!("\n*** demo of a List with shared ownership via Rc<T> ***");

    // Listing 15-17 (the Box version) cannot share a list -- ownership moves:
    //
    // // enum List {
    // //     Cons(i32, Box<List>),
    // //     Nil,
    // // }
    //
    // // let b = Cons(3, Box::new(a)); // b takes ownership of a...
    // // let c = Cons(4, Box::new(a)); // ...so this is use after move
    //
    // error[E0382]: use of moved value: `a`
    //   --> src/main.rs:11:30
    //    |
    //  9 |     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    //    |         - move occurs because `a` has type `List`, which does not
    //    |           implement the `Copy` trait
    // 10 |     let b = Cons(3, Box::new(a));
    //    |                              - value moved here
    // 11 |     let c = Cons(4, Box::new(a));
    //    |                              ^^^^ value used here after move

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); // Rc::clone: bump the counter, not a deep copy
    let c = Cons(4, Rc::clone(&a)); // the convention: Rc::clone(&a), not a.clone()

    // beyond the book: Debug shows each list embedding the SHARED tail --
    // the (5, 10) links exist once but are reachable from b and c alike.
    println!("a is {a:?}");
    println!("b is {b:?}");
    println!("c is {c:?}");

    // beyond the book: the box_demo sum() walk, now proving shared
    // reachability -- both sums descend into the SAME nodes.
    fn sum(list: &List) -> i32 {
        match list {
            Cons(value, tail) => value + sum(tail),
            Nil => 0,
        }
    }

    println!("sum of the b values = {}", sum(&b));
    println!("sum of the c values = {}", sum(&c));
}

// Listing 15-19: strong_count through the lifecycle. The inner scope is
// the point: closing it drops _c, and Rc's OWN Drop decrements the
// counter -- the "counters, not memory" case from drop_demo's header.
// Nobody calls any decrement manually: clone up, scope-end down.
// The name is strong_count, not count: Rc also tracks a weak_count --
// Weak<T> references (Rc::downgrade) that do NOT keep the value alive;
// they arrive in 15.6 as the reference-cycle antidote.
pub fn demo_strong_count() {
    println!("\n*** demo of the strong count of a shared Rc<List> ***");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
