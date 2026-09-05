// The consumer imports BOTH the types AND the trait: trait methods
// only exist in scope with the trait imported (glossary: "Trait
// must be in scope").

use aggregator::{NewsArticle, SocialPost, Summary};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("..."),
    };
    // The trait evolved (summarize_author finale) -- the article's
    // output changed WITHOUT touching main: now prints
    // "(Read more from by Iceburgh...)":
    println!("New article available! {}", article.summarize());
}
