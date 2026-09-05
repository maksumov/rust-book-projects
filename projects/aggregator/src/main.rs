// The consumer imports BOTH the types AND the trait: trait methods
// only exist in scope with the trait imported (glossary: "Trait
// must be in scope").

use aggregator::{self, NewsArticle, SocialPost, Summary};

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

    println!("\n--- &impl Summary (sugar): any implementing type ---");
    aggregator::notify(&post);
    aggregator::notify(&article);

    println!("\n--- <T: Summary> (the bound form) ---");
    aggregator::notify_bound(&post);

    println!("\n--- two params: different types allowed ---");
    aggregator::notify_two_different(&post, &article);

    println!("\n--- two params: same type forced ---");
    aggregator::notify_two_same(&post, &post);

    // The next line fails with E0308 "mismatched types":
    // expected `&SocialPost`, found `&NewsArticle`
    // aggregator::notify_two_same(&post, &article);

    println!("\n--- multiple bounds: Summary + Display ---");
    aggregator::notify_display(&post);

    // The next line fails with E0277: "`NewsArticle` doesn't
    // implement `std::fmt::Display`"
    // aggregator::notify_display(&article);

    println!("\n--- where clause ---");
    // Simple types fit the bounds: i32: Display + Clone,
    // String: Clone + Debug:
    aggregator::some_function(&42, &String::from("x"));

    println!("\n--- return summarizable ---");
    // The opaque `impl Summary` exposes ONLY the trait's methods:
    // no Debug/Display, the concrete type is not even nameable.
    // Print through the trait API:
    println!("{}", aggregator::returns_summarizable().summarize());
}
