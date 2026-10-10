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

pub fn demo_join() {
    println!("\n*** demo of tasks: joining two futures -- no spawn at all ***");

    // Listing 17-8: no spawn_task at all -- async blocks ARE futures.
    // Two of them, then trpl::join(fut1, fut2).await: we await the
    // JOIN, not the futures -- awaiting them one by one would be
    // sequential, the opposite of the goal. join is FAIR: it polls
    // each future equally, alternating, never letting one race ahead
    // -- the output is deterministic, the exact same order every run,
    // unlike threads and spawn_task (the 17-6 contrast, resolved).
    trpl::block_on(async {
        // The book's experiments (predict the output BEFORE running):
        //  - remove the async block from either loop
        //  - await each block immediately after defining it
        //  - wrap only the first loop, await it after the second's body

        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(TIME_TO_SLEEP)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(TIME_TO_SLEEP)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    })
}

pub fn demo_channel() {
    println!("\n*** demo of tasks: an async channel -- one message ***");

    trpl::block_on(async {
        // Listing 17-9: the async channel vs ch 16's mpsc -- rx is MUT
        // here, and recv() returns a future: awaiting it yields to the
        // runtime until a message or the close, instead of blocking the
        // thread. send needs no await: the channel is unbounded.
        let (tx, mut rx) = trpl::channel();

        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("received '{received}'");
    });
}

pub fn demo_channel_multiple() {
    println!("\n*** demo of tasks: async channel -- multiple messages, clean exit ***");

    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        // Listing 17-10: both loops in ONE async block -- two problems:
        // the messages arrive all at once (code within a single async
        // block is LINEAR: await order IS execution order, so every
        // send+sleep completes before the receive loop even starts),
        // and the program never exits (the while let waits for None
        // forever).
        //
        // // let vals = vec![
        // //     String::from("hi"),
        // //     String::from("from"),
        // //     String::from("the"),
        // //     String::from("future"),
        // // ];
        // //
        // // for val in vals {
        // //     tx.send(val).unwrap();
        // //     trpl::sleep(Duration::from_millis(TIME_TO_SLEEP)).await;
        // // }
        // //
        // // while let Some(value) = rx.recv().await {
        // //     println!("received '{value}'");
        // // }

        // Listing 17-11: split into two futures joined with join -- the
        // pauses appear (concurrent at last), but still no exit. The
        // book's chain: join waits for BOTH futures; rx_fut ends only
        // when the while let gets None; None comes only when the channel
        // closes; it closes when tx is dropped; tx merely BORROWS into
        // tx_fut, so it lives until the outer block ends -- and the
        // outer block is blocked on join. An async-flavored deadlock.
        //
        // // let tx_fut = async {
        // //     let vals = vec![
        // //         String::from("hi"),
        // //         String::from("from"),
        // //         String::from("the"),
        // //         String::from("future"),
        // //     ];
        // //
        // //     for val in vals {
        // //         tx.send(val).unwrap();
        // //         trpl::sleep(Duration::from_millis(TIME_TO_SLEEP)).await;
        // //     }
        // // };
        // //
        // // let rx_fut = async {
        // //     while let Some(value) = rx.recv().await {
        // //         println!("received '{value}'");
        // //     }
        // // };
        // //
        // // trpl::join(tx_fut, rx_fut).await;

        // Listing 17-12: async move -- one word breaks the chain: tx's
        // ownership moves into tx_fut, so it drops when tx_fut finishes
        // -> the channel closes -> recv returns None -> the while let
        // ends -> rx_fut completes -> join completes -> clean exit.
        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(TIME_TO_SLEEP)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    });
}
