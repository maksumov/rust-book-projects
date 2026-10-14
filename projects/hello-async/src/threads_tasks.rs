// ch 17.6: putting it all together - futures, tasks, and threads
// (listing 17-25). The rules of thumb: CPU-bound -> threads;
// I/O-bound -> async; both together whenever it helps. Under the
// hood the runtime is multithreaded by default (work stealing moves
// tasks between threads) - tasks are lightweight, runtime-managed
// threads.

use std::{thread, time::Duration};

// Beyond the book: scaled down from 1s (as elsewhere in this project
// - the pacing carries the lesson, not the absolute value).
const SLEEP_MS: u64 = 200;

pub fn demo_mixed_channel() {
    println!("\n*** demo of threads+tasks: a thread feeding an async channel ***");

    // Listing 17-25: the mix. An OS thread (ch 16 mechanics: a move
    // closure, thread::sleep blocking ITS OWN thread - fine there)
    // sends into an ASYNC channel; the async side receives in
    // block_on. The exit chain is the tasks.rs async move arc, now
    // crossing a thread boundary: the thread ends -> tx drops -> the
    // channel closes -> recv returns None -> clean exit.

    let (tx, mut rx) = trpl::channel();

    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(SLEEP_MS));
        }
    });

    trpl::block_on(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });
}
