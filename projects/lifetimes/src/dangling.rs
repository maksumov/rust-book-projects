// Lifetimes of chapter 10.3: every reference has one, and the
// borrow checker compares them to reject dangling references.
// Broken variants live here commented, with verbatim compiler
// errors (see main.rs for the orchestration).

pub fn demo() {
    println!("\n--- dangling references: scopes and the borrow checker ---");

    // Listing 10-18: the VALID version -- x ('b) outlives r ('a):
    let x = 5; //            ----------+-- 'b
    let r = &x; // --+-- 'a           |
    //                |
    println!("r: {r}"); //  |  (prints 5: the reference is valid)
    // ---------------+
    // The reference's lifetime 'a fits inside the data's 'b.

    // Listing 10-16: the BROKEN version, with the book's scope
    // annotations from 10-17. Uncomment and build to see the error:
    //     error[E0597]: `x` does not live long enough
    //         `x` dropped here while still borrowed
    //         borrow later used here
    //
    // let r;                // ---------+-- 'a
    // {                     //          |
    //     let x = 5;        // -+-- 'b  |
    //     r = &x;           //  |       |
    // }                     // -+  <-- x dies, r still borrowed
    // println!("r: {r}");   //          |
    //                       // ---------+
}
