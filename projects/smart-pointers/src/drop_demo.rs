// ch 15.3: Drop -- running code on cleanup (listings 15-14..15-16).
// The second half of the smart pointer pattern (Deref + Drop): code
// that runs when a value goes out of scope -- releasing files, locks,
// heap memory.

// Listing 15-14: no explicit drop calls anywhere -- Rust inserts them at
// scope end, in REVERSE creation order (d would go first; the early
// drop(c) added by 15-16 below is what reshuffles this demo's output).
#[derive(Debug)] // beyond the book: to show the values in the demo output
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn demo_drop_example() {
    println!("\n*** demo of smart pointers with implemented Drop trait ***");

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created: {:?} {:?}", c, d);

    // Listing 15-15: dropping early by CALLING THE METHOD is forbidden --
    // the reason is double free:
    //
    // c.drop();
    //
    // error[E0040]: explicit use of destructor method
    //   --> src/main.rs:16:7
    //    |
    // 16 |     c.drop();
    //    |       ^^^^ explicit destructor calls not allowed
    //    |
    // help: consider using `drop` function
    //    |
    // 16 -     c.drop();
    // 16 +     drop(c);
    //
    // Rust would STILL auto-drop c at the end of the scope, cleaning the
    // same value twice. "Destructor" is the general term for a cleanup
    // function -- the counterpart of a constructor.

    // Listing 15-16: the legal early drop -- std::mem::drop, a prelude
    // FUNCTION taking the value (ending its ownership), not a method:
    drop(c);
    println!("CustomSmartPointer dropped before the end of the demo");
    // The output proves the timing: c's "Dropping" line appears between
    // the two printlns, while d still drops at the end of the scope.
}
