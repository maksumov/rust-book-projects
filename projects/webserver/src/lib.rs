// ch 21.2: the ThreadPool library (listings 21-12..21-21).
// Interface-first + compiler-driven: main.rs demanded the API
// (21-12) and the errors built the crate - E0433 (no type
// `ThreadPool`) demanded this lib; E0599 demanded `new`; the next
// E0599 demanded `execute`. The bounds mirror thread::spawn's
// signature: FnOnce - a job runs exactly once; Send - the closure
// crosses into another thread; 'static - we cannot know how long
// it will run. The journey ended in a WORKING pool (21-20): one
// receiver, serialized by the Mutex, feeding all the workers.

use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

// Listing 21-16 (part 1): the pool KEEPS the sending end of the
// channel - the entrance of the job queue.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

// Listing 21-19: Job becomes a TYPE ALIAS (ch 20.3 paying off) -
// Box<dyn FnOnce() + Send + 'static>: a boxed closure the channel
// can carry; execute's bounds, now a NAME. execute sends Box::new(f)
// down the sender - unwrap: sending fails only when all receivers
// are gone, and the workers cannot stop while the pool exists.
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    // Listing 21-13: validating size - zero is a perfectly valid
    // usize but a meaningless pool size; assert! makes it a panic (a
    // programming error, unrecoverable by ch 9's rule - the book
    // notes the recoverable alternative, `build` returning
    // Result<ThreadPool, PoolCreationError>), and the # Panics doc
    // section declares the contract.

    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // Listing 21-16 (part 2): the channel is born in new - mpsc,
        // multiple producers (every execute call sends), ONE consumer:
        // the constraint that will demand Arc<Mutex<>> in 21-18. The
        // sender moves into the struct; the receiver is unused SO FAR -
        // the compiler's next instruction (21-17: share it with the
        // workers).
        let (sender, receiver) = mpsc::channel();

        // Listing 21-18: shared ownership + serialized receiving.
        // Arc - every worker owns the receiver (clone bumps the
        // count, ch 16); Mutex - the legal bridge from many & to
        // &mut (recv is a MUTATION: it takes &mut self, removing a
        // job from the queue), one worker receiving at a time.
        // &Receiver is not Sync; &Mutex<Receiver> is - the
        // type-level spell for "shared + mutable + across threads".
        let receiver = Arc::new(Mutex::new(receiver));

        // Listing 21-14: space for the threads. JoinHandle<()> is what
        // thread::spawn RETURNS (T = () - our jobs return nothing);
        // with_capacity pre-allocates - the count is known up front, no
        // growth reallocations. The loop body began as the book's
        // empty placeholder - what a thread should RUN before jobs
        // exist was the next problem (the Worker, 21-15).
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

// Listing 21-15: the Worker - a thread WITH an identity, the
// pooling term for a thread that waits for jobs (kitchen staff:
// they stand by until an order arrives). The pool stores Workers,
// not raw JoinHandles; Worker stays PRIVATE - an implementation
// detail the server (main.rs) never sees. The spawn closure began
// empty - a thread that ran nothing, waiting for the channel
// (The book's note: thread::spawn PANICS if the OS refuses a
// thread - production pools use thread::Builder::spawn, the
// Result-returning variant.)
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

// Listing 21-17: the receiver goes to the workers - and refuses:
//
// // error[E0382]: use of moved value: `receiver`
// //   move occurs because `receiver` has type
// //   `std::sync::mpsc::Receiver<Job>`, which does not implement
// //   the `Copy` trait ... value moved here, in previous
// //   iteration of loop
//
// mpsc is single-CONSUMER: the receiving end has no clone, yet
// every worker needs it AND must mutate it (recv) - the
// instruction for 21-18's Arc<Mutex<>>.
//
// Listing 21-20: the worker loop - the pool WORKS. Three unwraps,
// three contracts: lock().unwrap() - a poisoned mutex (someone
// panicked holding it) re-panics here, the correct action; recv()
// .unwrap() - the sender is gone (cannot happen while the pool
// lives); recv() BLOCKS - the worker stands by until a job arrives.
// The `let` is load-bearing: the MutexGuard temporary dies at the
// statement's end, RELEASING the lock before job() runs - one
// worker receives while ALL of them execute. (21-21 will show the
// while let version that breaks exactly here.)
//
// Listing 21-21: the tempting alternative - and why it is wrong:
//
// // while let Ok(job) = receiver.lock().unwrap().recv() {
// //     println!("Worker {id} got a job; executing.");
// //     job();
// // }
//
// It compiles and runs - but serializes everything: while let (and
// if let, and match) hold their temporaries until the END of the
// block, so the MutexGuard - the LOCK - lives through job(): one
// worker works while the rest wait on the mutex. The plain `let`
// above drops the guard at the statement's end; that half-line of
// syntax is the difference between a pool and a queue.
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {id} got a job; executing.");

                job();
            }
        });

        Worker { id, thread }
    }
}
