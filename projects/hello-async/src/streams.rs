// ch 17.4: streams -- futures in sequence (listings 17-21..17-22).
// A stream is an async iterator: items arriving over time. Not yet in
// the standard library -- the ecosystem's common definition; the full
// trait anatomy arrives in 17.5.

use trpl::StreamExt;

pub fn demo_stream_from_iter() {
    println!("\n*** demo of streams: an iterator becomes a stream ***");

    // Listing 17-21 (without the StreamExt import) does not compile:
    //
    // error[E0599]: no method named `next` found for struct
    // `tokio_stream::iter::Iter` in the current scope
    //   = help: items from traits can only be used if the trait
    // is in scope
    // help: the following traits which provide `next` are
    // implemented but not in scope; perhaps you want to import one
    // of them:
    //   + use crate::trpl::StreamExt;
    //
    // The surprise: not Stream but StreamExt -- the Ext ("extension")
    // pattern: a minimal base trait (Stream, poll_next) plus a
    // SEPARATE trait of conveniences (next, the iterator-like
    // utilities), blanket-implemented for every Stream. The import
    // at the top of this file IS listing 17-22.

    // Listing 17-22: any iterator becomes a stream; while let --
    // the language cannot yet `for` over async sequences (the same
    // shape as the channel's recv loop in tasks.rs).
    trpl::block_on(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });
}
