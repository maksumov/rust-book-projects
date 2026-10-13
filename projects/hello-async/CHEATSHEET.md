# threads vs async decision cheatsheet (chapter 17)

The decision layer on top of this project's demos; examples and
outputs live in the modules.

## Threads or async?

|  | threads (std) | async (tasks) |
|---|---|---|
| who switches | the OS, preemptively, at any moment | the runtime, cooperatively, at await points |
| best for | CPU-bound work (parallelism) | I/O-bound work (concurrency at scale) |
| cost per unit | a stack (MBs) -- thousands | a state machine (bytes) -- millions |
| forgetting to yield | impossible (preemption) | starves everything (`yield_now` is the fix) |
| together? | yes -- the runtime is multithreaded underneath (work stealing moves tasks across threads) |

## The sleep triangle (yielding.rs)

- `thread::sleep` BLOCKS the thread.
- `trpl::sleep` WAITS, then yields.
- `yield_now` yields NOW.

## The combinator map

- `block_on` -- the sync->async bridge: runs ONE future to completion,
  blocking its thread. Every demo in this project carries its own.
- `join(f1, f2)` -- await BOTH, fairly polled: deterministic interleaving.
- `join!(f1, f2, ..)` -- the macro form: any number of futures known at
  compile time.
- `join_all(vec)` -- a runtime-sized collection; the futures must be
  pinned (`pin!`, stack or `Box::pin` for escapes).
- `select(f1, f2)` -- the FIRST to finish wins; NOT fair (it polls in
  argument order: put the real future before the timer).
- `yield_now` -- hand control back without waiting for anything.

## Ownership notes that save the day

- `send` MOVES the value: the sender must not use it afterwards.
- Dropping the LAST transmitter closes the channel: `recv` yields
  `None`, the `while let` loop ends, the program exits. A merely
  borrowed `tx` inside an async block = a hang (the tasks.rs arc).
- `async move` for every producer block; `move` closures into threads
  likewise (ch 16).
- Futures are LAZY: creating is not running. JS promises start on
  creation; Rust futures do not -- they wait for an await.

## Going deeper

The book's own pointers from 17.5: *Asynchronous Programming in Rust*
(https://rust-lang.github.io/async-book/), chapters 2 (execution) and
4 (pinning). Best read after ch 18-20 of the book itself.
