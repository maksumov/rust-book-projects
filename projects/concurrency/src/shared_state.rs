// ch 16.3 (+ 16.4): shared-state concurrency (listings 16-12..16-15).
// The counterpart of message passing: multiple threads access the same
// data -- multiple ownership, guarded by a mutex. The two rules
// (acquire before use, release when done) are made unforgettable by
// the type system: the data lives behind Mutex<T>, and the guard's
// Drop releases the lock. Send/Sync (16.4) close the story below.

use std::sync::Mutex;

pub fn demo_mutex_api() {
    println!("\n*** demo of shared state: mutex api in a single-threaded context ***");

    // Listing 16-12: the type is Mutex<i32>, not i32 -- using the value
    // REQUIRES lock(), which blocks until the lock is ours. The guard
    // combines both smart-pointer themes: Deref to the inner data (15.2)
    // + Drop releasing the lock at scope end (15.3). lock() returns
    // Result because a panicking holder POISONS the mutex (the Debug
    // output shows: poisoned: false) -- unwrap propagates that.
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
}
