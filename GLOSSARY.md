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
- [Constants (ch 3.1)](#constants-ch-31)
- [Crate (ch 7.1)](#crate-ch-71)
- [Crate root (ch 7.1)](#crate-root-ch-71)
- [Custom Cargo subcommands (ch 14.5)](#custom-cargo-subcommands-ch-145)
- [Data race (ch 4.2)](#data-race-ch-42)
- [Deadlock (ch 16.3)](#deadlock-ch-163)
- [Deref coercion (ch 4.2)](#deref-coercion-ch-42)
- [Destructor (ch 15.3)](#destructor-ch-153)
- [Documentation comments (ch 14.2)](#documentation-comments-ch-142)
- [Fearless concurrency (ch 16.0)](#fearless-concurrency-ch-160)
- [Fn traits (ch 13.1)](#fn-traits-ch-131)
- [Indirection (ch 15.1)](#indirection-ch-151)
- [Inner vs outer attributes (ch 9.2)](#inner-vs-outer-attributes-ch-92)
- [Integration tests (ch 11.3)](#integration-tests-ch-113)
- [Interior mutability (ch 15.5)](#interior-mutability-ch-155)
- [Iterator (ch 13.2)](#iterator-ch-132)
- [Iterator adapters vs consuming adapters (ch 13.2)](#iterator-adapters-vs-consuming-adapters-ch-132)
- [Lifetime (ch 10.3)](#lifetime-ch-103)
- [Lifetime elision rules (ch 10.3)](#lifetime-elision-rules-ch-103)
- [Lint levels (ch 14.2)](#lint-levels-ch-142)
- [Memory leak (ch 15.6)](#memory-leak-ch-156)
- [Message passing vs shared state (ch 16.0)](#message-passing-vs-shared-state-ch-160)
- [Monomorphization (ch 10.1)](#monomorphization-ch-101)
- [Move (ch 4.1)](#move-ch-41)
- [move closures (ch 13.1)](#move-closures-ch-131)
- [Mutex and lock (ch 16.3)](#mutex-and-lock-ch-163)
- [Newtype pattern (ch 10.2)](#newtype-pattern-ch-102)
- [NLL, non-lexical lifetimes (ch 4.2)](#nll-non-lexical-lifetimes-ch-42)
- [Opaque type, impl Trait (ch 10.2)](#opaque-type-impl-trait-ch-102)
- [Orphan rule (ch 10.2)](#orphan-rule-ch-102)
- [Ownership (ch 4.1)](#ownership-ch-41)
- [Package (ch 7.1)](#package-ch-71)
- [panic vs Result guidelines (ch 9.3)](#panic-vs-result-guidelines-ch-93)
- [RAII (ch 4.1)](#raii-ch-41)
- [Rc and reference counting (ch 15.4)](#rc-and-reference-counting-ch-154)
- [Re-export (ch 14.2)](#re-export-ch-142)
- [Release profiles (ch 14.1)](#release-profiles-ch-141)
- [Resolver versions (ch 14.3)](#resolver-versions-ch-143)
- [Send and Sync (ch 16.4)](#send-and-sync-ch-164)
- [Shadowing (ch 3.1)](#shadowing-ch-31)
- [SipHash and BuildHasher (ch 8.3)](#siphash-and-buildhasher-ch-83)
- [Slice (ch 4.3)](#slice-ch-43)
- [Smart pointers (ch 15.0)](#smart-pointers-ch-150)
- [Standard output vs standard error (ch 12.6)](#standard-output-vs-standard-error-ch-126)
- [Static lifetime (ch 10.3)](#static-lifetime-ch-103)
- [Strong vs weak references (ch 15.6)](#strong-vs-weak-references-ch-156)
- [Test double and mock objects (ch 15.5)](#test-double-and-mock-objects-ch-155)
- [Trait (ch 10.2)](#trait-ch-102)
- [Trait bound (ch 10.1)](#trait-bound-ch-101)
- [Trait must be in scope (ch 9.2)](#trait-must-be-in-scope-ch-92)
- [Unit tests (ch 11.3)](#unit-tests-ch-113)
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

Related: [Borrowing](#borrowing-ch-42), [Data race](#data-race-ch-42), [Lifetime](#lifetime-ch-103), [NLL, non-lexical lifetimes](#nll-non-lexical-lifetimes-ch-42)
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

## Constants (ch 3.1)

`const NAME: Type = expr;` -- always immutable, the type annotation
is required, the value must be computable at compile time; valid
for the whole program within its scope. SCREAMING_SNAKE naming.

Related: —
In repo: —
Book: https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html#declaring-constants

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

Related: [Borrow checker](#borrow-checker-ch-103), [Borrowing](#borrowing-ch-42), [Fearless concurrency](#fearless-concurrency-ch-160)
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

## Documentation comments (ch 14.2)

The /// and //! comment styles that generate HTML documentation
via rustdoc (cargo doc --open): /// documents the item AFTER it,
//! the containing item (crate or module). Markdown supported; the
# Examples blocks are compiled and run by cargo test as doctests
-- the third test section -- keeping docs and code in sync. Common
sections: # Examples, # Panics, # Errors, # Safety.

Related: [Crate root](#crate-root-ch-71), [Lint levels](#lint-levels-ch-142)
In repo: `projects/art/src/lib.rs` (/// and //! incl. Examples/Errors)
Book: https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments

## Fearless concurrency (ch 16.0)

The chapter's namesake: ownership and type checking turn many
concurrency errors into COMPILE-TIME errors -- a wrong program
refuses to build instead of racing in production. The tools are
mostly standard-library types (threads, channels, Mutex, Arc)
gated by two language-level marker traits, Send and Sync.

Related: [Data race](#data-race-ch-42), [Send and Sync](#send-and-sync-ch-164)
In repo: `projects/concurrency` (the whole chapter)
Book: https://doc.rust-lang.org/stable/book/ch16-00-concurrency.html

## Fn traits (ch 13.1)

The additive closure hierarchy picked by API bounds: `FnOnce`
(may move captures out -- callable once), `FnMut` (may mutate --
repeatable), `Fn` (read-only). Every closure implements all it
can; bounds choose the weakest they need (e.g. `unwrap_or_else`
takes `FnOnce`, `sort_by_key` takes `FnMut`).

Related: [Closure](#closure-ch-131), [Trait bound](#trait-bound-ch-101)
In repo: `projects/closures/src/fn_traits.rs`

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

Related: [Closure](#closure-ch-131), [Iterator adapters vs consuming adapters](#iterator-adapters-vs-consuming-adapters-ch-132), [Combinators](#combinators-ch-92), [Zero-cost abstraction](#zero-cost-abstraction-ch-134)
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

Related: [Documentation comments](#documentation-comments-ch-142), [Inner vs outer attributes](#inner-vs-outer-attributes-ch-92)
In repo: `projects/art/src/lib.rs` (the crate-root attribute)
Book: https://doc.rust-lang.org/rustc/lints/levels.html

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

## Monomorphization (ch 10.1)

How Rust compiles generics: the compiler generates a concrete copy
of the generic code for every type it is used with -- static
dispatch, zero runtime cost, larger binaries and longer compile
times. The opposite strategy is type erasure (TypeScript, Java):
one copy, dynamic dispatch. The practical trade-off returns in
chapter 18 as `impl Trait` vs `dyn Trait`.

Related: [Opaque type, impl Trait](#opaque-type-impl-trait-ch-102), [Trait bound](#trait-bound-ch-101), [Trait must be in scope](#trait-must-be-in-scope-ch-92), [Zero-cost abstraction](#zero-cost-abstraction-ch-134)
In repo: —
Book: https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#performance-of-code-using-generics

## Move (ch 4.1)

Assignment that invalidates the source: the stack part (pointer,
length, capacity) is copied, the heap data is not, and the source
variable dies. Not a shallow copy -- precisely because of the
invalidation; this prevents double free. Rust never deep-copies
implicitly (explicit deep copy: clone).

Related: [Borrowing](#borrowing-ch-42), [Channel, mpsc](#channel-mpsc-ch-162), [Ownership](#ownership-ch-41), [move closures](#move-closures-ch-131)
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

## Newtype pattern (ch 10.2)

Wrap a foreign type in a local tuple struct (`struct Meters(f64)`)
to gain the right to implement traits for it under the orphan rule.
Also serves nominal branding (meters vs feet), like branded types
in TypeScript. Covered in depth in chapter 19.

Related: [Coherence](#coherence-ch-102), [Orphan rule](#orphan-rule-ch-102)
In repo: —
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
need Box<dyn Trait> (ch 18).

Related: [Trait](#trait-ch-102), [Monomorphization](#monomorphization-ch-101)
In repo: `projects/aggregator/src/main.rs` (the return summarizable comment)
Book: https://doc.rust-lang.org/stable/book/ch10-02-traits.html#returning-types-that-implement-traits

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

Related: [Backtrace](#backtrace-ch-91), [Combinators](#combinators-ch-92), [Unwinding vs abort](#unwinding-vs-abort-ch-91)
In repo: anchors in `projects/error-handling/src/main.rs` and `projects/panic/src/main.rs`
Book: https://doc.rust-lang.org/stable/book/ch09-03-to-panic-or-not-to-panic.html

## RAII (ch 4.1)

Resource Acquisition Is Initialization (a C++ term): resources are
released at the end of an item's lifetime. Rust's drop-at-scope-end
is this pattern -- the book notes the kinship explicitly.

Related: [Destructor](#destructor-ch-153), [Ownership](#ownership-ch-41)
In repo: —
Book: https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html

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

Related: [Crate root](#crate-root-ch-71)
In repo: `projects/art/src/lib.rs` (the collapsed pub use block)
Book: https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api

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

## Send and Sync (ch 16.4)

The two language-level marker traits (no methods) behind fearless
concurrency. Send: ownership of the type may be TRANSFERRED between
threads (Rc is not Send -- its count is not thread-safe; Arc is).
Sync: &T is safe to share across threads (&T: Send); RefCell/Cell
are not Sync, Mutex is. Compositions inherit automatically; manual
impls are unsafe (ch 20). The E0277 of the Rc counter attempt is
these traits talking.

Related: [Arc, atomic reference counting](#arc-atomic-reference-counting-ch-163), [Fearless concurrency](#fearless-concurrency-ch-160)
In repo: `projects/concurrency/src/shared_state.rs` (the 16-14 comment)
Book: https://doc.rust-lang.org/stable/book/ch16-04-extensible-concurrency-sync-and-send.html

## Shadowing (ch 3.1)

`let` with an already-used name creates a NEW variable that
overshadows the old one until it is itself shadowed or the scope
ends. Unlike `mut`: the type may change across shadowings, and the
variable stays immutable between them.

Related: —
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

Related: [Borrowing](#borrowing-ch-42), [Deref coercion](#deref-coercion-ch-42)
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

Related: [Deref coercion](#deref-coercion-ch-42), [Destructor](#destructor-ch-153), [Indirection](#indirection-ch-151), [Rc and reference counting](#rc-and-reference-counting-ch-154), [Strong vs weak references](#strong-vs-weak-references-ch-156)
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

## Static lifetime (ch 10.3)

`'static`: the reference is valid for the entire program; every
string literal is 'static (stored in the binary). An error message
suggesting 'static is usually a dangling-reference smell -- fix
the lifetimes instead of reaching for it.

Related: [Lifetime](#lifetime-ch-103), [move closures](#move-closures-ch-131)
In repo: `projects/lifetimes/src/excerpt.rs`
Book: https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html#the-static-lifetime

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

Related: [Trait bound](#trait-bound-ch-101), [Coherence](#coherence-ch-102), [Trait must be in scope](#trait-must-be-in-scope-ch-92), [Opaque type, impl Trait](#opaque-type-impl-trait-ch-102)
In repo: `projects/aggregator/src/lib.rs`
Book: https://doc.rust-lang.org/stable/book/ch10-02-traits.html#defining-a-trait

## Trait bound (ch 10.1)

A constraint on a generic parameter -- `T: Summary`: the code works
only with types having that behavior. `&impl Trait` is sugar for it;
several bounds combine with `+`; `where` moves them out of the
signature for readability.

Related: [Blanket implementations](#blanket-implementations-ch-102), [Fn traits](#fn-traits-ch-131), [Monomorphization](#monomorphization-ch-101), [Trait](#trait-ch-102)
In repo: `projects/aggregator/src/lib.rs` (the notify family)
Book: https://doc.rust-lang.org/stable/book/ch10-02-traits.html#trait-bound-syntax

## Trait must be in scope (ch 9.2)

Calling a trait method requires the trait itself to be imported:
`read_to_string` only exists on `File` with `use std::io::Read` in
scope. Inherent methods (like `File::open`) need no import.

Related: [Monomorphization](#monomorphization-ch-101), [Trait](#trait-ch-102)
In repo: `projects/error-handling/src/error_propagation.rs` (module header note)
Book: https://doc.rust-lang.org/stable/book/ch10-02-traits.html#implementing-a-trait-on-a-type

## Unit tests (ch 11.3)

Small, focused tests in src/ beside the code they test (the
#[cfg(test)] mod tests convention): one module in isolation, and
CAN test private interfaces -- the tests module is a child, and
children see their ancestors' private items.

Related: [Integration tests](#integration-tests-ch-113), [Test double and mock objects](#test-double-and-mock-objects-ch-155)
In repo: `projects/testing/src/*` (the tests module in each)
Book: https://doc.rust-lang.org/stable/book/ch11-03-test-organization.html#unit-tests

## Unwinding vs abort (ch 9.1)

Default panic behavior: unwind -- walk back up the stack, running
destructors. panic = 'abort' in a [profile] skips cleanup: smaller
binaries, but cleanup and thread isolation (catch_unwind) are gone.

Related: [panic vs Result guidelines](#panic-vs-result-guidelines-ch-93), [Release profiles](#release-profiles-ch-141)
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

Related: [Monomorphization](#monomorphization-ch-101), [Iterator](#iterator-ch-132)
In repo: `projects/minigrep` (13.3 rewrite as the case study)
