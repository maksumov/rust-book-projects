// `String::from(...)` and `.to_string()` are equivalent ways to
// build from a literal; `String::new()` gives an empty string.
// Note: unlike `Vec::new()`, no type annotation is needed here --
// `String::new()` returns the concrete `String`, not a generic.
pub fn strings_creation_demo() {
    println!("\n*** Strings creation demo ***");

    let empty = String::new();

    println!("Empty string initialized via String::new(): {empty:?}");

    let data = "first initial contents";

    let string_from_literal = data.to_string();

    // The method also works on a literal directly:
    let string_to_string_direct = "second initial contents".to_string();

    println!(
        "New strings created using .to_string() method: {string_from_literal:?} and {string_to_string_direct:?}"
    );

    let string_from_associated_fn = String::from("third initial contents");

    println!(
        "New string created using String::from associated function: {string_from_associated_fn:?}"
    );

    let different_hellos = [
        String::from("السلام عليكم"),
        String::from("Dobrý den"),
        String::from("Hello"),
        String::from("שלום"),
        String::from("नमस्ते"),
        String::from("こんにちは"),
        String::from("안녕하세요"),
        String::from("你好"),
        String::from("Olá"),
        String::from("Здравствуйте"),
        String::from("Hola"),
    ];

    println!("Hellos in different languages: {different_hellos:#?}");
}

// Appending: `push_str` takes &str and borrows, so the appended
// variable stays usable (listing 8-16); `push` takes a single char
// by value -- chars are Copy, so the variable survives regardless.
pub fn strings_appending_demo() {
    println!("\n*** Strings appending demo ***");

    let mut s = String::from("foo");
    let bar = "bar";
    let ch = 'c';

    println!("String after creation: {s:?}");

    s.push_str(bar);
    println!("String after appending {bar:?} using .push_str() method: {s:?}");
    println!("bar: &str is still valid: {bar:?}");

    s.push(ch);
    println!("String after appending {ch:?} using .push() method: {s:?}");
    println!("ch: char is still valid (chars are Copy): {ch:?}");
}

// Concatenation with `+`: `s1 + &s2` is sugar for `s1.add(&s2)` with
// `fn add(self, s: &str) -> String` -- `self` by value (s1 is MOVED
// and dies), `s` as &str (s2 survives: borrowed via deref coercion
// &String -> &str, chapter 4/15 preview).
pub fn strings_concatenation_demo() {
    println!("\n*** Strings concatenation demo ***");

    // Note the asymmetry: `s1` without `&`, `s2` with `&` -- exactly
    // the `add(self, s: &str)` signature from the comment above:
    // only the LEFT operand is consumed.
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("s3 is {s3:?}");
    // The next line fails with "borrow of moved value: `s1`":
    // println!("{s1:?}");

    // Chaining works (each `+` consumes the previous String result)
    // but gets unwieldy fast -- hard to read, easy to mess up the &.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {s:?}");

    // `format!` formats like println! but RETURNS a String; it borrows
    // every argument, so nothing is consumed -- the idiomatic choice.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("s is {s:?}");
    println!("s1 is still valid after format!: {s1:?}");
}
