// ch 17.1: futures and the async syntax (listings 17-1..17-5). A future
// is a value that may not be ready yet; `async fn` compiles to a
// function returning an anonymous Future (the desugared form below).
// Futures are LAZY -- nothing runs until awaited (like iterators,
// unlike threads). The trpl crate wraps futures + tokio; block_on is
// the sync/async bridge. Caveat: network timing varies per run.

use trpl::Html;

// Listing 17-1: an async fn -- await each step that takes time (the
// response headers, then the whole body). Listing 17-2 merely chains
// the same steps: trpl::get(url).await.text().await. Note that await
// is a POSTFIX keyword -- it goes after the expression.
async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

// What the compiler makes of `async fn`: a plain function returning an
// anonymous type implementing Future, body wrapped in an async move
// block -- the book's "roughly equivalent" form:
//
// // use std::future::Future;
// //
// // fn page_title(url: &str) -> impl Future<Output = Option<String>> {
// //     async move {
// //         let text = trpl::get(url).await.text().await;
// //         Html::parse(&text)
// //             .select_first("title")
// //             .map(|title| title.inner_html())
// //     }
// // }

// Listings 17-3..17-4: in the book, MAIN itself tries to be async:
//
// // async fn main() {   // error[E0752]: `main` function is not
// //     ...             // allowed to be `async`
// // }
//
// Someone must RUN the future, and main is the program's starting
// point -- it cannot be that someone. The fix: block_on, a SYNC fn
// blocking its thread until the future completes. (#[tokio::main]
// is exactly this, rewritten by a macro.)
//
// Repo adaptation: our main never attempts this -- it stays sync by
// design and only calls sync demo functions. The sync->async bridge
// lives INSIDE each demo (its own block_on), so E0752 is real for
// the book's layout but hidden here by construction.
pub fn demo_page_title(url: &str) {
    println!("\n*** demo of futures: one URL, one title, via block_on ***");

    trpl::block_on(async {
        match page_title(url).await {
            Some(title) => println!("The title for '{url}' was '{title}'"),
            None => println!("{url} had no title"),
        }
    })
}
