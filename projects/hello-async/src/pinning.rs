// ch 17.5: a closer look at the traits for async (listings 17-23..17-24).
// This section is mostly anatomy: one runnable arc (join_all + pin!),
// the rest is a reference card of trait definitions and the Pin/Unpin
// story -- the "how it works under the hood" half of the chapter.

use std::future::Future;
use std::pin::{Pin, pin};
use std::time::Duration;

// The Future trait itself, verbatim:
//
// // pub trait Future {
// //     type Output;
// //     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
// // }
//
// poll is the heartbeat: the runtime asks "ready yet?" and gets
// Poll::Ready(value) or Poll::Pending. await compiles to a poll loop
// with a handoff: on Pending, the runtime parks THIS future and runs
// others -- that loop is why async code needs a runtime at all (the
// E0752 story from futures.rs). Polling a Ready future again is
// usually a panic, like Iterator::next after None. And note the self
// type -- Pin<&mut Self> -- which is exactly what the rest of this
// module is about.

pub fn demo_join_all() {
    println!("\n*** demo of pinning: join_all over a Vec of futures ***");

    // Scaled down from the book's 1s (as in tasks.rs: no lesson in
    // the value, only wall-clock time).
    const SLEEP_MS: u64 = 200;

    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        // Listing 17-23: a number of futures unknown until runtime --
        // box them into a Vec of trait objects... and join_all rejects:
        //
        // // let futures: Vec<Box<dyn Future<Output = ()>>> = vec![
        // //     Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut),
        // // ];
        // // trpl::join_all(futures).await;
        //
        // error[E0277]: `dyn Future<Output = ()>` cannot be unpinned
        //  48 |         trpl::join_all(futures).await;
        //     |                                 ^^^^^ the trait `Unpin` is
        //     |                                     not implemented for
        //     |                                     `dyn Future<Output = ()>`
        //  = note: consider using the `pin!` macro
        //
        // Why: Box<T> implements Future only if T: Future + Unpin, and
        // compiler-generated futures may be SELF-REFERENTIAL -- they
        // are !Unpin by default (the Pin card below the demo).

        let tx1 = tx.clone();
        // Listing 17-24: pin! each future where it is defined -- pinned
        // ON THE STACK, no Box needed -- and the Vec holds
        // Pin<&mut dyn Future> instead. join_all is satisfied, and
        // futures can now be added/removed at runtime.
        let tx1_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];
            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(SLEEP_MS)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });

        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(SLEEP_MS)).await;
            }
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx_fut];

        trpl::join_all(futures).await;
    });
}

// The Pin and Unpin story (the E0277 above, explained):
//
// Each await point becomes a state in a compiler-generated enum, and
// a variant may hold references INTO the same future -- a
// self-referential value. Moving such a value would leave those
// references pointing at the old, now-invalid location.
//
// Pin<P> is a wrapper for a pointer-like P guaranteeing that the
// POINTED-TO value never moves (the pointer itself may). Unpin is a
// marker trait -- no methods, exactly like Send/Sync from ch 16 --
// meaning "safe to move even under Pin". Almost every type is Unpin
// (String, Vec, numbers); !Unpin is the special case, and it only
// matters when a Pin is involved. This is why plain await "just
// works" (the future is pinned implicitly) while collecting futures
// into a Vec demands an explicit pin!.

// The Stream trait -- an iterator crossed with a future:
//
// // trait Stream {
// //     type Item;
// //     fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>)
// //         -> Poll<Option<Self::Item>>;
// // }
//
// Future's single Output x Iterator's sequence of Items; not in std
// yet -- the ecosystem's shared definition. StreamExt (the E0599
// import from streams.rs) supplies next(); its real signature returns
// Next<'_, Self> -- a FUTURE type, a workaround from before
// async-fn-in-traits existed, kept for compatibility. Implement
// Stream alone and StreamExt arrives blanket-implemented on top.
