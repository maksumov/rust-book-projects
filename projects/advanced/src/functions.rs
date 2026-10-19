// ch 20.4: advanced functions and closures (listings 20-28..20-34).
// Function pointers (fn - a TYPE, not the Fn trait), named fns and
// enum initializers where closures are expected, returning closures:
// impl Trait (one opaque type per use) vs Box<dyn Fn> (one type for
// many closures).

// Listing 20-28: the fn TYPE (lowercase f) - a function pointer, a
// concrete type written directly, no generic bound needed. Function
// names ARE pointers; fn implements all three closure traits, so a
// generic + Fn API accepts both (fn-only APIs are the C-FFI niche).
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

pub fn demo_fn_pointers() {
    println!("\n*** demo of advanced functions: function pointers ***");

    // Listing 20-28: the fn name passed as the pointer; 12.
    let answer = do_twice(add_one, 5);
    println!("The answer is: {answer}");

    // Listings 20-29/20-30: the same map either way - an inline
    // closure, or the named fn as a pointer. The fn call needs
    // FULLY QUALIFIED syntax (20.2): several to_string exist, this
    // one is ToString's (implemented for every Display). (The book's
    // vec! is trimmed to an array here - clippy::useless_vec.)
    let list_of_numbers = [1, 2, 3];
    let a: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let b: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("closure: {:?}, fn: {:?}", a, b);

    // Listing 20-31: an enum variant's initializer IS a function -
    // usable as a pointer too. The enum is declared fn-level exactly
    // as the book does inside main; the values are read back (sum)
    // so nothing dangles dead.
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    let total: u32 = list_of_statuses
        .iter()
        .map(|s| match s {
            Status::Value(v) => *v,
            Status::Stop => 0,
        })
        .sum();
    let _ = Status::Stop;
    println!(
        "statuses: {}, values sum: {}",
        list_of_statuses.len(),
        total
    );
}

// Listing 20-32: returning a closure - impl Trait works when each
// fn returns ONE closure shape.
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

// Listing 20-33: two impl-Fn returns in one Vec do NOT mix: every
// impl Trait use is a DISTINCT opaque type - same signature, two
// types (as with async blocks, ch 17). The compiler's own words:
//
// // let handlers = vec![returns_closure(), returns_initialized_closure(123)];
// // error[E0308]: mismatched types
// //   = note: expected opaque type `impl Fn(i32) -> i32`
// //          found opaque type `impl Fn(i32) -> i32`
// //   = note: distinct uses of `impl Trait` result in different
// //          opaque types

// Listing 20-34: the fix - one trait-object type for all of them
// (the book reuses the name returns_closure; renamed here so the
// 20-32 impl-Fn twin can stay live). The Thunk of 20-26 in action;
// dyn Fn is a DST (20.3), hence the fat pointer - and the shared
// type that lets them sit in one Vec.
fn returns_boxed_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}

pub fn demo_returning_closures() {
    println!("\n*** demo of advanced functions: returning closures ***");

    // Listing 20-32: one fn, one closure shape - impl Trait is enough.
    let f = returns_closure();
    println!("returns_closure(5): {}", f(5));

    // Listing 20-34: many closures, one Vec - Box<dyn Fn> gives them
    // all the same type.
    let handlers: Vec<Box<dyn Fn(i32) -> i32>> =
        vec![returns_boxed_closure(), returns_initialized_closure(123)];
    for handler in handlers {
        let output = handler(5);
        println!("{output}");
    }
}
