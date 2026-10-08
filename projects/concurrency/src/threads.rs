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
