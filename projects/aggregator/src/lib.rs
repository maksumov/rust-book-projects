// A library crate consumed by the binary in src/main.rs -- the
// lib+bin pattern (ch 7.3 best practice; first in this collection).
// Everything `pub` here is the public API the bin gets to see.

pub trait Summary {
    // Listing 10-14: a DEFAULT implementation -- types may keep it
    // (empty impl block) or override it (the overriding syntax is
    // exactly the implementing syntax).
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Keeps the trait default as-is:
impl Summary for NewsArticle {}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    // Overrides default implementation (unchanged from 10-13):
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
