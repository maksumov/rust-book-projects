// ch 15.1: Box<T> -- the most basic smart pointer (heap allocation,
// listings 15-1..15-5). The cons list is this section's capstone: the
// use case where indirection is the ONLY feature needed.

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

// Listings 15-2..15-4: the first attempt cannot compile -- the compiler
// cannot compute a size for a type recursive WITHOUT indirection:
//
// // enum List {          // error[E0072]: recursive type `List` has infinite size
// //     Cons(i32, List), // help: insert some indirection (e.g., a `Box`, `Rc`,
// //     Nil,             // or `&`) to break the cycle
// // }
//
// Size math: Cons needs i32 + List, List needs max(Cons, Nil) = i32 +
// (i32 + ...) -- ad infinitum (figure 15-1). A pointer's size never
// depends on the pointee, so Box<List> in the tail ends the recursion
// (figure 15-2): every List value on the stack is exactly i32 + one
// pointer, the links live on the heap.

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn demo_cons() {
    use List::{Cons, Nil};

    println!("\n*** demo of a recursive type enabled by Box indirection ***");

    // Listing 15-5: each tail is Box::new(...), putting every link on the
    // heap. The Debug print is beyond the book -- the book prints nothing
    // here (and returns to this idea with derive(Debug) in listing 15-24).
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{list:?}");

    // Beyond the book: the canonical cons-list operation -- walk the
    // links, sum the values; the recursion mirrors the type's shape.
    // Bonus: this is what silences the dead-code warning above -- a
    // derived Debug is intentionally IGNORED by that analysis, so
    // printing alone left the fields "never read".
    fn sum(list: &List) -> i32 {
        match list {
            Cons(value, tail) => value + sum(tail),
            Nil => 0,
        }
    }

    println!("sum of the values = {}", sum(&list));
}
