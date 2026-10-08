// ch 16.3 (+ 16.4): shared-state concurrency (listings 16-12..16-15).
// The counterpart of message passing: multiple threads access the same
// data -- multiple ownership, guarded by a mutex. The two rules
// (acquire before use, release when done) are made unforgettable by
// the type system: the data lives behind Mutex<T>, and the guard's
// Drop releases the lock. Send/Sync (16.4) close the story below.

use std::sync::{Arc, Mutex};
use std::thread;

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

pub fn demo_shared_counter() {
    println!("\n*** demo of shared state: shared access to mutex ***");

    // Listing 16-13: ten threads, one Mutex -- rejected. Ownership of
    // counter cannot move into ten closures:
    //
    // error[E0382]: borrow of moved value: `counter`
    //  8 |     for _ in 0..10 {
    //    |     -------------- inside of this loop
    //  9 |         let handle = thread::spawn(move || {
    //    |                                    ------- value moved into closure here, in previous iteration of loop
    // 21 |     println!("Result: {}", *counter.lock().unwrap());
    //    |                             ^^^^^^^ value borrowed here after move
    // // let counter = Mutex::new(0);
    // // let mut handles = vec![];
    // //
    // // for _ in 0..10 {
    // //     let handle = thread::spawn(move || {
    // //         let mut num = counter.lock().unwrap();
    // //
    // //         *num += 1;
    // //     });
    // //     handles.push(handle);
    // // }

    // Listing 16-14: the ch 15 answer -- multiple owners via Rc -- now
    // fails differently: the counter would cross threads, but Rc's
    // reference count is not thread-safe:
    //
    // error[E0277]: `Rc<Mutex<i32>>` cannot be sent between threads safely
    //   = help: the trait `Send` is not implemented for `Rc<Mutex<i32>>`
    //
    // The bridge to 16.4: Send/Sync are the marker traits deciding what
    // may cross or be shared between threads.
    // // let counter = Rc::new(Mutex::new(0));
    // // let mut handles = vec![];
    // //
    // // for _ in 0..10 {
    // //     let counter = Rc::clone(&counter);
    // //     let handle = thread::spawn(move || {
    // //         let mut num = counter.lock().unwrap();
    // //
    // //         *num += 1;
    // //     });
    // //     handles.push(handle);
    // // }

    // Listing 16-15: Arc<T> -- the SAME API as Rc, atomically
    // reference-counted (atomic = the counter bumps are thread-safe).
    // Arc::clone per thread, Mutex inside: multiple owners AND mutation
    // -- the RefCell/Rc pair's threaded sibling. Result: 10 -- ten
    // increments, none lost.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// ch 16.4, the story closed: Send and Sync -- marker traits (no
// methods) built into the language, not the standard library.
//   Send: ownership of the type may be TRANSFERRED between threads
//         (Rc is not Send -- its counter is not thread-safe; Arc is).
//   Sync: &T may be shared across threads (&T: Send). RefCell and the
//         Cell family are not Sync (runtime borrow checks are
//         single-threaded); Mutex is.
// Compositions inherit: a type of Send/Sync parts is Send/Sync itself;
// implementing them manually is unsafe (ch 20). The parallel pairs:
//   Rc : RefCell  ::  Arc : Mutex    (single-threaded vs threaded)
// And Mutex's own logic bug the compiler cannot catch: deadlocks (the
// sibling of Rc's reference cycles) -- see exercises/16_deadlock.
