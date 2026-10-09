// ch 17.2: applying concurrency with async (listings 17-6..17-13).
// The ch 16 patterns rebuilt on tasks -- runtime-managed, not OS
// threads: spawn_task, join handles, channels, move blocks. Same
// shapes, different machinery underneath.

use std::time::Duration;

// Beyond the book: the book sleeps 500ms; 200 keeps the whole run
// quick -- the value carries no lesson, only wall-clock time.
const TIME_TO_SLEEP: u64 = 200;

pub fn demo_spawn_task() {
    println!("\n*** demo of tasks: spawn, and the main task's race ***");

    // Listing 17-6: the 16-1 story rebuilt -- spawn_task + trpl::sleep
    // (...).await. sleep is ASYNC here: an await point handing control
    // back to the runtime, not a blocking call. When the enclosing
    // async block ends, the spawned task is shut down unfinished --
    // the 1..10 loop gets cut around 5, like the thread version.
    //
    // Contrast with ch 16: there the OS preemptively switched THREADS;
    // here the runtime switches TASKS cooperatively -- only at await
    // points. await never blocks a thread: the task yields, the runtime
    // runs whichever task is ready. The interleaving looks like the
    // thread version's; the machinery differs (and no second OS thread
    // exists). Deterministic output arrives only with join's fairness
    // (17-8).
    trpl::block_on(async {
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(TIME_TO_SLEEP)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(TIME_TO_SLEEP)).await;
        }
    });
}

pub fn demo_join_handle() {
    println!("\n*** demo of tasks: awaiting the join handle to completion ***");

    // Listing 17-7: same fix as ch 16's join -- but the handle itself
    // IS a future: handle.await.unwrap() (its Output is a Result), no
    // blocking join method. The first task now runs all the way to 9.
    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(TIME_TO_SLEEP)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(TIME_TO_SLEEP)).await;
        }

        handle.await.unwrap();
    });
}
