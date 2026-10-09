// ch 17.1: futures and the async syntax (listings 17-1..17-5). A future
// is a value that may not be ready yet; `async fn` compiles to a
// function returning an anonymous Future (the desugared form below).
// Futures are LAZY -- nothing runs until awaited (like iterators,
// unlike threads). The trpl crate wraps futures + tokio; block_on is
// the sync/async bridge. Caveat: network timing varies per run.

use trpl::{Either, Html};

// Listings 17-1..17-2, evolved for 17-5: the chained form (17-2) --
// await is a POSTFIX keyword -- with the URL riding along in the
// return: the race needs to name its winner.
async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
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
        let (_, maybe_title) = page_title(url).await;
        match maybe_title {
            Some(title) => println!("The title for '{url}' was '{title}'"),
            None => println!("{url} had no title"),
        }
    })
}

// Listing 17-5: both futures are CREATED but not awaited -- laziness
// is what makes the race possible (creating != running, unlike
// threads). select awaits whichever finishes FIRST; Either is a
// two-case type with NO success/failure semantics (unlike Result) --
// Left = the first argument won, Right = the second. Either URL can
// legitimately win; which one does varies per run.
pub fn demo_race(url1: &str, url2: &str) {
    println!("\n*** demo of futures: two URLs, one winner, via select ***");

    trpl::block_on(async {
        let title_fut_1 = page_title(url1);
        let title_fut_2 = page_title(url2);

        let (url, maybe_title) = match trpl::select(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        }
    })
}
