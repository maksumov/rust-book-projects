// ch 20.5: the user side (listings 20-37..20-39). So far the impl
// is MANUAL - one per type, exactly what the derive (next commit)
// spares the user from.

// Listing 20-39: the manual impl - the code #[derive(HelloMacro)]
// will soon generate.
use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
