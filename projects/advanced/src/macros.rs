// ch 20.5: macros. Code that writes code, expanded BEFORE
// compilation - before types exist, which is why they can do what
// fns cannot: variable arity (vec!, println!) and syntax-level
// generation. The price: within a crate a macro must be DEFINED
// before its use in source order, and the definition is matched
// token-by-token, not type-checked. Two kinds: declarative
// (macro_rules!, this module) and procedural (a separate
// proc-macro workspace - the hello_macro case, next commits).

// Listing 20-35: the book's sketch of std's vec! - renamed my_vec!
// here (a macro_rules! named vec would shadow the prelude one);
// #[macro_export] publishes it at the crate root (the same
// attribute std uses). Read the pattern: $( $x:expr ),* matches a
// comma-separated repetition of expressions bound to $x; the body
// repeats the pushes in lockstep ($( ... )*) inside a block that
// evaluates to the final v. The empty arm is beyond the book -
// the simplified sketch trips unused_mut on my_vec![] (the mut
// with zero pushes); the REAL vec! carries such arms too.
#[macro_export]
macro_rules! my_vec {
    ( ) => { Vec::new() };
    ( $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            $( v.push($x); )*
            v
        }
    };
}

pub fn demo_declarative_macros() {
    println!("\n*** demo of macros: macro_rules! ***");

    let v = my_vec![1, 2, 3];
    println!("my_vec![1, 2, 3] = {:?}", v);

    // The variadic arity no fn can offer: the same macro, any
    // number of arguments - zero, one, many.
    let empty: Vec<i32> = my_vec![];
    let one = my_vec!["a"];
    println!("empty: {:?}, one: {:?}", empty, one);
}
