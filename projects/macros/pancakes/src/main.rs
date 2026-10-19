// ch 20.5: the user side (listings 20-37..20-39).

// Listing 20-39: what the user wrote before the derive existed -
// kept as a comment; the impl is now generated:
//
// // impl HelloMacro for Pancakes {
// //     fn hello_macro() {
// //         println!("Hello, Macro! My name is Pancakes!");
// //     }
// // }

// Listing 20-37: the goal reached - one attribute, no impl block.
// Both crates in scope: the trait from hello_macro, the derive
// macro from hello_macro_derive.
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
