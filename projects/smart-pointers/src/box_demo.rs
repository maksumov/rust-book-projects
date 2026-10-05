// ch 15.1: Box<T> -- the most basic smart pointer (heap allocation,
// listings 15-1..15-5). The cons list joining this module later.

pub fn demo_box() {
    println!("\n*** demo of using Box to store an i32 value on the heap ***");

    // Listing 15-1: a single i32 on the heap. Not something you would
    // normally do -- a plain stack i32 is the better fit -- but it shows
    // that a Box behaves like any owned value: dereferences for use, and
    // when it leaves scope BOTH the stack pointer and the heap data are
    // deallocated.
    let b = Box::new(5);
    println!("b = {b}");
}
