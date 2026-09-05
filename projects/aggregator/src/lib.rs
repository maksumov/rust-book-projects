use std::clone::Clone;
use std::fmt::{self, Debug, Display};

// A library crate consumed by the binary in src/main.rs -- the
// lib+bin pattern (ch 7.3 best practice; first in this collection).
// Everything `pub` here is the public API the bin gets to see.

// The subsection's finale: a REQUIRED method (summarize_author)
// plus a default that CALLS it -- implementors provide the small
// required part and get the rest for free. Note: an override
// cannot call the default it replaces.
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implements only the required part -- inherits the calling default
impl Summary for NewsArticle {
    // Beyond the book (it keeps only SocialPost at this step):
    // the now-required summarize_author needs an impl here too.
    fn summarize_author(&self) -> String {
        format!("by {}", self.author)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    // The summarize override is GONE: the default (built on
    // summarize_author) takes over.
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // Overrides default implementation (mostly unchanged from 10-13):
    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content)
    }
}

// "Using Traits as Parameters": &impl Trait accepts any type
// implementing the trait -- sugar for a trait bound
// (<T: Summary>), which the next fragments introduce.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// The equivalent long form: &impl Summary is sugar for this trait
// bound. Same semantics; the full form can express what sugar
// cannot -- see notify_two_same below.
pub fn notify_bound<T: Summary>(item: &T) {
    println!("Bound form: {}", item.summarize());
}

// Two params of possibly DIFFERENT types: each &impl is its own type.
pub fn notify_two_different(item1: &impl Summary, item2: &impl Summary) {
    println!(
        "Two different: {} | {}",
        item1.summarize(),
        item2.summarize()
    );
}

// A single bound forces BOTH params to be the SAME type -- the
// semantic difference the sugar form cannot express.
pub fn notify_two_same<T: Summary>(item1: &T, item2: &T) {
    println!("Two alike: {} | {}", item1.summarize(), item2.summarize());
}

// Multiple bounds with +: Display enables {}-printing of the item
// itself, Summary enables summarize().
pub fn notify_display<T: Summary + Display>(item: &T) {
    println!("{} says: {}", item, item.summarize());
}

// Where clause: the same bounds, moved after the signature --
// readability for multi-generic functions (the book's own name).
pub fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("where accepts Display t: {t} and Debug u: {u:?}");
    0
}

// Beyond the book: Display for SocialPost lets notify_display be
// called with a live demo type (NewsArticle deliberately stays
// without it -- see the commented error in main).
impl Display for SocialPost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}", self.username)
    }
}

// "Returning Types That Implement Traits": the concrete type stays
// hidden from the caller -- especially useful for iterator and
// closure types (chapter 13). Only ONE concrete type may be
// returned; see the commented switch variant below (trait objects,
// chapter 18, lift that restriction).
pub fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    }
}

// The book's "single type only" lesson: returning DIFFERENT types
// through impl Trait fails with E0308:
//     "`if` and `else` have incompatible types"
//     expected `NewsArticle`, found `SocialPost`
// The compiler's help even suggests the fix: "you could change the
// return type to be a boxed trait object" -- Box<dyn Trait>,
// covered in chapter 18. Uncomment and build to see it.
// pub fn returns_summarizable_switch(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         SocialPost {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             repost: false,
//         }
//     }
// }
