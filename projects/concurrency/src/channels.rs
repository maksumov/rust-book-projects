// ch 16.2: message passing -- channels between threads (listings
// 16-6..16-11). "Do not communicate by sharing memory; instead, share
// memory by communicating" (the Go slogan the book quotes). A channel
// transfers OWNERSHIP: once sent, the sender must not use the value --
// single-ownership concurrency, the counterpart of shared-state (16.3).
// mpsc = multiple producer, single consumer.

use std::sync::mpsc;
use std::thread;

pub fn demo_channel() {
    println!("\n*** demo of channels: a spawned thread sends, main receives ***");

    // Listing 16-6 (the bare channel) does not compile: with nothing
    // sent or received, the value type is not inferable:
    //
    // // let (tx, rx) = mpsc::channel();
    //
    // error[E0282]: type annotations needed

    // Listings 16-7..16-8: tx MOVES into the spawned thread; send takes
    // ownership and returns Result (error if the receiver is dropped);
    // recv BLOCKS until a value arrives (error once all senders are
    // dropped -- the channel is closed); try_recv is the non-blocking
    // sibling for "check while doing other work".
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();

        // Listing 16-9: using val AFTER sending it is rejected -- the value
        // now belongs to the receiving thread. Ownership makes message
        // passing safe:
        //
        // // tx.send(val).unwrap();
        // // println!("val is {val}");
        //
        // error[E0382]: borrow of moved value: `val`
        //  9 |         tx.send(val).unwrap();
        //    |                 --- value moved here
        // 10 |         println!("val is {val}");
        //    |                           ^^^ value borrowed here after move
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
