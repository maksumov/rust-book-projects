# Glossary

Terms and concepts from The Rust Book, collected while studying.
Each entry: a definition, connections to related entries, where the
concept lives in this repo (if anywhere), and the book section it
came from. Entries are alphabetical; the chapter tag shows where
the term first appeared in the study flow. Related links are
bidirectional: if X lists Y, then Y lists X.

## Table of Contents
- [Arc, atomic reference counting (ch 16.3)](#arc-atomic-reference-counting-ch-163)
- [Associated functions (ch 5.3)](#associated-functions-ch-53)
- [Associated type (ch 20.2)](#associated-type-ch-202)
- [Async runtime (ch 17.1)](#async-runtime-ch-171)
- [At bindings (ch 19.3)](#at-bindings-ch-193)
- [Backtrace (ch 9.1)](#backtrace-ch-91)
- [Binary target (ch 14.4)](#binary-target-ch-144)
- [Blanket implementations (ch 10.2)](#blanket-implementations-ch-102)
- [Borrow checker (ch 10.3)](#borrow-checker-ch-103)
- [Borrow-based lookup (ch 8.3)](#borrow-based-lookup-ch-83)
- [Borrowing (ch 4.2)](#borrowing-ch-42)
- [Buffer overread (ch 9.1)](#buffer-overread-ch-91)
- [Channel, mpsc (ch 16.2)](#channel-mpsc-ch-162)
- [Closure (ch 13.1)](#closure-ch-131)
- [Coherence (ch 10.2)](#coherence-ch-102)
- [Combinators (ch 9.2)](#combinators-ch-92)
- [Concurrency vs parallelism (ch 17.0)](#concurrency-vs-parallelism-ch-170)
- [Constants (ch 3.1)](#constants-ch-31)
- [Cooperative multitasking (ch 17.3)](#cooperative-multitasking-ch-173)
- [CPU-bound vs I/O-bound (ch 17.0)](#cpu-bound-vs-io-bound-ch-170)
- [Crate (ch 7.1)](#crate-ch-71)
- [Crate root (ch 7.1)](#crate-root-ch-71)
- [Custom Cargo subcommands (ch 14.5)](#custom-cargo-subcommands-ch-145)
- [Data race (ch 4.2)](#data-race-ch-42)
- [Deadlock (ch 16.3)](#deadlock-ch-163)
- [Declarative macro (ch 20.5)](#declarative-macro-ch-205)
- [Deref coercion (ch 4.2)](#deref-coercion-ch-42)
- [Destructor (ch 15.3)](#destructor-ch-153)
- [Destructuring (ch 19.3)](#destructuring-ch-193)
- [Documentation comments (ch 14.2)](#documentation-comments-ch-142)
- [DST, dynamically sized type (ch 20.3)](#dst-dynamically-sized-type-ch-203)
- [Duck typing (ch 18.2)](#duck-typing-ch-182)
- [Dyn compatibility (ch 18.2)](#dyn-compatibility-ch-182)
- [Encapsulation (ch 18.1)](#encapsulation-ch-181)
- [Fearless concurrency (ch 16.0)](#fearless-concurrency-ch-160)
- [FFI, foreign function interface (ch 20.1)](#ffi-foreign-function-interface-ch-201)
- [Fn traits (ch 13.1)](#fn-traits-ch-131)
- [Fully qualified syntax (ch 20.2)](#fully-qualified-syntax-ch-202)
- [Function pointer (ch 20.4)](#function-pointer-ch-204)
- [Future (ch 17.1)](#future-ch-171)
- [Ignoring in patterns (ch 19.3)](#ignoring-in-patterns-ch-193)
- [Indirection (ch 15.1)](#indirection-ch-151)
- [Initializer function (ch 20.4)](#initializer-function-ch-204)
- [Inner vs outer attributes (ch 9.2)](#inner-vs-outer-attributes-ch-92)
- [Integration tests (ch 11.3)](#integration-tests-ch-113)
- [Interior mutability (ch 15.5)](#interior-mutability-ch-155)
- [Iterator (ch 13.2)](#iterator-ch-132)
- [Iterator adapters vs consuming adapters (ch 13.2)](#iterator-adapters-vs-consuming-adapters-ch-132)
- [Lifetime (ch 10.3)](#lifetime-ch-103)
- [Lifetime elision rules (ch 10.3)](#lifetime-elision-rules-ch-103)
- [Lint levels (ch 14.2)](#lint-levels-ch-142)
- [Macro (ch 20.5)](#macro-ch-205)
- [Match guard (ch 19.3)](#match-guard-ch-193)
- [Memory leak (ch 15.6)](#memory-leak-ch-156)
- [Message passing vs shared state (ch 16.0)](#message-passing-vs-shared-state-ch-160)
- [Miri (ch 20.1)](#miri-ch-201)
- [Monomorphization (ch 10.1)](#monomorphization-ch-101)
- [Move (ch 4.1)](#move-ch-41)
- [move closures (ch 13.1)](#move-closures-ch-131)
- [Mutex and lock (ch 16.3)](#mutex-and-lock-ch-163)
- [Never type (ch 20.3)](#never-type-ch-203)
- [Newtype pattern (ch 10.2)](#newtype-pattern-ch-102)
- [NLL, non-lexical lifetimes (ch 4.2)](#nll-non-lexical-lifetimes-ch-42)
- [Opaque type, impl Trait (ch 10.2)](#opaque-type-impl-trait-ch-102)
- [Operator overloading (ch 20.2)](#operator-overloading-ch-202)
- [Orphan rule (ch 10.2)](#orphan-rule-ch-102)
- [Ownership (ch 4.1)](#ownership-ch-41)
- [Package (ch 7.1)](#package-ch-71)
- [panic vs Result guidelines (ch 9.3)](#panic-vs-result-guidelines-ch-93)
- [Pin and Unpin (ch 17.5)](#pin-and-unpin-ch-175)
- [Procedural macro (ch 20.5)](#procedural-macro-ch-205)
- [RAII (ch 4.1)](#raii-ch-41)
- [Raw pointer (ch 20.1)](#raw-pointer-ch-201)
- [Rc and reference counting (ch 15.4)](#rc-and-reference-counting-ch-154)
- [Re-export (ch 14.2)](#re-export-ch-142)
- [Refutable vs irrefutable (ch 19.2)](#refutable-vs-irrefutable-ch-192)
- [Release profiles (ch 14.1)](#release-profiles-ch-141)
- [Resolver versions (ch 14.3)](#resolver-versions-ch-143)
- [Rustonomicon (ch 20.1)](#rustonomicon-ch-201)
- [SAFETY comment (ch 20.1)](#safety-comment-ch-201)
- [Send and Sync (ch 16.4)](#send-and-sync-ch-164)
- [Shadowing (ch 3.1)](#shadowing-ch-31)
- [SipHash and BuildHasher (ch 8.3)](#siphash-and-buildhasher-ch-83)
- [Slice (ch 4.3)](#slice-ch-43)
- [Smart pointers (ch 15.0)](#smart-pointers-ch-150)
- [Standard output vs standard error (ch 12.6)](#standard-output-vs-standard-error-ch-126)
- [State pattern (ch 18.3)](#state-pattern-ch-183)
- [Static lifetime (ch 10.3)](#static-lifetime-ch-103)
- [Static vs const (ch 20.1)](#static-vs-const-ch-201)
- [Static vs dynamic dispatch (ch 18.2)](#static-vs-dynamic-dispatch-ch-182)
- [Stream (ch 17.4)](#stream-ch-174)
- [Strong vs weak references (ch 15.6)](#strong-vs-weak-references-ch-156)
- [Supertrait (ch 20.2)](#supertrait-ch-202)
- [Test double and mock objects (ch 15.5)](#test-double-and-mock-objects-ch-155)
- [Trait (ch 10.2)](#trait-ch-102)
- [Trait bound (ch 10.1)](#trait-bound-ch-101)
- [Trait must be in scope (ch 9.2)](#trait-must-be-in-scope-ch-92)
- [Trait object (ch 18.2)](#trait-object-ch-182)
- [Type alias (ch 20.3)](#type-alias-ch-203)
- [Undefined behavior (ch 20.1)](#undefined-behavior-ch-201)
- [Union (ch 20.1)](#union-ch-201)
- [Unit tests (ch 11.3)](#unit-tests-ch-113)
- [unsafe Rust (ch 20.1)](#unsafe-rust-ch-201)
- [unsafe trait (ch 20.1)](#unsafe-trait-ch-201)
- [Unwinding vs abort (ch 9.1)](#unwinding-vs-abort-ch-91)
- [Workspace (ch 14.3)](#workspace-ch-143)
- [Yank (ch 14.2)](#yank-ch-142)
- [Zero-cost abstraction (ch 13.4)](#zero-cost-abstraction-ch-134)
---

## Arc, atomic reference counting (ch 16.3)

The thread-safe sibling of Rc<T>: the SAME API, but the reference
count is bumped with atomic operations, safe under concurrency.
Arc::clone per thread, Mutex inside -- the Arc<Mutex<T>> combo gives
multiple owners AND mutation across threads. Not the default
anywhere: thread safety is a performance penalty paid only when
needed.

Related: [Mutex and lock](#mutex-and-lock-ch-163), [Rc and reference counting](#rc-and-reference-counting-ch-154), [Send and Sync](#send-and-sync-ch-164)
In repo: `projects/concurrency/src/shared_state.rs` (demo_shared_counter)
Book: https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html#atomic-reference-counting-with-arct

## Associated functions (ch 5.3)

Functions defined in an `impl` block, associated with the type --
with or without `self`. Without `self` they need no instance
(`String::from`, `Rectangle::square`) and often serve as
constructors.

Related: —
In repo: `projects/rectangles/src/main.rs` (the square constructor)
Book: https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html#associated-functions

## Associated type (ch 20.2)

A type placeholder declared IN a trait (`type Item;`) that the impl
pins once (`type Item = u32`) - the impl decides, the caller
discovers: no annotations at call sites, unlike generics. With a
generic parameter a type could implement the trait MANY times
(Iterator<u8> AND Iterator<String>), forcing annotations everywhere;
an associated type allows exactly ONE impl. Iterator's Item and
Add's Output are the canonical pair.

Related: [Iterator](#iterator-ch-132), [Operator overloading](#operator-overloading-ch-202), [Trait](#trait-ch-102)
In repo: `projects/advanced/src/traits.rs` (Counter, Point)
Book: https://doc.rust-lang.org/stable/book/ch20-02-advanced-traits.html#defining-traits-with-associated-types

## Async runtime (ch 17.1)

The crate that drives futures: polling them, parking the Pending
ones, running the ready ones. Rust ships NO runtime of its own
(unlike most languages) -- tokio and friends are choices; the
book's trpl wraps tokio. block_on is the sync->async bridge; the
polling core is called the executor.

Related: [Cooperative multitasking](#cooperative-multitasking-ch-173), [Future](#future-ch-171)
In repo: `projects/hello-async` (every demo's block_on)
Book: https://doc.rust-lang.org/stable/book/ch17-01-futures-and-syntax.html#executing-an-async-function-with-a-runtime

## At bindings (ch 19.3)

The @ operator tests and binds at once: id @ 3..=7 checks the range
AND captures the value into id. The contrast arms: a bare range
tests but binds nothing (the arm's code cannot name the value); a
bare name binds but tests nothing.

Related: —
In repo: `projects/patterns/src/syntax.rs` (demo_at_bindings)
Book: https://doc.rust-lang.org/stable/book/ch19-03-pattern-syntax.html#using--bindings

## Backtrace (ch 9.1)

The list of all functions called to reach a point; shown on panic
with RUST_BACKTRACE=1. Read from the top until the first file YOU
wrote -- that is where the problem originated. Requires debug
symbols (default in debug builds).

Related: [panic vs Result guidelines](#panic-vs-result-guidelines-ch-93)
In repo: `projects/panic/src/backtrace.rs`
Book: https://doc.rust-lang.org/stable/book/ch09-01-unrecoverable-errors-with-panic.html#unwinding-the-stack-or-aborting-in-response-to-a-panic

## Binary target (ch 14.4)

The runnable program a crate produces when it has a src/main.rs
(or a configured [[bin]]) -- as opposed to a library target,
which cannot run on its own but can be included by other
programs. cargo install requires binary targets and drops the
compiled executables into $HOME/.cargo/bin; it is a convenience
for Rust tools, not a replacement for a system package manager.

Related: [Crate](#crate-ch-71), [Custom Cargo subcommands](#custom-cargo-subcommands-ch-145)
In repo: every project in this repo has one; projects/art has both target kinds
Book: https://doc.rust-lang.org/stable/book/ch14-04-installing-binaries.html

## Blanket implementations (ch 10.2)

Implementing a trait for EVERY type satisfying a bound:
`impl<T: Display> ToString for T`. Powers the everyday `.to_string()`
on any Display type. Listed in the trait docs "Implementors" section.

Related: [Trait bound](#trait-bound-ch-101)
In repo: `projects/aggregator/src/lib.rs` (the Pair block comment)
Book: https://doc.rust-lang.org/stable/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods

## Borrow checker (ch 10.3)

The compiler pass comparing the scopes of borrows: a reference may
not outlive the data it points to. Rejects violations with E0597
"does not live long enough"; made ergonomic by NLL.

Related: [Borrowing](#borrowing-ch-42), [Data race](#data-race-ch-42), [Lifetime](#lifetime-ch-103), [NLL, non-lexical lifetimes](#nll-non-lexical-lifetimes-ch-42), [unsafe Rust](#unsafe-rust-ch-201)
In repo: `projects/lifetimes/src/dangling.rs`
Book: https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html#the-borrow-checker

## Borrow-based lookup (ch 8.3)

`HashMap::get` is generic over `Q` where `K: Borrow<Q>`; since
`String: Borrow<str>`, a map with `String` keys can be looked up
by a plain `&str` -- no allocation, unlike building a `String` key
per lookup. Distinct from deref coercion: this is `Borrow`-based
type equality at the API level.

Related: [Deref coercion](#deref-coercion-ch-42)
In repo: `projects/collections/src/demos/hashmaps.rs` (comment inside the lookup loop)
Book: https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#accessing-values-in-a-hash-map

## Borrowing (ch 4.2)

Creating a reference to a value: `&T` borrows without taking
ownership -- the owner keeps it, and the borrower may not outlive
it. Mutable borrowing (`&mut T`) follows the one-mutable-or-many-
immutable rule.

Related: [Borrow checker](#borrow-checker-ch-103), [Data race](#data-race-ch-42), [Lifetime](#lifetime-ch-103), [Move](#move-ch-41), [NLL, non-lexical lifetimes](#nll-non-lexical-lifetimes-ch-42), [Ownership](#ownership-ch-41), [Slice](#slice-ch-43)
In repo: `projects/ownership` (chapter 4.2 comments)
Book: https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html

## Buffer overread (ch 9.1)

Reading past the end of a data structure -- undefined behavior in
C and a security vulnerability class. Rust's indexing panics
instead of returning arbitrary adjacent memory.

Related: —
In repo: `projects/panic/src/backtrace.rs` (v[99] panics)
Book: https://doc.rust-lang.org/stable/book/ch09-01-unrecoverable-errors-with-panic.html

## Channel, mpsc (ch 16.2)

A channel has two halves: transmitter (tx) and receiver (rx);
closed when either is dropped. mpsc = multiple producer, single
consumer: clone the transmitter, never the receiver. send takes
OWNERSHIP of the value (the sender cannot use it afterwards); recv
blocks until a value or the close; rx doubles as an iterator ending
at close. tx.clone() is a true capability clone, unlike Rc::clone's
refcount bump.

Related: [Message passing vs shared state](#message-passing-vs-shared-state-ch-160), [Move](#move-ch-41)
In repo: `projects/concurrency/src/channels.rs`
Book: https://doc.rust-lang.org/stable/book/ch16-02-message-passing.html

## Closure (ch 13.1)

An anonymous function, storable in a variable or passable as an
argument; unlike `fn` items, it captures its defining environment
(immutably, mutably, or by value via `move`). Parameter and return
types are inferred from the first call and locked in (E0308);
a `fn` item can see the environment but never capture it (E0434).

Related: [Combinators](#combinators-ch-92), [Fn traits](#fn-traits-ch-131), [Iterator](#iterator-ch-132), [Iterator adapters vs consuming adapters](#iterator-adapters-vs-consuming-adapters-ch-132), [move closures](#move-closures-ch-131)
In repo: `projects/closures/src/function_vs_closure.rs`

## Coherence (ch 10.2)

The global property: for any (trait, type) pair there is at most
one `impl` in the whole program. Without it, two crates could both
implement `Display for Vec<i32>`, making method calls ambiguous --
so Rust makes such programs impossible to compile. Enforced by the
orphan rule.

Related: [Orphan rule](#orphan-rule-ch-102), [Newtype pattern](#newtype-pattern-ch-102), [Trait](#trait-ch-102)
In repo: —
Book: https://doc.rust-lang.org/stable/book/ch10-02-traits.html#implementing-a-trait-on-a-type

## Combinators (ch 9.2)

Small closure-taking adapter methods on `Option`/`Result`/iterators
that compose flat pipelines instead of nested match pyramids:
`unwrap_or_else`, `map`, `map_err`, `and_then`, `ok_or`, ... The
`_else` variants are lazy -- the default is computed only on the
error path. Match stays preferable for genuinely multi-way logic.

Related: [Closure](#closure-ch-131), [Iterator](#iterator-ch-132), [Iterator adapters vs consuming adapters](#iterator-adapters-vs-consuming-adapters-ch-132), [panic vs Result guidelines](#panic-vs-result-guidelines-ch-93)
In repo: `projects/error-handling/src/main.rs` (note above the demo pair)
Book: https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html#alternatives-to-using-match-with-resultt-e

## Concurrency vs parallelism (ch 17.0)

The team metaphor: one person switching between tasks is
concurrency; several people, one task each, is parallelism; and
some work turns out serial no matter what (dependencies between
tasks). Async usually provides concurrency; threads can provide
parallelism; the two intersect (a runtime may use threads
underneath).

Related: [CPU-bound vs I/O-bound](#cpu-bound-vs-io-bound-ch-170), [Fearless concurrency](#fearless-concurrency-ch-160)
In repo: `projects/concurrency` and `projects/hello-async` (the two models side by side)
Book: https://doc.rust-lang.org/stable/book/ch17-00-async-await.html#parallelism-and-concurrency

## Constants (ch 3.1)

`const NAME: Type = expr;` -- always immutable, the type annotation
is required, the value must be computable at compile time; valid
for the whole program within its scope. SCREAMING_SNAKE naming.

Related: [Static vs const](#static-vs-const-ch-201)
In repo: —
Book: https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html#declaring-constants

## Cooperative multitasking (ch 17.3)

Tasks decide WHEN to yield -- only at await points -- and carry
the responsibility for it: a future without await points starves
the others (even an already-elapsed timer cannot fire while the
thread is blocked). The explicit handoff: yield_now. The
alternative is the OS's preemptive threads (ch 16). The sleep
triangle: thread::sleep blocks, trpl::sleep waits then yields,
yield_now yields now.

Related: [Async runtime](#async-runtime-ch-171), [Fearless concurrency](#fearless-concurrency-ch-160)
In repo: `projects/hello-async/src/yielding.rs` (the sleep triangle)
Book: https://doc.rust-lang.org/stable/book/ch17-03-more-futures.html#yielding-control-to-the-runtime

## CPU-bound vs I/O-bound (ch 17.0)

What limits the task: processing speed (a video export --
compute-bound) or the arrival of input/output (a download). The
rules of thumb that follow: CPU-bound work -> threads; I/O-bound
work -> async; combine both whenever it helps.

Related: [Concurrency vs parallelism](#concurrency-vs-parallelism-ch-170)
In repo: `projects/hello-async/src/threads_tasks.rs` (the module header)
Book: https://doc.rust-lang.org/stable/book/ch17-00-async-await.html

## Crate (ch 7.1)

The smallest unit of compilation. Two forms: binary crates
(runnable, have `main`) and library crates (shared functionality;
what "crate" colloquially means). A package may hold many binaries
but at most one library.

Related: [Binary target](#binary-target-ch-144), [Crate root](#crate-root-ch-71), [Integration tests](#integration-tests-ch-113), [Package](#package-ch-71), [Workspace](#workspace-ch-143)
In repo: `projects/aggregator` (both forms in one package)
Book: https://doc.rust-lang.org/stable/book/ch07-01-packages-and-crates.html

## Crate root (ch 7.1)

The source file the compiler starts from, making up the root
module of the crate: src/main.rs (binary) or src/lib.rs (library).
Its contents form the implicit module `crate`.

Related: [Crate](#crate-ch-71), [Documentation comments](#documentation-comments-ch-142), [Package](#package-ch-71), [Re-export](#re-export-ch-142)
In repo: `projects/restaurant/src/lib.rs`
Book: https://doc.rust-lang.org/stable/book/ch07-01-packages-and-crates.html

## Custom Cargo subcommands (ch 14.5)

Any executable named cargo-something on $PATH becomes cargo
something. The dispatch is purely name-based -- binary naming,
not author intent: hexyl is no subcommand, cargo-audit is. Custom
commands appear in cargo --list, so cargo install doubles as a
distributor of Cargo extensions.

Related: [Binary target](#binary-target-ch-144)
In repo: `exercises/14_publishing` (stage 1 installs a cargo-* binary)
Book: https://doc.rust-lang.org/stable/book/ch14-05-extending-cargo.html

## Data race (ch 4.2)

Two or more pointers to the same data, at least one writing, no
synchronization -- undefined behavior in most languages. Rust's
borrow rules (one mutable XOR many immutable) prevent data races
at compile time.

Related: [Borrow checker](#borrow-checker-ch-103), [Borrowing](#borrowing-ch-42), [Fearless concurrency](#fearless-concurrency-ch-160), [Static vs const](#static-vs-const-ch-201), [Undefined behavior](#undefined-behavior-ch-201)
In repo: —
Book: https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html#mutable-references

## Deadlock (ch 16.3)

Two threads, two locks: each acquires one and waits forever for
the other. The Mutex-era sibling of Rc's reference cycles -- a
logic bug that is memory safe and that the compiler cannot catch
(Rust prevents data races, not deadlocks). Mitigations: consistent
lock ordering, or a channel redesign (single ownership, no locks).

Related: [Memory leak](#memory-leak-ch-156), [Mutex and lock](#mutex-and-lock-ch-163)
In repo: `exercises/16_deadlock` (the book's dare, as an exercise)
Book: https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html

## Declarative macro (ch 20.5)

macro_rules!: a match on the TOKENS of the call - arms of patterns
and replacement code, compared before the compiler reads the
meaning. $( $x:expr ),* binds each matched expression to $x (zero
or more, comma-separated); $( ... )* in the body repeats in
lockstep. #[macro_export] ships it from the crate root. The vec!
sketch (listing 20-35) in seven lines; ours adds the empty arm
the book's version lacks (unused_mut on vec![]).

Related: [Macro](#macro-ch-205), [Procedural macro](#procedural-macro-ch-205)
In repo: `projects/advanced/src/macros.rs` (my_vec!)
Book: https://doc.rust-lang.org/stable/book/ch20-05-macros.html#declarative-macros-for-general-metaprogramming

## Deref coercion (ch 4.2)

The compiler converts `&String` to `&str` (and similar reference
conversions) at call sites automatically -- why `s1 + &s2` works
when `add` actually takes `&str`. The depth version (ch 15.2): the
coercion chains Deref::deref calls -- &MyBox<String> -> &String ->
&str -- as many times as needed, resolved at compile time. The
three cases: &T -> &U and the one-way &mut T -> &U (Deref), plus
&mut T -> &mut U (DerefMut); &T -> &mut T never happens.

Related: [Borrow-based lookup](#borrow-based-lookup-ch-83), [Slice](#slice-ch-43), [Smart pointers](#smart-pointers-ch-150)
In repo: `projects/collections/src/demos/strings.rs` (concatenation comment); `projects/smart-pointers/src/deref.rs` (the coercion demo)
Book: https://doc.rust-lang.org/stable/book/ch08-02-strings.html#concatenating-with--or-format

## Destructor (ch 15.3)

The general term for a cleanup function, the counterpart of a
constructor; Drop::drop is Rust's destructor -- auto-inserted at
scope end, in reverse creation order. Explicit calls are forbidden
(E0040 -- a double free); std::mem::drop is the legal early drop.
The division of responsibility is enforced: after drop() runs, the
fields drop recursively by themselves (moving them out is E0509).
Drop releases what the auto field-drop will NOT: non-memory
resources, counters (Rc's Drop decrements strong_count), and memory
held through raw pointers or FFI -- why Box/Rc deallocate in Drop.

Related: [RAII](#raii-ch-41), [Smart pointers](#smart-pointers-ch-150)
In repo: `projects/smart-pointers/src/drop_demo.rs` (module header)
Book: https://doc.rust-lang.org/stable/book/ch15-03-drop.html

## Destructuring (ch 19.3)

Breaking a value into parts by matching its shape: a pattern names
where each piece goes - in let (let (x, y) = point;), a match arm,
a for binding, or a function parameter (&(x, y)). Structs: explicit
rename (Point { x: a }) or the field shorthand (Point { x, y });
literals INSIDE a pattern test while other fields bind
(Point { x: 0, y }). Enums: the arm mirrors the variant's shape
(none / struct-like / tuple-like), nesting arbitrarily deep.
Destructuring MOVES an owning value - one construction per let.

Related: [Ignoring in patterns](#ignoring-in-patterns-ch-193), [Initializer function](#initializer-function-ch-204), [Move](#move-ch-41), [Refutable vs irrefutable](#refutable-vs-irrefutable-ch-192)
In repo: `projects/patterns/src/syntax.rs` (demo_destructuring)
Book: https://doc.rust-lang.org/stable/book/ch19-03-pattern-syntax.html#destructuring-to-break-apart-values

## Documentation comments (ch 14.2)

The /// and //! comment styles that generate HTML documentation
via rustdoc (cargo doc --open): /// documents the item AFTER it,
//! the containing item (crate or module). Markdown supported; the
# Examples blocks are compiled and run by cargo test as doctests
-- the third test section -- keeping docs and code in sync. Common
sections: # Examples, # Panics, # Errors, # Safety.

Related: [Crate root](#crate-root-ch-71), [Lint levels](#lint-levels-ch-142), [SAFETY comment](#safety-comment-ch-201)
In repo: `projects/art/src/lib.rs` (/// and //! incl. Examples/Errors)
Book: https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments

## DST, dynamically sized type (ch 20.3)

A type whose size is known only at runtime: str (not &str) and
every trait. A bare DST value is impossible (E0277) - the golden
rule is to put DSTs behind a pointer, and the pointer gets FAT:
two words, address + metadata. The metadata KIND depends on the
DST - a length for str/[T] (16 bytes for &str on x64), a VTABLE
pointer for dyn Trait (the method table of the concrete impl - the
real reason dyn Trait lives behind a pointer). Box<str> stays fat:
the metadata is part of the POINTER VALUE, not the allocation;
String is not a pointer at all - an owning struct (ptr/len/cap,
24 bytes). Generic fns get an implicit T: Sized; ?Sized relaxes it
(the only trait allowed in the ?Trait syntax), the parameter then
going behind a pointer.

Related: [Slice](#slice-ch-43), [Trait object](#trait-object-ch-182)
In repo: `projects/advanced/src/types.rs` (demo_dst)
Book: https://doc.rust-lang.org/stable/book/ch20-03-advanced-types.html#dynamically-sized-types-and-the-sized-trait

## Duck typing (ch 18.2)

"If it walks like a duck and quacks like a duck..." - caring only
about the messages a value responds to, not its concrete type.
Trait objects give Rust a compile-time CHECKED version: run's
loop calls draw() without knowing the component - and a type
missing the method is an E0277 before any code runs, not a
runtime error.

Related: [Trait bound](#trait-bound-ch-101), [Trait object](#trait-object-ch-182)
In repo: `projects/oop/src/select_box.rs` (the 18-10 comment)
Book: https://doc.rust-lang.org/stable/book/ch18-02-trait-objects.html

## Dyn compatibility (ch 18.2)

The rules a trait must satisfy to be usable as dyn Trait. The
headline: no methods returning Self by value or taking self by
value - the compiler cannot know the concrete type at the call
site. self: Box<Self> works (the state pattern's transitions);
default impls returning self do not (hence some duplication in
blog.rs). Renamed from "object safety" - the old name still
appears in older material.

Related: [Trait object](#trait-object-ch-182), [State pattern](#state-pattern-ch-183)
In repo: `projects/oop/src/blog.rs` (the Box<Self> signatures)
Book: https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility

## Encapsulation (ch 18.1)

Implementation details unreachable from outside: private by
default, pub as the deliberate API. The AveragedCollection
invariant: fields stay private, so the cached average can never
fall out of sync - and the internals (a Vec becoming a HashSet?)
can change without touching the users.

Related: [Re-export](#re-export-ch-142), [State pattern](#state-pattern-ch-183)
In repo: `projects/oop/src/encapsulation.rs` (the E0603 probe in main)
Book: https://doc.rust-lang.org/stable/book/ch18-01-what-is-oo.html#encapsulation-that-hides-implementation-details

## Fearless concurrency (ch 16.0)

The chapter's namesake: ownership and type checking turn many
concurrency errors into COMPILE-TIME errors -- a wrong program
refuses to build instead of racing in production. The tools are
mostly standard-library types (threads, channels, Mutex, Arc)
gated by two language-level marker traits, Send and Sync.

Related: [Concurrency vs parallelism](#concurrency-vs-parallelism-ch-170), [Cooperative multitasking](#cooperative-multitasking-ch-173), [Data race](#data-race-ch-42), [Send and Sync](#send-and-sync-ch-164)
In repo: `projects/concurrency` (the whole chapter)
Book: https://doc.rust-lang.org/stable/book/ch16-00-concurrency.html

## FFI, foreign function interface (ch 20.1)

The boundary between Rust and other languages. An `unsafe extern "C"`
block DECLARES a foreign fn -- signature only, no body; the
definition lives in a linked library, resolved by the unmangled C
symbol (a wrong name is a LINK error, not a runtime one). Calls are
unsafe unless the decl carries the `safe` keyword -- the author's
PROMISE that no unsafe block is needed (abs(i32) has no
memory-safety concerns). C symbols are program-global: abs cannot
be redeclared with another safety, so the safe-keyword demo uses
labs. Miri cannot execute native code -- cfg(miri) stub demos keep
a project miri-clean.

Related: [Function pointer](#function-pointer-ch-204), [Union](#union-ch-201), [unsafe Rust](#unsafe-rust-ch-201)
In repo: `projects/advanced/src/unsafe_rust.rs` (demo_ffi, demo_ffi_safe)
Book: https://doc.rust-lang.org/stable/book/ch20-01-unsafe-rust.html#using-extern-functions-to-call-external-code

## Fn traits (ch 13.1)

The additive closure hierarchy picked by API bounds: `FnOnce`
(may move captures out -- callable once), `FnMut` (may mutate --
repeatable), `Fn` (read-only). Every closure implements all it
can; bounds choose the weakest they need (e.g. `unwrap_or_else`
takes `FnOnce`, `sort_by_key` takes `FnMut`).

Related: [Closure](#closure-ch-131), [Function pointer](#function-pointer-ch-204), [Initializer function](#initializer-function-ch-204), [Opaque type, impl Trait](#opaque-type-impl-trait-ch-102), [Trait bound](#trait-bound-ch-101), [Type alias](#type-alias-ch-203)
In repo: `projects/closures/src/fn_traits.rs`

## Fully qualified syntax (ch 20.2)

The most explicit call form: `<Type as Trait>::function(receiver,
args...)` - "treat Dog as Animal for this call". Needed when
associated functions without self collide: Dog::baby_name picks the
inherent fn; Animal::baby_name is E0790 (which impl of Animal?);
<Dog as Animal>::baby_name() says it exactly. With methods, the
receiver already disambiguates (Pilot::fly(&person) suffices).
Every inferable part of the form is omittable - FQS is the spot
where nothing can be inferred away.

Related: [Trait](#trait-ch-102), [Trait must be in scope](#trait-must-be-in-scope-ch-92)
In repo: `projects/advanced/src/traits.rs` (demo_fully_qualified_syntax)
Book: https://doc.rust-lang.org/stable/book/ch20-02-advanced-traits.html#disambiguating-between-identically-named-methods

## Function pointer (ch 20.4)

fn (lowercase f) - a TYPE, not to be confused with the Fn closure
trait: a concrete type written directly in a parameter (do_twice(f:
fn(i32) -> i32)), no generic bound needed. Function names ARE
pointers; fn implements all three closure traits, so a generic +
Fn API accepts both closures and fns - write APIs that way;
fn-only parameters are the C-FFI niche (C has no closures).

Related: [FFI, foreign function interface](#ffi-foreign-function-interface-ch-201), [Fn traits](#fn-traits-ch-131)
In repo: `projects/advanced/src/functions.rs` (demo_fn_pointers)
Book: https://doc.rust-lang.org/stable/book/ch20-04-advanced-functions-and-closures.html#function-pointers

## Future (ch 17.1)

A value that may not be ready yet; `async fn` compiles to a plain
function returning an anonymous Future type -- a state machine
over the await points. LAZY, like an iterator: creating a future
does nothing, awaiting runs it (unlike JS promises, which start on
creation). The trait's heartbeat: poll(self: Pin<&mut Self>, cx)
-> Poll<Output>.

Related: [Async runtime](#async-runtime-ch-171), [Iterator](#iterator-ch-132), [Pin and Unpin](#pin-and-unpin-ch-175), [Stream](#stream-ch-174)
In repo: `projects/hello-async/src/futures.rs` (the desugared form)
Book: https://doc.rust-lang.org/stable/book/ch17-01-futures-and-syntax.html

## Ignoring in patterns (ch 19.3)

Three grades. `_`: never binds - the value is not even moved, it
stays usable afterwards. A `_`-prefixed name (`_x`): silences the
unused-variable warning but STILL BINDS - an owning value is moved
out (E0382 if used again). `..`: the unmatched rest - allowed once
per pattern; `(.., x, ..)` is ambiguous and rejected. A nested `_`
tests the SHAPE while ignoring contents: (Some(_), Some(_)).

Related: [Destructuring](#destructuring-ch-193), [Move](#move-ch-41)
In repo: `projects/patterns/src/syntax.rs` (demo_ignoring)
Book: https://doc.rust-lang.org/stable/book/ch19-03-pattern-syntax.html#ignoring-values-in-a-pattern

## Indirection (ch 15.1)

Storing a POINTER to the next value instead of the value inline.
An enum reserves max(variant sizes) + tag; a recursive variant
makes that equation self-referential (E0072, "infinite size").
A pointer occupies usize bytes regardless of the pointee, so a
Box<List> tail gives every node a fixed size -- the nesting moves
to the heap. For the cons list, indirection is the ONLY feature
needed: no Rc/RefCell machinery.

Related: [Smart pointers](#smart-pointers-ch-150)
In repo: `projects/smart-pointers/src/box_demo.rs` (commented E0072 + size math)
Book: https://doc.rust-lang.org/stable/book/ch15-01-box.html#enabling-recursive-types-with-boxes

## Initializer function (ch 20.4)

Every enum variant name IS a function: Status::Value has the type
fn(u32) -> Status; tuple-struct names are initializers too. Being
fn items they implement the closure traits, so they stand wherever
a closure is expected: (0..20).map(Status::Value). The ch 6 fact
(variant names as constructors) - now with a type. The pattern-side
mirror is destructuring: what an initializer builds, a pattern
takes apart.

Related: [Destructuring](#destructuring-ch-193), [Fn traits](#fn-traits-ch-131)
In repo: `projects/advanced/src/functions.rs` (demo_fn_pointers)
Book: https://doc.rust-lang.org/stable/book/ch20-04-advanced-functions-and-closures.html#function-pointers

## Inner vs outer attributes (ch 9.2)

`#[...]` is an OUTER attribute: it attaches to the next item (e.g.
a single `mod` line). `#![...]` is an INNER attribute: it applies
to the enclosing item -- in the crate root, to the whole crate.
The `!` here means "inner", not a macro invocation.

Related: [Lint levels](#lint-levels-ch-142)
In repo: `projects/error-handling/src/main.rs` (commented-out `#![allow(dead_code)]`)
Book: https://doc.rust-lang.org/reference/attributes.html

## Integration tests (ch 11.3)

External to the library: files in tests/, each compiled as its
own crate using only the public API -- exactly like an outside
consumer. No #[cfg(test)] needed. Impossible for binary-only
crates (main.rs exposes nothing to `use`); the lib+bin pattern
is the remedy.

Related: [Crate](#crate-ch-71), [Unit tests](#unit-tests-ch-113)
In repo: `projects/testing/tests/integration_test.rs`
Book: https://doc.rust-lang.org/stable/book/ch11-03-test-organization.html#integration-tests

## Interior mutability (ch 15.5)

A design pattern: mutating data even though only an immutable
reference to it exists -- normally forbidden by the borrowing
rules. Built on unsafe code wrapped in a safe API: RefCell<T>
moves the borrow checking to RUNTIME (a violation panics -- already
borrowed -- instead of failing to compile). The canonical use:
mock objects recording calls behind &self. Single-threaded;
Mutex<T> is the threaded sibling (ch 16). Rc<RefCell<T>> combines
multiple owners with mutation.

Related: [Mutex and lock](#mutex-and-lock-ch-163), [Rc and reference counting](#rc-and-reference-counting-ch-154), [Test double and mock objects](#test-double-and-mock-objects-ch-155)
In repo: `projects/smart-pointers/src/refcell.rs` and `messenger.rs`
Book: https://doc.rust-lang.org/stable/book/ch15-05-interior-mutability.html

## Iterator (ch 13.2)

The sequence abstraction: a type implementing the `Iterator` trait
(sole required method `next`, handing out one `Item` at a time as
Some, then None). Lazy -- no work happens until a consuming method
runs; a for loop creates and consumes one implicitly. iter() borrows
immutably, iter_mut() mutably, into_iter() takes ownership.

Related: [Associated type](#associated-type-ch-202), [Closure](#closure-ch-131), [Combinators](#combinators-ch-92), [Future](#future-ch-171), [Iterator adapters vs consuming adapters](#iterator-adapters-vs-consuming-adapters-ch-132), [Stream](#stream-ch-174), [Zero-cost abstraction](#zero-cost-abstraction-ch-134)
In repo: `projects/iterators/src/laziness.rs`

## Iterator adapters vs consuming adapters (ch 13.2)

Two method families on `Iterator`: adapters (map, filter, take) are
lazy -- they wrap the source in a NEW iterator and do no work; a
chain must end in a consuming adapter (sum, collect) that calls
next to exhaustion and takes ownership of the iterator. The
unused-`Map` warning is how the compiler enforces laziness
awareness.

Related: [Iterator](#iterator-ch-132), [Closure](#closure-ch-131), [Combinators](#combinators-ch-92)
In repo: `projects/iterators/src/iterator_adapters.rs`

## Lifetime (ch 10.3)

`'a`: generic lifetime parameters tying references together in a
signature. They describe RELATIONSHIPS ("valid as long as...") and
never change how long anything lives. Every reference has a
lifetime; usually it is inferred.

Related: [Borrow checker](#borrow-checker-ch-103), [Borrowing](#borrowing-ch-42), [Lifetime elision rules](#lifetime-elision-rules-ch-103), [NLL, non-lexical lifetimes](#nll-non-lexical-lifetimes-ch-42), [Static lifetime](#static-lifetime-ch-103)
In repo: `projects/lifetimes/src/longest.rs`
Book: https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html#lifetime-annotation-syntax

## Lifetime elision rules (ch 10.3)

Three deterministic patterns the compiler applies so annotations
can be omitted: (1) each reference parameter gets its own lifetime;
(2) with exactly one input lifetime, it is assigned to all outputs;
(3) in a method with &self, self's lifetime is assigned to all
outputs. Ambiguity after the rules -> E0106.

Related: [Lifetime](#lifetime-ch-103)
In repo: `projects/lifetimes/src/excerpt.rs` (rule 3 in announce_and_return_part)
Book: https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html#lifetime-elision

## Lint levels (ch 14.2)

The four strengths a lint attribute can set: allow (silence the
lint), warn (warning -- the default for most lints), deny (compilation
error), forbid (deny plus immune to a later allow). Set crate-wide
with an inner attribute at the crate root: #![warn(missing_docs)]
demands doc comments on every public item -- including pub mod and
enum variants, which is stricter than the book's own art example.
A stricter sibling to try: missing_docs_in_private_items.

Related: [Documentation comments](#documentation-comments-ch-142), [Inner vs outer attributes](#inner-vs-outer-attributes-ch-92), [SAFETY comment](#safety-comment-ch-201)
In repo: `projects/art/src/lib.rs` (the crate-root attribute)
Book: https://doc.rust-lang.org/rustc/lints/levels.html

## Macro (ch 20.5)

A family, not one feature: code that writes code, expanded BEFORE
compilation - before types exist, hence powers no fn has (variable
arity: vec![1] or vec![1, 2, 3]; implementing traits - runtime is
too late). The price: definitions are more complex than fns and
must be defined (or brought into scope) BEFORE use in the file.
Four kinds: one declarative (macro_rules!) and three procedural
(derive, attribute-like, function-like).

Related: [Declarative macro](#declarative-macro-ch-205), [Procedural macro](#procedural-macro-ch-205)
In repo: `projects/advanced/src/macros.rs`, `projects/macros`
Book: https://doc.rust-lang.org/stable/book/ch20-05-macros.html#the-difference-between-macros-and-functions

## Match guard (ch 19.3)

An if AFTER the pattern in a match arm - the arm fires only when
BOTH hold. Guards are not patterns: their variables are the OUTER
ones (the shadowing cure: Some(n) if n == y), and conditions like
"x is even" have no pattern spelling. The guard applies to the
WHOLE or-pattern - (1 | 3 | 5) if y - and per-alternative guards
do not exist on stable (E0658, experimental). The cost: with
guards involved, exhaustiveness is no longer checked.

Related: [Shadowing](#shadowing-ch-31)
In repo: `projects/patterns/src/syntax.rs` (demo_shadowing_traps)
Book: https://doc.rust-lang.org/stable/book/ch19-03-pattern-syntax.html#adding-conditionals-with-match-guards

## Memory leak (ch 15.6)

Memory that is never cleaned up. Rust does NOT guarantee
leak-freedom: leaks are memory safe. Rc<T> + RefCell<T> can form
reference cycles whose strong counts never reach zero -- a logic
bug the compiler cannot catch. Guards: tests and reviews, or
breaking the cycle with Weak<T>.

Related: [Rc and reference counting](#rc-and-reference-counting-ch-154), [Strong vs weak references](#strong-vs-weak-references-ch-156), [Deadlock](#deadlock-ch-163)
In repo: `projects/smart-pointers/src/cycles.rs` (the frozen 2/2 counts)
Book: https://doc.rust-lang.org/stable/book/ch15-06-reference-cycles.html

## Message passing vs shared state (ch 16.0)

The two approaches to concurrency, both first-class in Rust
(the Go slogan: "Do not communicate by sharing memory; instead,
share memory by communicating"). Message passing = channels, where
ownership of each value TRANSFERS to the receiver -- single
ownership, like moving. Shared state = Mutex-protected data with
multiple owners -- like Rc's shared ownership, with locking instead
of borrow checking.

Related: [Channel, mpsc](#channel-mpsc-ch-162), [Mutex and lock](#mutex-and-lock-ch-163)
In repo: `projects/concurrency/src/channels.rs` and `shared_state.rs`
Book: https://doc.rust-lang.org/stable/book/ch16-00-concurrency.html

## Miri (ch 20.1)

An interpreter for Rust's mid-level IR: runs the program without
native codegen and catches undefined behavior as it happens --
out-of-bounds, use-after-free, dangling pointers, data races,
Stacked Borrows aliasing violations. Silent about sound code; on
unsupported operations its own note says "it does not indicate a
bug in the program". cargo +nightly miri run (the first run builds
its own std from the rust-src component).

Related: [Rustonomicon](#rustonomicon-ch-201), [Undefined behavior](#undefined-behavior-ch-201), [unsafe Rust](#unsafe-rust-ch-201)
In repo: `exercises/20_miri`
Book: https://doc.rust-lang.org/stable/book/ch20-01-unsafe-rust.html

## Monomorphization (ch 10.1)

How Rust compiles generics: the compiler generates a concrete copy
of the generic code for every type it is used with -- static
dispatch, zero runtime cost, larger binaries and longer compile
times. The opposite strategy is type erasure (TypeScript, Java):
one copy, dynamic dispatch. The practical trade-off returns in
chapter 18 as `impl Trait` vs `dyn Trait`.

Related: [Opaque type, impl Trait](#opaque-type-impl-trait-ch-102), [Static vs dynamic dispatch](#static-vs-dynamic-dispatch-ch-182), [Trait bound](#trait-bound-ch-101), [Trait must be in scope](#trait-must-be-in-scope-ch-92), [Zero-cost abstraction](#zero-cost-abstraction-ch-134)
In repo: —
Book: https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#performance-of-code-using-generics

## Move (ch 4.1)

Assignment that invalidates the source: the stack part (pointer,
length, capacity) is copied, the heap data is not, and the source
variable dies. Not a shallow copy -- precisely because of the
invalidation; this prevents double free. Rust never deep-copies
implicitly (explicit deep copy: clone).

Related: [Borrowing](#borrowing-ch-42), [Channel, mpsc](#channel-mpsc-ch-162), [Destructuring](#destructuring-ch-193), [Ignoring in patterns](#ignoring-in-patterns-ch-193), [move closures](#move-closures-ch-131), [Ownership](#ownership-ch-41)
In repo: `projects/collections/src/demos/hashmaps.rs` (managing ownership demo)
Book: https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move

## move closures (ch 13.1)

The `move` keyword before a closure's parameter list forces capture
by value even when the body would only need a reference -- required
when the closure must outlive the enclosing scope (thread::spawn
demands 'static data).

Related: [Closure](#closure-ch-131), [Move](#move-ch-41), [Static lifetime](#static-lifetime-ch-103)
In repo: `projects/closures/src/capturing_references.rs`

## Mutex and lock (ch 16.3)

Mutual exclusion: only one thread holds the data at a time. The
data lives BEHIND Mutex<T> -- the type system forces lock() before
any use. The returned MutexGuard derefs to the data (Deref) and
releases the lock at scope end (Drop) -- both ch 15 themes in one
type; forgetting to unlock is impossible. lock() returns Result: a
panicking holder poisons the mutex. Mutex provides interior
mutability, like RefCell -- with deadlocks as its uncatchable bug.

Related: [Arc, atomic reference counting](#arc-atomic-reference-counting-ch-163), [Interior mutability](#interior-mutability-ch-155), [Message passing vs shared state](#message-passing-vs-shared-state-ch-160), [Deadlock](#deadlock-ch-163)
In repo: `projects/concurrency/src/shared_state.rs` (demo_mutex_api)
Book: https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html

## Never type (ch 20.3)

! - the empty type, zero values; the type of expressions that
never produce one: continue, panic!, a breakless loop, diverging
fns (fn bar() -> !). Coerces into ANY type - why a match may mix
`Ok(num) => num` with `Err(_) => continue` (the ch 2 guess game,
finally explained) and Option::unwrap may pair T with a panic!
arm. Without ! there is no coercion - mismatched arm types stay
E0308.

Related: [panic vs Result guidelines](#panic-vs-result-guidelines-ch-93), [Refutable vs irrefutable](#refutable-vs-irrefutable-ch-192)
In repo: `projects/advanced/src/types.rs` (demo_never_type)
Book: https://doc.rust-lang.org/stable/book/ch20-03-advanced-types.html#the-never-type-that-never-returns

## Newtype pattern (ch 10.2)

Wrap a foreign type in a local tuple struct (`struct Meters(f64)`)
to gain the right to implement traits for it under the orphan rule
(ch 20.2's Wrapper around Vec<String> for Display); also serves
nominal branding (Millimeters vs Meters - even across the +
operator), like branded types in TypeScript. Ch 20.3's opening
adds API abstraction: the People wrapper hides its HashMap and
its ids - callers speak only names. Zero runtime cost - the
wrapper elides at compile time; methods are NOT inherited (Deref
delegation or manual forwarding, ch 15.2).

Related: [Coherence](#coherence-ch-102), [Operator overloading](#operator-overloading-ch-202), [Orphan rule](#orphan-rule-ch-102), [Type alias](#type-alias-ch-203)
In repo: `projects/advanced/src/traits.rs` (Millimeters, Wrapper)
Book: https://doc.rust-lang.org/stable/book/ch10-02-traits.html#implementing-a-trait-on-a-type

## NLL, non-lexical lifetimes (ch 4.2)

A borrow ends at its LAST USE, not at the end of the scope: using a
reference and mutating the owner afterwards compiles fine, while
mutating between two uses does not.

Related: [Borrow checker](#borrow-checker-ch-103), [Borrowing](#borrowing-ch-42), [Lifetime](#lifetime-ch-103)
In repo: `projects/collections/src/demos/vectors.rs` (borrow conflict demo)
Book: https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html#mutable-references

## Opaque type, impl Trait (ch 10.2)

`-> impl Trait` hides the concrete return type; the caller sees only
the trait's methods (no Debug/Display through it) and cannot name
the type. A single concrete type only -- different types per branch
need Box<dyn Trait> (ch 18). Ch 20.4 sharpened it: every USE of
impl Trait is a DISTINCT opaque type - two impl-Fn returns cannot
share a Vec (E0308, "distinct uses of impl Trait result in
different opaque types"; the async-block parallel from ch 17); the
trait object - Box<dyn Fn> - is the one-type fix.

Related: [Fn traits](#fn-traits-ch-131), [Monomorphization](#monomorphization-ch-101), [Trait](#trait-ch-102)
In repo: `projects/aggregator/src/main.rs` (the return summarizable comment)
Book: https://doc.rust-lang.org/stable/book/ch10-02-traits.html#returning-types-that-implement-traits

## Operator overloading (ch 20.2)

Custom behavior for existing operators by implementing the traits
in std::ops (+ is the Add trait) - inventing NEW operators is
impossible. Add carries the associated type Output and the DEFAULT
TYPE PARAMETER Rhs = Self: a plain `impl Add for Point` means
Point + Point; `impl Add<Meters> for Millimeters` pins the
right-hand side to another type, the unit conversion living inside
the impl. Operators are traits in method-call disguise: a + b is
Add::add(a, b).

Related: [Associated type](#associated-type-ch-202), [Newtype pattern](#newtype-pattern-ch-102), [Trait](#trait-ch-102)
In repo: `projects/advanced/src/traits.rs` (demo_operator_overloading)
Book: https://doc.rust-lang.org/stable/book/ch20-02-advanced-traits.html#using-default-generic-parameters-and-operator-overloading

## Orphan rule (ch 10.2)

The enforcement mechanism of coherence: writing `impl Trait for
Type` requires the trait OR the type to be local to your crate.
Named for the forbidden case -- an impl whose "parents" (both the
trait and the type) are absent from the current crate.

Related: [Coherence](#coherence-ch-102), [Newtype pattern](#newtype-pattern-ch-102)
In repo: —
Book: https://doc.rust-lang.org/stable/book/ch10-02-traits.html#implementing-a-trait-on-a-type

## Ownership (ch 4.1)

The set of rules governing memory: each value has exactly ONE
owner; when the owner goes out of scope, `drop` frees the memory
(RAII). Checked entirely at compile time -- the third way between
garbage collection and manual malloc/free.

Related: [Borrowing](#borrowing-ch-42), [Move](#move-ch-41), [RAII](#raii-ch-41)
In repo: `projects/ownership`
Book: https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html

## Package (ch 7.1)

A bundle of one or more crates with a Cargo.toml describing how to
build them. At most one library crate; any number of binaries
(src/main.rs plus each file in src/bin/). Cargo itself is a
package.

Related: [Crate](#crate-ch-71), [Crate root](#crate-root-ch-71), [Workspace](#workspace-ch-143)
In repo: every project's Cargo.toml in this repo
Book: https://doc.rust-lang.org/stable/book/ch07-01-packages-and-crates.html

## panic vs Result guidelines (ch 9.3)

panic! (and unwrap/expect) is for failures that mean a bug -- a
broken invariant -- and is fine in examples, prototypes and tests.
Result is for expected failures (missing files, bad input) where
the caller may recover; propagate with `?`.

Related: [Backtrace](#backtrace-ch-91), [Combinators](#combinators-ch-92), [Never type](#never-type-ch-203), [Unwinding vs abort](#unwinding-vs-abort-ch-91)
In repo: anchors in `projects/error-handling/src/main.rs` and `projects/panic/src/main.rs`
Book: https://doc.rust-lang.org/stable/book/ch09-03-to-panic-or-not-to-panic.html

## Pin and Unpin (ch 17.5)

Compiler-generated futures may be self-referential -- moving one
would dangle its internal references. Pin<P> wraps a pointer-like
P guaranteeing the pointed-to value never moves (the pointer
itself may). Unpin is a marker trait -- like Send/Sync -- meaning
"safe to move even under Pin"; almost every type has it, and it
only matters when Pin is involved. Plain await pins implicitly;
collecting futures (join_all) demands an explicit pin!.

Related: [Future](#future-ch-171), [Send and Sync](#send-and-sync-ch-164)
In repo: `projects/hello-async/src/pinning.rs`
Book: https://doc.rust-lang.org/stable/book/ch17-05-traits-for-async.html

## Procedural macro (ch 20.5)

A fn over token streams - TokenStream in, TokenStream out - in its
OWN crate (proc-macro = true); the attribute picks the kind:
#[proc_macro_derive] (impl generation - the HelloMacro case: syn
parses to a syntax tree, quote! templates the impl back,
stringify! freezing the name into a compile-time literal),
#[proc_macro_attribute] (two streams - the attribute's tokens and
the item; MUST return the item or it vanishes), #[proc_macro]
(function-like). Runs at COMPILE time of the consumer: println!
inside lands in the build log; the expansion cache re-runs it only
when the consumer's code changes.

Related: [Declarative macro](#declarative-macro-ch-205), [Macro](#macro-ch-205)
In repo: `projects/macros/hello_macro_derive` (HelloMacro, logged)
Book: https://doc.rust-lang.org/stable/book/ch20-05-macros.html#procedural-macros-for-generating-code-from-attributes

## RAII (ch 4.1)

Resource Acquisition Is Initialization (a C++ term): resources are
released at the end of an item's lifetime. Rust's drop-at-scope-end
is this pattern -- the book notes the kinship explicitly.

Related: [Destructor](#destructor-ch-153), [Ownership](#ownership-ch-41)
In repo: —
Book: https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html

## Raw pointer (ch 20.1)

*const T / *mut T: an address that may be null, unaligned, aliased
or dangling -- the compiler assumes nothing, so creating one is
SAFE (&raw const / &raw mut borrow operators; the old `&x as
*const T` cast still roams) while dereferencing needs unsafe. No
Deref, no Drop -- unlike a smart pointer it owns nothing;
Box::into_raw hands heap ownership over, Box::from_raw takes it
back (the OwnedI32 RAII cycle). A slice IS pointer + length:
as_mut_ptr + from_raw_parts_mut reassemble one (the split_at_mut
case, sound because mid <= len is asserted).

Related: [Slice](#slice-ch-43), [Smart pointers](#smart-pointers-ch-150), [unsafe Rust](#unsafe-rust-ch-201)
In repo: `projects/advanced/src/unsafe_rust.rs` (listings 20-1..20-7)
Book: https://doc.rust-lang.org/stable/book/ch20-01-unsafe-rust.html#dereferencing-a-raw-pointer

## Rc and reference counting (ch 15.4)

Rc<T>: multiple owners of one value. Rc::clone bumps strong_count
(a counter increment, never a deep copy); the Drop impl decrements;
at zero the value is cleaned. Immutable access only, single
thread (Arc<T> is the threaded sibling, ch 16). The convention of
writing Rc::clone(&a) rather than a.clone() exists to keep deep
copies visually distinguishable from counter bumps.

Related: [Arc, atomic reference counting](#arc-atomic-reference-counting-ch-163), [Interior mutability](#interior-mutability-ch-155), [Memory leak](#memory-leak-ch-156), [Smart pointers](#smart-pointers-ch-150), [Strong vs weak references](#strong-vs-weak-references-ch-156)
In repo: `projects/smart-pointers/src/rc.rs`
Book: https://doc.rust-lang.org/stable/book/ch15-04-rc.html

## Re-export (ch 14.2)

pub use: making a public item public in ANOTHER location, as if it
were defined there -- decoupling the internal module hierarchy
from the public API. Users write use art::PrimaryColor instead of
use art::kinds::PrimaryColor; cargo doc lists re-exports on the
crate front page. The deep paths remain available too.

Related: [Crate root](#crate-root-ch-71), [Encapsulation](#encapsulation-ch-181)
In repo: `projects/art/src/lib.rs` (the collapsed pub use block)
Book: https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api

## Refutable vs irrefutable (ch 19.2)

Whether a pattern can fail. Irrefutable matches any value (x, a
tuple of names, Point { x, y }) - demanded by let, for, and
function parameters, where a miss has no meaningful handling.
Refutable can fail (Some(x), 5, Ok(n)) - plain let rejects it
(E0005, the compiler itself suggesting let...else); if let /
while let / let...else exist to give the miss an exit. The
reverse mismatch - an irrefutable pattern in let...else - warns
about the unreachable else.

Related: [Destructuring](#destructuring-ch-193), [Never type](#never-type-ch-203)
In repo: `projects/patterns/src/refutability.rs`
Book: https://doc.rust-lang.org/stable/book/ch19-02-refutability.html

## Release profiles (ch 14.1)

Predefined customizable build configurations: dev (cargo build --
fast compiles, opt-level 0, unoptimized + debuginfo) and release
(cargo build --release -- opt-level 3, optimized; compile once,
run many). Overridden per-project via [profile.*] sections in
Cargo.toml; already used in this repo: panic = 'abort' in the
panic project's release profile (ch 9.1).

Related: [Unwinding vs abort](#unwinding-vs-abort-ch-91)
In repo: `projects/panic/Cargo.toml` (the [profile.release] customization)

## Resolver versions (ch 14.3)

The version of Cargo's dependency resolution algorithm
(resolver = "1"|"2"|"3" in the workspace root). The dividing line
runs through FEATURE UNIFICATION: resolver 1 merges feature flags
across all platforms and dependency kinds (host, target, dev),
activating features a build never asked for; 2 (edition 2021
default) stopped unifying platform-specific and host deps; 3
(edition 2024 default) also keeps dev-dependency features out of
normal builds. A VIRTUAL workspace has no edition to imply the
version -- it must be set explicitly, or cargo falls back to 1
with a warning.

Related: [Workspace](#workspace-ch-143)
In repo: `projects/add/Cargo.toml` (the workspace root)
Book: https://doc.rust-lang.org/cargo/reference/resolver.html

## Rustonomicon (ch 20.1)

The book on unsafe Rust internals: aliasing rules (why &mut must
stay unique; split_at_mut as the canonical sound example), manual
Send/Sync impls, unions, transmute. The go-to after ch 20.1 --
everything the chapter leaves out of frame lives there.

Related: [Miri](#miri-ch-201), [unsafe Rust](#unsafe-rust-ch-201)
In repo: `projects/advanced/src/unsafe_rust.rs` (the header link)
Book: https://doc.rust-lang.org/nomicon/

## SAFETY comment (ch 20.1)

The convention accompanying unsafe: `// SAFETY:` directly above an
unsafe block (or unsafe impl), `# Safety` doc section on an unsafe
fn or unsafe trait -- each stating the invariant that makes the
code sound ("mid <= len is asserted, so the halves are disjoint").
Mechanically enforceable: clippy::undocumented_unsafe_blocks warns
on every uncommented block/impl (on in projects/advanced),
missing_safety_doc demands the doc sections.

Related: [Documentation comments](#documentation-comments-ch-142), [Lint levels](#lint-levels-ch-142), [unsafe Rust](#unsafe-rust-ch-201)
In repo: `projects/advanced/src/main.rs` (the enforcing lint)
Book: https://doc.rust-lang.org/stable/book/ch20-01-unsafe-rust.html#accessing-or-modifying-a-mutable-static-variable

## Send and Sync (ch 16.4)

The two language-level marker traits (no methods) behind fearless
concurrency. Send: ownership of the type may be TRANSFERRED between
threads (Rc is not Send -- its count is not thread-safe; Arc is).
Sync: &T is safe to share across threads (&T: Send); RefCell/Cell
are not Sync, Mutex is. Compositions inherit automatically; manual
impls are unsafe (ch 20). The E0277 of the Rc counter attempt is
these traits talking.

Related: [Arc, atomic reference counting](#arc-atomic-reference-counting-ch-163), [Fearless concurrency](#fearless-concurrency-ch-160), [Pin and Unpin](#pin-and-unpin-ch-175), [unsafe trait](#unsafe-trait-ch-201)
In repo: `projects/concurrency/src/shared_state.rs` (the 16-14 comment)
Book: https://doc.rust-lang.org/stable/book/ch16-04-extensible-concurrency-sync-and-send.html

## Shadowing (ch 3.1)

`let` with an already-used name creates a NEW variable that
overshadows the old one until it is itself shadowed or the scope
ends. Unlike `mut`: the type may change across shadowings, and the
variable stays immutable between them.

Related: [Match guard](#match-guard-ch-193)
In repo: `projects/error-handling/src/error_propagation.rs` (string1 re-bound per version)
Book: https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html#shadowing

## SipHash and BuildHasher (ch 8.3)

The default `HashMap` hasher is SipHash: DoS-resistant, trading
some speed for security. A faster/slower hasher can be plugged in
via a type implementing the `BuildHasher` trait.

Related: —
In repo: `projects/collections/src/demos/hashmaps.rs` (comment above the use line)
Book: https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#hashing-functions

## Slice (ch 4.3)

A reference to a contiguous sequence: `&str` for strings, `&[T]`
for collections. A fat reference (pointer + length) with no
ownership; taking &str (not &String) lets a function accept both
String slices and literals -- via deref coercions.

Related: [Borrowing](#borrowing-ch-42), [Deref coercion](#deref-coercion-ch-42), [DST, dynamically sized type](#dst-dynamically-sized-type-ch-203), [Raw pointer](#raw-pointer-ch-201)
In repo: `projects/collections/src/demos/strings.rs` (the slicing demo)
Book: https://doc.rust-lang.org/stable/book/ch04-03-slices.html

## Smart pointers (ch 15.0)

Data structures acting like a pointer while OWNING the data they
point to (references only borrow); built as structs implementing
Deref (behave like references) and Drop (custom cleanup). Box::new
allocates fresh and holds a copy (Copy types) or the moved value --
a Box can never alias its source. The chapter's set: Box (heap),
Rc (shared ownership), RefCell (runtime borrow rules), Weak
(non-owning).

Related: [Deref coercion](#deref-coercion-ch-42), [Destructor](#destructor-ch-153), [Indirection](#indirection-ch-151), [Raw pointer](#raw-pointer-ch-201), [Rc and reference counting](#rc-and-reference-counting-ch-154), [Strong vs weak references](#strong-vs-weak-references-ch-156), [Trait object](#trait-object-ch-182)
In repo: `projects/smart-pointers` (the whole chapter)
Book: https://doc.rust-lang.org/stable/book/ch15-00-smart-pointers.html

## Standard output vs standard error (ch 12.6)

The two output streams of a command line program: stdout carries
the data (successful results), stderr the diagnostics (error
messages). The split lets `program > out.txt` capture results in a
file while errors still reach the screen. println! writes to
stdout only; eprintln! writes to stderr -- minigrep's error paths
print via eprintln! and exit nonzero (process::exit(1)).

Related: —
In repo: `projects/minigrep/src/main.rs` (both eprintln! branches in main)
Book: https://doc.rust-lang.org/stable/book/ch12-06-writing-to-stderr-instead-of-stdout.html

## State pattern (ch 18.3)

Behavior depends on the state, and the transition rules live
INSIDE the state objects - the value holding them knows nothing.
Rust hosts three roads: OO-style dyn states (blog.rs), states as
TYPES - invalid states do not compile (blog_typestate.rs), and
the enum + match road (exercises/18_state_pattern).

Related: [Dyn compatibility](#dyn-compatibility-ch-182), [Encapsulation](#encapsulation-ch-181), [Trait object](#trait-object-ch-182)
In repo: `projects/oop/src/blog.rs` and `blog_typestate.rs`
Book: https://doc.rust-lang.org/stable/book/ch18-03-oo-design-patterns.html

## Static lifetime (ch 10.3)

`'static`: the reference is valid for the entire program; every
string literal is 'static (stored in the binary). An error message
suggesting 'static is usually a dangling-reference smell -- fix
the lifetimes instead of reaching for it.

Related: [Lifetime](#lifetime-ch-103), [move closures](#move-closures-ch-131), [Static vs const](#static-vs-const-ch-201)
In repo: `projects/lifetimes/src/excerpt.rs`
Book: https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html#the-static-lifetime

## Static vs const (ch 20.1)

A static has ONE fixed address for the whole program (a const is
inlined at every use) and may be computed at runtime. A mutable
static puts reads/writes behind unsafe -- the compiler cannot prove
single-threaded access -- and the deny-by-default static_mut_refs
forbids taking references, including println's invisible autoref:
read through *(&raw const COUNTER), binding the pointer first
(clippy's deref_addrof suggestion would trip static_mut_refs -- the
lints contradict). The checked replacement for counters: AtomicU32
(plain movs on x86).

Related: [Constants](#constants-ch-31), [Data race](#data-race-ch-42), [Static lifetime](#static-lifetime-ch-103), [unsafe Rust](#unsafe-rust-ch-201)
In repo: `projects/advanced/src/unsafe_rust.rs` (demo_static)
Book: https://doc.rust-lang.org/stable/book/ch20-01-unsafe-rust.html#static-variables

## Static vs dynamic dispatch (ch 18.2)

How a method call resolves. Static: known at compile time -
generics monomorphize into concrete copies, zero runtime cost,
inlining possible. Dynamic: a vtable lookup at runtime - one code
path for many types, a small per-call cost, no inlining. The
gui.rs 18-6 contrast: homogeneous collections -> generics;
heterogeneous -> dyn.

Related: [Monomorphization](#monomorphization-ch-101), [Trait object](#trait-object-ch-182), [Zero-cost abstraction](#zero-cost-abstraction-ch-134)
In repo: `projects/oop/src/gui.rs` (the 18-6 comment)
Book: https://doc.rust-lang.org/stable/book/ch18-02-trait-objects.html#performing-dynamic-dispatch

## Stream (ch 17.4)

An async iterator: items becoming ready over time. poll_next ->
Poll<Option<Item>> -- Future's single Output crossed with
Iterator's sequence of Items. Not in std yet: the ecosystem's
shared definition; StreamExt (the Ext pattern) supplies next,
blanket-implemented for every Stream.

Related: [Future](#future-ch-171), [Iterator](#iterator-ch-132)
In repo: `projects/hello-async/src/streams.rs`
Book: https://doc.rust-lang.org/stable/book/ch17-04-streams.html

## Strong vs weak references (ch 15.6)

Strong links (Rc::clone) express ownership and keep the value
alive; weak links (Rc::downgrade -> Weak<T>) do neither --
weak_count is ignored when deciding to drop. Before use a weak
link must upgrade() to an Option<Rc<T>>: None means the value is
already gone, so weak links never dangle. The parent direction of
a tree is the canonical weak link -- it breaks the parent-child
cycle.

Related: [Memory leak](#memory-leak-ch-156), [Rc and reference counting](#rc-and-reference-counting-ch-154), [Smart pointers](#smart-pointers-ch-150)
In repo: `projects/smart-pointers/src/cycles.rs` (demo_weak_tree)
Book: https://doc.rust-lang.org/stable/book/ch15-06-reference-cycles.html#preventing-reference-cycles-using-weakt

## Supertrait (ch 20.2)

A requirement ON a trait: `trait OutlinePrint: fmt::Display` works
only for types that also implement Display - so the trait's default
method may call self.to_string(). A trait bound on a trait; the
compiler enforces it at the impl site (E0277 for Display-less
types). The payoff: default methods may lean on capabilities they
cannot provide themselves.

Related: [Trait](#trait-ch-102), [Trait bound](#trait-bound-ch-101)
In repo: `projects/advanced/src/traits.rs` (demo_supertrait)
Book: https://doc.rust-lang.org/stable/book/ch20-02-advanced-traits.html#using-supertraits

## Test double and mock objects (ch 15.5)

A test double stands in for a real type during tests; a mock
object is the kind that RECORDS what happens, so the test can
assert on the calls. The book's Messenger mock must push into a
Vec behind &self -- interior mutability resolves the contradiction
without changing the production trait for testing's sake.

Related: [Interior mutability](#interior-mutability-ch-155), [Unit tests](#unit-tests-ch-113)
In repo: `projects/smart-pointers/src/messenger.rs`
Book: https://doc.rust-lang.org/stable/book/ch15-05-interior-mutability.html#testing-with-mock-objects

## Trait (ch 10.2)

A nominal declaration of shared behavior: method signatures that an
implementing type must provide via an explicit `impl Trait for Type`
(unlike structural TS interfaces). Defaults allowed; impls governed
by coherence and the orphan rule.

Related: [Trait bound](#trait-bound-ch-101), [Coherence](#coherence-ch-102), [Trait must be in scope](#trait-must-be-in-scope-ch-92), [Opaque type, impl Trait](#opaque-type-impl-trait-ch-102), [unsafe trait](#unsafe-trait-ch-201), [Associated type](#associated-type-ch-202), [Fully qualified syntax](#fully-qualified-syntax-ch-202), [Operator overloading](#operator-overloading-ch-202), [Supertrait](#supertrait-ch-202)
In repo: `projects/aggregator/src/lib.rs`
Book: https://doc.rust-lang.org/stable/book/ch10-02-traits.html#defining-a-trait

## Trait bound (ch 10.1)

A constraint on a generic parameter -- `T: Summary`: the code works
only with types having that behavior. `&impl Trait` is sugar for it;
several bounds combine with `+`; `where` moves them out of the
signature for readability.

Related: [Blanket implementations](#blanket-implementations-ch-102), [Duck typing](#duck-typing-ch-182), [Fn traits](#fn-traits-ch-131), [Monomorphization](#monomorphization-ch-101), [Supertrait](#supertrait-ch-202), [Trait](#trait-ch-102)
In repo: `projects/aggregator/src/lib.rs` (the notify family)
Book: https://doc.rust-lang.org/stable/book/ch10-02-traits.html#trait-bound-syntax

## Trait must be in scope (ch 9.2)

Calling a trait method requires the trait itself to be imported:
`read_to_string` only exists on `File` with `use std::io::Read` in
scope. Inherent methods (like `File::open`) need no import.

Related: [Fully qualified syntax](#fully-qualified-syntax-ch-202), [Monomorphization](#monomorphization-ch-101), [Trait](#trait-ch-102)
In repo: `projects/error-handling/src/error_propagation.rs` (module header note)
Book: https://doc.rust-lang.org/stable/book/ch10-02-traits.html#implementing-a-trait-on-a-type

## Trait object (ch 18.2)

A pointer (Box<dyn Trait>, &dyn Trait) to a value plus a table of
its trait methods, filled at runtime. Abstraction over shared
BEHAVIOR only - no fields can be added through it. Enables
heterogeneous collections (Vec<Box<dyn Draw>>) and open
extension: the library never knew SelectBox.

Related: [DST, dynamically sized type](#dst-dynamically-sized-type-ch-203), [Dyn compatibility](#dyn-compatibility-ch-182), [Duck typing](#duck-typing-ch-182), [Smart pointers](#smart-pointers-ch-150), [State pattern](#state-pattern-ch-183), [Static vs dynamic dispatch](#static-vs-dynamic-dispatch-ch-182)
In repo: `projects/oop/src/gui.rs` (with select_box.rs as the consumer)
Book: https://doc.rust-lang.org/stable/book/ch18-02-trait-objects.html

## Type alias (ch 20.3)

`type Name = Existing;` - a SYNONYM, not a new type: Kilometers
and i32 mix freely, zero compile-time protection (the exact
counter-example to the newtype pattern). The use case is
repetition: Thunk for Box<dyn Fn() + Send + 'static>, and
std::io::Result<T> - a PARTIALLY APPLIED generic
(std::result::Result<T, std::io::Error>) that keeps the whole
API: ? and every Result method intact.

Related: [Fn traits](#fn-traits-ch-131), [Newtype pattern](#newtype-pattern-ch-102)
In repo: `projects/advanced/src/types.rs` (demo_type_aliases)
Book: https://doc.rust-lang.org/stable/book/ch20-03-advanced-types.html#type-synonyms-and-type-aliases

## Undefined behavior (ch 20.1)

Code the compiler owes nothing to: memory corruption, security
holes, anything at all. NOT the same as a panic -- a panic is a
safe, controlled unwinding; UB is the absence of guarantees. unsafe
does not cause UB; unsound unsafe does (an invariant not actually
upheld). Converting potential UB into a panic (assert! before the
unsafe block in split_at_mut) is a strict safety upgrade.

Related: [Data race](#data-race-ch-42), [Miri](#miri-ch-201), [unsafe Rust](#unsafe-rust-ch-201), [Unwinding vs abort](#unwinding-vs-abort-ch-91)
In repo: `exercises/20_miri`
Book: https://doc.rust-lang.org/stable/book/ch20-01-unsafe-rust.html#unsafe-superpowers

## Union (ch 20.1)

Like a struct, but exactly ONE field is valid at a time -- which
one is on the programmer. Reading a union field is unsafe (the
memory may hold garbage of another branch); writing is safe. A C
interoperability niche (overlapping C unions); no listing in the
book.

Related: [FFI, foreign function interface](#ffi-foreign-function-interface-ch-201), [unsafe Rust](#unsafe-rust-ch-201)
In repo: `projects/advanced/src/unsafe_rust.rs` (the closing comment)
Book: https://doc.rust-lang.org/stable/book/ch20-01-unsafe-rust.html

## Unit tests (ch 11.3)

Small, focused tests in src/ beside the code they test (the
#[cfg(test)] mod tests convention): one module in isolation, and
CAN test private interfaces -- the tests module is a child, and
children see their ancestors' private items.

Related: [Integration tests](#integration-tests-ch-113), [Test double and mock objects](#test-double-and-mock-objects-ch-155)
In repo: `projects/testing/src/*` (the tests module in each)
Book: https://doc.rust-lang.org/stable/book/ch11-03-test-organization.html#unit-tests

## unsafe Rust (ch 20.1)

The unsafe keyword unlocks exactly five superpowers: dereference a
raw pointer, call an unsafe fn, access a mutable static, implement
an unsafe trait, access union fields. It does NOT turn off the
borrow checker. An unsafe fn shifts the proof to the caller; a safe
fn may keep unsafe as an implementation detail (split_at_mut stays
a safe API). The litmus test for a signature: can a caller writing
only safe code cause UB with valid arguments? yes -> unsafe fn.

Related: [Borrow checker](#borrow-checker-ch-103), [FFI, foreign function interface](#ffi-foreign-function-interface-ch-201), [Miri](#miri-ch-201), [Raw pointer](#raw-pointer-ch-201), [Rustonomicon](#rustonomicon-ch-201), [SAFETY comment](#safety-comment-ch-201), [Static vs const](#static-vs-const-ch-201), [Undefined behavior](#undefined-behavior-ch-201), [Union](#union-ch-201), [unsafe trait](#unsafe-trait-ch-201)
In repo: `projects/advanced/src/unsafe_rust.rs` (the whole module)
Book: https://doc.rust-lang.org/stable/book/ch20-01-unsafe-rust.html#unsafe-superpowers

## unsafe trait (ch 20.1)

A trait whose invariants the compiler cannot check; every impl must
sign with `unsafe impl` -- the promise lives at the impl, while
CALLING the trait's methods stays safe. The everyday case: manual
Send for a raw-pointer wrapper (E0277 without the impl). The
contract-bearing case: AnyBits ("every bit pattern is a valid
value") -- a safe reinterpret standing on it turns UB-producing if
an impl lies (bool's 0/1 niche). Doc # Safety sections required.

Related: [Send and Sync](#send-and-sync-ch-164), [Trait](#trait-ch-102), [unsafe Rust](#unsafe-rust-ch-201)
In repo: `projects/advanced/src/unsafe_rust.rs` (demo_unsafe_trait)
Book: https://doc.rust-lang.org/stable/book/ch20-01-unsafe-rust.html#implementing-an-unsafe-trait

## Unwinding vs abort (ch 9.1)

Default panic behavior: unwind -- walk back up the stack, running
destructors. panic = 'abort' in a [profile] skips cleanup: smaller
binaries, but cleanup and thread isolation (catch_unwind) are gone.

Related: [panic vs Result guidelines](#panic-vs-result-guidelines-ch-93), [Release profiles](#release-profiles-ch-141), [Undefined behavior](#undefined-behavior-ch-201)
In repo: `projects/panic/Cargo.toml` (the release-profile comment)
Book: https://doc.rust-lang.org/stable/book/ch09-01-unrecoverable-errors-with-panic.html#unwinding-the-stack-or-aborting-in-response-to-a-panic

## Workspace (ch 14.3)

A set of packages developed in tandem, sharing one Cargo.lock and
one target/ directory at the workspace root. The root Cargo.toml
has a [workspace] section (no [package]); membership is listed in
members, but dependencies between members are still explicit path
dependencies. One shared lockfile pins every member to the same
dependency versions; -p selects a package for run/test/publish,
and each crate is published separately.

Related: [Crate](#crate-ch-71), [Package](#package-ch-71), [Resolver versions](#resolver-versions-ch-143)
In repo: `projects/add` (the virtual workspace root)
Book: https://doc.rust-lang.org/stable/book/ch14-03-cargo-workspaces.html

## Yank (ch 14.2)

cargo yank --vers X.Y.Z: marks a published version so that NEW
projects cannot depend on it, while existing Cargo.lock files keep
working; --undo reverses it. Versions on crates.io are permanent --
a yank is the only lever, and it deletes nothing (not even
accidentally uploaded secrets: rotate those immediately).

Related: —
In repo: —
Book: https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html#deprecating-versions-from-cratesio

## Zero-cost abstraction (ch 13.4)

An abstraction that imposes no runtime overhead over hand-written
lower-level code: iterators compile to roughly the same assembly
as explicit loops (unrolling, bounds-check elimination). The
book's evidence: for-loop vs iterator search on a large text --
~19.6M vs ~19.2M ns, parity. Stroustrup's phrasing: "What you
don't use, you don't pay for. And what you do use, you couldn't
hand code any better."

Related: [Iterator](#iterator-ch-132), [Monomorphization](#monomorphization-ch-101), [Static vs dynamic dispatch](#static-vs-dynamic-dispatch-ch-182)
In repo: `projects/minigrep` (13.3 rewrite as the case study)
