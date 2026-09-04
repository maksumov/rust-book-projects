// The consumer imports BOTH the types AND the trait: trait methods
// only exist in scope with the trait imported (glossary: "Trait
// must be in scope").

use aggregator::{SocialPost, Summary};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());
}
