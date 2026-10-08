# Chapter 16: Shared-State Concurrency — Exercises

1. From the book: create a Rust program that deadlocks. Two threads,
   two `Mutex`es, each thread acquiring one lock and then reaching for
   the other (the book's closing dare; the shared_state.rs footer
   points here). Observe: the program hangs -- interrupt it.
2. Diagnose: find where the threads are stuck -- strategic println!s
   before each lock attempt, or a debugger.
3. beyond the book: fix it two ways -- (a) consistent lock ORDER (all
   code acquires A before B); (b) redesign with `std::sync::mpsc`
   (one owner, no locks -- ch 16.2's answer to ch 16.3's problem).
4. beyond the book: poisoning -- make a thread panic while holding a
   lock; observe the next `lock()` returning `Err` (the `poisoned`
   flag from demo_mutex_api's Debug output, now in action).

Source: https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html

From the book:

> If you're interested in deadlocks, try creating a Rust program that
> has a deadlock; then, research deadlock mitigation strategies for
> mutexes in any language and have a go at implementing them in Rust.
