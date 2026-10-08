// ch 16.2: message passing -- channels between threads (listings
// 16-6..16-11). "Do not communicate by sharing memory; instead, share
// memory by communicating" (the Go slogan the book quotes). A channel
// transfers OWNERSHIP: once sent, the sender must not use the value --
// single-ownership concurrency, the counterpart of shared-state (16.3).
// mpsc = multiple producer, single consumer.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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

pub fn demo_channel_stream() {
    println!("\n*** demo of channels: sending multiple messages and pausing between each one ***");

    // Listing 16-10: the producer sends four strings, pausing a second
    // between sends. Main has no sleep of its own -- the pauses in the
    // OUTPUT prove main is waiting on the channel. No explicit recv
    // anymore: rx IS an iterator, and the loop ends when the channel
    // closes (all senders dropped -- here, when the spawned thread ends).
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

pub fn demo_multiple_producers() {
    println!("\n*** demo of channels: mpsc - multiple producers, one receiver ***");

    // Listing 16-11: clone the transmitter BEFORE the first thread:
    // tx1 goes to thread one, the ORIGINAL tx moves into thread two.
    // One receiver consumes both streams, interleaved in an order the
    // scheduler picks per run -- the nondeterminism caveat of the
    // module header, now by design.
    let (tx, rx) = mpsc::channel();

    // A true clone of the SENDING CAPABILITY (not a refcount bump like
    // Rc::clone, ch 15): each thread gets an independent transmitter,
    // and the channel closes only when ALL of them are dropped -- which
    // is exactly what ends the for loop below.
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
