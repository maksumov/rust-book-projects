// ch 15.6: reference cycles can leak memory (listings 15-25..15-29).
// Rc + RefCell lets links be rewired at runtime -- including into a
// cycle, where strong counts never reach zero and the memory leaks.
// Memory leaks are MEMORY SAFE in Rust: preventing them is not a
// guarantee; a cycle is a logic bug. The antidote: Weak<T>.

use List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

// Listing 15-25: the tail slot is now RefCell<Rc<List>> -- rewirable;
// tail() exposes it for the demo.
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// Listing 15-26: create a, then b pointing at a, then REWIRE a's
// tail to b -- a cycle. Both counts freeze at 2 and never reach 0:
// the memory is never dropped. Rust does not catch this.
pub fn demo_cycle() {
    println!("\n*** demo of a cycle ***");

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    // println!("a next item = {:?}", a.tail());

    // beyond the book: the cycle makes a full walk impossible -- the
    // box_demo/rc.rs sum() would recurse forever (the same reason the
    // Debug print above overflows the stack). A depth guard terminates
    // AND makes the cycle visible in the values themselves:
    fn sum_up_to(list: &List, depth: usize) -> i32 {
        match list {
            Cons(value, tail) if depth > 0 => value + sum_up_to(&tail.borrow(), depth - 1),
            Cons(_, _) | Nil => 0,
        }
    }
    println!("sum depth 4 from a = {}", sum_up_to(&a, 4)); // 5+10+5+10: the cycle repeats
    println!("sum depth 2 from b = {}", sum_up_to(&b, 2)); // 10+5, cut by the guard
}
