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
