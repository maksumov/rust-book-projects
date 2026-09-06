// The heart of 10.3: `longest` evolves from a compile error to a
// fully annotated signature, then gets stress-tested at the call
// sites. Broken variants are commented with verbatim errors.

pub fn demo() {
    println!("\n--- longest: lifetime annotations in a signature ---");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    // Listing 10-21: the fixed signature -- one lifetime 'a ties
    // BOTH inputs and the output: the returned reference is valid
    // as long as BOTH arguments are ('a = the SHORTER of the two).
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // Listing 10-20: the same function WITHOUT annotations.
    // Uncomment and build to see the error and its golden help:
    //     error[E0106]: missing lifetime specifier
    //       expected named lifetime parameter
    //     = help: this function's return type contains a borrowed
    //       value, but the signature does not say whether it is
    //       borrowed from `x` or `y`
    //     help: consider introducing a named lifetime parameter
    //       fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() { x } else { y }
    // }

    // Listing 10-22: different lifetimes, but the USE stays inside
    // the shorter one -- the borrow checker approves:
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    // Listing 10-23: the result escapes the shorter lifetime.
    // Uncomment and build to see the error:
    //     error[E0597]: `string2` does not live long enough
    //         borrowed value does not live long enough
    //         `string2` dropped here while still borrowed
    //         borrow later used here
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");
}

// Listing 10-21: 'a ties the references together -- annotations do
// NOT change how long anything lives; they describe the RELATION.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
