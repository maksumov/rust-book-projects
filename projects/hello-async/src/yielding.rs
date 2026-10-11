// ch 17.3: yielding control to the runtime (listings 17-14..17-20).
// Between await points, async code is plain synchronous code -- it can
// STARVE every other future on the same thread. The fix: yield points
// (yield_now); the capstone: a hand-built timeout.

use std::{thread, time::Duration};

// Listing 17-14: deliberately BLOCKING -- thread::sleep, not
// trpl::sleep: a stand-in for real CPU-bound work that never yields.
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

pub fn demo_starvation() {
    println!("\n*** demo of yielding: starvation -- no await points, no switching ***");

    trpl::block_on(async {
        // Listing 17-15: two futures, each with blocking slow() work and
        // only ONE await point at the end. Everything between await points
        // is synchronous: a's slows all run, a yields; b's slows all run --
        // b owns the thread for their whole duration, and even a's
        // ALREADY-ELAPSED 50ms timer cannot fire while the thread is
        // blocked. No interleaving between the slow calls; a finishes
        // after b's blocking stretch (visible in the output order).

        let a = async {
            println!("'a' started.");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' finished.");
        };

        trpl::select(a, b).await;
    });
}
