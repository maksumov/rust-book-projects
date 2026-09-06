// Structs CAN hold references -- at the price of a lifetime
// parameter (this closes the loop with the structs project's
// "missing lifetime specifier" anchor). Elision rule 3 then keeps
// METHOD signatures clean.

pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

// Listing 10-24: an instance cannot outlive the reference it holds.
impl<'a> ImportantExcerpt<'a> {
    // Elision rule 1 applies to &self; the return is not a
    // reference -- no annotations needed at all:
    pub fn level(&self) -> i32 {
        3
    }

    // Elision rule 3: two input lifetimes (&self, announcement),
    // one of them is &self -> ALL outputs get self's lifetime:
    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

pub fn demo() {
    println!("\n--- structs with references + method elision ---");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("level = {}", i.level());
    println!("part = {}", i.announce_and_return_part("a excerpt demo"));

    // The static lifetime: 'static -- valid for the ENTIRE program.
    // Every string literal is 'static (stored in the binary):
    let s: &'static str = "I have a static lifetime.";
    println!("{s}");

    // Caution from the book: an error suggesting 'static is usually
    // a dangling-reference smell -- fix the lifetimes instead of
    // reaching for 'static.
}
