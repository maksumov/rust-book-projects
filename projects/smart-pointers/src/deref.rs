// ch 15.2: Deref -- treating smart pointers like regular references
// (listings 15-6..15-13).

use std::ops::Deref;

pub fn demo_deref_operator() {
    println!(
        "\n*** demo of using the dereference operator to follow a reference to an i32 value ***"
    );

    // Listing 15-6: y is an arrow to x -- assertions must follow the
    // reference with * to reach the value (5 == &5 would not compile).
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Beyond the book: making the reference itself visible. {:?} on &i32
    // transparently formats the pointee (prints 5, not the arrow); the
    // Pointer trait's {:p} is the dedicated address formatter.
    println!("x = {x}, living at {:p}", &x);
    println!("y points to {:p}, and *y = {}", y, *y); // same address as &x
    assert_eq!(format!("{y:p}"), format!("{:p}", &x)); // y really is the arrow to x
}

pub fn demo_deref_on_box() {
    println!("\n*** demo of using the dereference operator on a Box<i32> ***");

    // Listing 15-7: the same *y works on a Box -- but unlike 15-6, y holds
    // a COPIED value of x allocated on the heap, not an arrow to x itself.
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Beyond the book: the {:p} trick again -- this time the addresses
    // DIFFER: stack (x) vs heap (the Box's copied 5).
    println!("x = {x}, living at {:p}", &x); // a stack address
    println!("y points to {:p}, and *y = {}", y, *y); // a heap address, not &x
}

// Listing 15-8: a Box-like wrapper. Unlike the real Box<T>, it does NOT
// put its data on the heap -- the point is the pointer-like BEHAVIOR
// (Deref), not the storage. Side effect: our {:p} trick does not apply
// here -- Pointer is an opt-in trait MyBox does not implement.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Listing 15-9 (the attempt without Deref) fails -- the * operator only
// works on references and Deref types:
//
// // error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
// //   --> src/main.rs:14:19
// //    |
// // 14 |     assert_eq!(5, *y);
// //    |                   ^^ can't be dereferenced

// Listing 15-10: implementing Deref teaches the compiler to dereference
// us. The *y sugar desugars to *(y.deref()) -- ONE call, then a plain
// dereference, so the substitution never recurses infinitely. deref
// must return a reference: returning the value would MOVE it out of
// self (ownership, ch 4).
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn demo_my_box() {
    println!("\n*** demo of using the dereference operator on a MyBox<i32> ***");

    // Listings 15-9..15-10: the assert trick now compiles -- *y on a
    // MyBox<i32> goes through our deref.
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // No {:p} here: Pointer is opt-in and MyBox does not implement it
    // (see the 15-8 note) -- *y speaks for the deref going through.
    println!("x = {x}, *y = {}", *y);
}

// Listings 15-11..15-12: hello takes &str, but &m is a &MyBox<String>.
// Deref coercion inserts deref calls as many times as needed, resolved
// entirely at COMPILE time (no runtime cost): &MyBox<String> -> &String
// -> &str -- the second hop is String's own Deref impl in std.
pub fn demo_deref_coercion() {
    println!("\n*** demo of using the dereference coercion ***");

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // the coercion chain at work

    // Listing 15-13: without coercion, the same call would need the full
    // manual unwrapping -- deref to the String, then slice it whole:
    //
    // hello(&(*m)[..]);
}

// Listing 15-11: a plain &str parameter -- the TARGET type that coercion
// converts to. Signatures stay reference-simple thanks to Deref.
fn hello(name: &str) {
    println!("Hello, {name}!");
}

// The three coercion cases (Deref / DerefMut):
//   &T     -> &U     when T: Deref<Target = U>
//   &mut T -> &mut U when T: DerefMut<Target = U>
//   &mut T -> &U     when T: Deref<Target = U>
// The reverse (&T -> &mut T) NEVER happens: many shared references may
// exist, so promoting one to unique would break the borrowing rules.
