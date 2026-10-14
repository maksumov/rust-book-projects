// ch 17.3: yielding control to the runtime (listings 17-14..17-20).
// Between await points, async code is plain synchronous code - it can
// STARVE every other future on the same thread. The fix: yield points;
// the capstone: a hand-built timeout. The module's sleep triangle:
// thread::sleep BLOCKS; trpl::sleep WAITS, then yields; yield_now
// yields NOW.

use std::{thread, time::Duration};
use trpl::Either;

// Listing 17-14: deliberately BLOCKING - thread::sleep, not
// trpl::sleep: a stand-in for real CPU-bound work that never yields.
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

pub fn demo_starvation() {
    println!("\n*** demo of yielding: starvation - no await points, no switching ***");

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

pub fn demo_yield_sleep() {
    println!("\n*** demo of yielding: sleep as a makeshift yield point ***");

    trpl::block_on(async {
        // Listing 17-16: trpl::sleep(1ms).await between the slow calls.
        // Note WHICH sleep: the async one - an await point; a plain
        // thread::sleep here would change nothing from 17-15. Await
        // points appear, and with them the interleaving. But sleep is
        // a FAKE yield: we do not want to WAIT, we want to hand control
        // back - and timers have granularity limits (this sleep takes
        // at least 1ms even for a 1ns argument).

        let one_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::sleep(one_ms).await;
            slow("a", 10);
            trpl::sleep(one_ms).await;
            slow("a", 20);
            trpl::sleep(one_ms).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::sleep(one_ms).await;
            slow("b", 10);
            trpl::sleep(one_ms).await;
            slow("b", 15);
            trpl::sleep(one_ms).await;
            slow("b", 350);
            trpl::sleep(one_ms).await;
            println!("'b' finished.");
        };

        trpl::select(a, b).await;
    });
}

pub fn demo_yield_now() {
    println!("\n*** demo of yielding: yield_now - honest cooperative switching ***");

    trpl::block_on(async {
        // Listing 17-17: yield_now().await - the honest version: hand
        // control to the runtime NOW, without waiting for anything.
        // This is COOPERATIVE multitasking: each future decides when to
        // yield - and therefore carries the responsibility of yielding
        // often enough (17-15 is what happens when it does not). Yielding
        // is cheap but not free: break up work where it makes sense, and
        // measure before optimizing. The output shape matches 17-16 - the
        // difference is mechanics and speed, not the interleaving pattern.

        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };

        trpl::select(a, b).await;
    });
}

pub fn demo_timeout() {
    println!("\n*** demo of yielding: a hand-built timeout (select + sleep) ***");

    // Listing 17-18: API-first - write the intended usage BEFORE the
    // implementation exists. The slow future sleeps 5s; the budget is
    // 2 - the timeout wins ("Failed after 2 seconds").
    trpl::block_on(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}

// Listings 17-19..17-20: first the SIGNATURE, derived from the usage's
// requirements: generic over the future (F: Future), a Duration
// (convenient to hand straight to sleep), Err carrying the spent
// budget. Then the implementation: race the future against a sleep
// timer via select. NOTE the argument order: select is NOT fair --
// it polls the arguments in the order given, so the future goes
// FIRST, getting a chance to complete even under a tiny budget.
// Futures compose: select + sleep grows into a new tool; timeouts +
// retries grow further (the book's closing suggestion).
async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}
