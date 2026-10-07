// ch 15.5 (part 2): Rc<RefCell<i32>> -- multiple owners AND mutation
// (listing 15-24). Rc alone gives shared READ access; a RefCell inside
// lets a mutation through one handle reach every owner at once.
// Single-threaded only (the threaded sibling is Mutex, ch 16).

// Listing 15-24: each head holds Rc<RefCell<i32>> -- ONE shared,
// mutable cell per value.
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

pub fn demo_refcell() {
    println!("\n*** demo of a List with internal mutability via RefCell<T> ***");

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("a before = {a:?}");
    println!("b before = {b:?}");
    println!("c before = {c:?}");

    // The key line -- three mechanics in one statement: deref the Rc
    // (automatic), borrow_mut the RefCell (returns a RefMut guard),
    // deref THAT to reach the i32. Through ONE handle -- a, b and c
    // all observe 15 afterwards (the before/after prints are beyond
    // the book: it prints only "after").
    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");

    // beyond the book: the sum() walk adapted -- each head now needs
    // *value.borrow() to read through its RefCell (rc.rs's version read
    // the plain i32). One borrow at a time, so no runtime panic.
    fn sum(list: &List) -> i32 {
        match list {
            Cons(value, tail) => *value.borrow() + sum(tail),
            Nil => 0,
        }
    }

    println!("sum of the a values = {}", sum(&a));
    println!("sum of the b values = {}", sum(&b));
    println!("sum of the c values = {}", sum(&c));
}

// The section's decision table:
//   Box<T>:    single owner; immutable or mutable borrows, compile time
//   Rc<T>:     multiple owners; immutable borrows, compile time
//   RefCell:   single owner; immutable or mutable borrows, RUNTIME
//              (panic on rule violation instead of a compiler error)
