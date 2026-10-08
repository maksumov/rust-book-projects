// ch 16.1: threads -- running code simultaneously (listings 16-1..16-5).
// Output caveat: the interleaving varies per run -- the OS scheduler
// decides; sample outputs in comments are examples, not canonical.

use std::thread;
use std::time::Duration;

pub fn demo_spawn() {
    println!("\n*** demo of threads: spawn and the main thread's race ***");

    // Listing 16-1: spawn takes a closure for the new thread. Watch the
    // output: when MAIN finishes, spawned threads are killed whether or
    // not they finished -- our 1..10 loop gets cut around 5. The sleep
    // calls force turn-taking; without them one thread may hog the run.
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn demo_join_after() {
    println!("\n*** demo of threads: join AFTER main's loop (interleaved, then waiting) ***");

    // Listing 16-2: join() AFTER main's loop: prints interleave, main
    // waits at the end -- the spawned loop runs to completion (16-1
    // cut it).
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

pub fn demo_join_before() {
    println!("\n*** demo of threads: join BEFORE main's loop (fully sequential) ***");

    // The book's unnumbered experiment: the SAME handle.join(), hoisted
    // ABOVE main's loop: no interleaving at all -- main blocks first,
    // the spawned thread runs alone to 9, only then main prints 1..4.
    // Where join is called decides whether the threads overlap at all.
    // Contrast demo_join_after.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
