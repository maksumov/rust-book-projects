# Glossary

Terms and concepts from The Rust Book, collected while studying.
Each entry: a definition, connections to related entries, where the
concept lives in this repo (if anywhere), and the book section it
came from. Entries are alphabetical; the chapter tag shows where
the term first appeared in the study flow.

## Table of Contents

- [Borrow-based lookup (ch 8.3)](#borrow-based-lookup-ch-83)
- [Blanket implementations (ch 10.2)](#blanket-implementations-ch-102)
- [Coherence (ch 10.2)](#coherence-ch-102)
- [Combinators (ch 9.2)](#combinators-ch-92)
- [Deref coercion (ch 4.2)](#deref-coercion-ch-42)
- [Inner vs outer attributes (ch 9.2)](#inner-vs-outer-attributes-ch-92)
- [Monomorphization (ch 10.1)](#monomorphization-ch-101)
- [Newtype pattern (ch 10.2)](#newtype-pattern-ch-102)
- [NLL, non-lexical lifetimes (ch 4.2)](#nll-non-lexical-lifetimes-ch-42)
- [Opaque type, impl Trait (ch 10.2)](#opaque-type-impl-trait-ch-102)
- [Orphan rule (ch 10.2)](#orphan-rule-ch-102)
- [panic vs Result guidelines (ch 9.3)](#panic-vs-result-guidelines-ch-93)
- [SipHash and BuildHasher (ch 8.3)](#siphash-and-buildhasher-ch-83)
- [Trait (ch 10.2)](#trait-ch-102)
- [Trait bound (ch 10.1)](#trait-bound-ch-101)
- [Trait must be in scope (ch 9.2)](#trait-must-be-in-scope-ch-92)

---

## Borrow-based lookup (ch 8.3)

`HashMap::get` is generic over `Q` where `K: Borrow<Q>`; since
`String: Borrow<str>`, a map with `String` keys can be looked up
by a plain `&str` -- no allocation, unlike building a `String` key
per lookup. Distinct from deref coercion: this is `Borrow`-based
type equality at the API level.

Related: [Deref coercion](#deref-coercion-ch-42)
In repo: `projects/collections/src/demos/hashmaps.rs` (comment inside the lookup loop)
Book: https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#accessing-values-in-a-hash-map

## Blanket implementations (ch 10.2)

Implementing a trait for EVERY type satisfying a bound:
`impl<T: Display> ToString for T`. Powers the everyday `.to_string()`
on any Display type. Listed in the trait docs "Implementors" section.

Related: [Trait bound](#trait-bound-ch-101)
In repo: —
Book: https://doc.rust-lang.org/stable/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods

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

Related: [panic vs Result guidelines](#panic-vs-result-guidelines-ch-93)
In repo: `projects/error-handling/src/main.rs` (note above the demo pair)
Book: https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html#alternatives-to-using-match-with-resultt-e

## Deref coercion (ch 4.2)

The compiler converts `&String` to `&str` (and similar reference
conversions) at call sites automatically -- why `s1 + &s2` works
when `add` actually takes `&str`. Covered in depth in chapter 15.

Related: [Borrow-based lookup](#borrow-based-lookup-ch-83)
In repo: `projects/collections/src/demos/strings.rs` (concatenation comment)
Book: https://doc.rust-lang.org/stable/book/ch08-02-strings.html#concatenating-with--or-format

## Inner vs outer attributes (ch 9.2)

`#[...]` is an OUTER attribute: it attaches to the next item (e.g.
a single `mod` line). `#![...]` is an INNER attribute: it applies
to the enclosing item -- in the crate root, to the whole crate.
The `!` here means "inner", not a macro invocation.

Related: —
In repo: `projects/error-handling/src/main.rs` (commented-out `#![allow(dead_code)]`)
Book: https://doc.rust-lang.org/reference/attributes.html

## Monomorphization (ch 10.1)

How Rust compiles generics: the compiler generates a concrete copy
of the generic code for every type it is used with -- static
dispatch, zero runtime cost, larger binaries and longer compile
times. The opposite strategy is type erasure (TypeScript, Java):
one copy, dynamic dispatch. The practical trade-off returns in
chapter 18 as `impl Trait` vs `dyn Trait`.

Related: [Opaque type, impl Trait](#opaque-type-impl-trait-ch-102), [Trait bound](#trait-bound-ch-101), [Trait must be in scope](#trait-must-be-in-scope-ch-92)
In repo: —
Book: https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#performance-of-code-using-generics

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

Related: —
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

## panic vs Result guidelines (ch 9.3)

panic! (and unwrap/expect) is for failures that mean a bug -- a
broken invariant -- and is fine in examples, prototypes and tests.
Result is for expected failures (missing files, bad input) where
the caller may recover; propagate with `?`.

Related: [Combinators](#combinators-ch-92)
In repo: anchors in `projects/error-handling/src/main.rs` and `projects/panic/src/main.rs`
Book: https://doc.rust-lang.org/stable/book/ch09-03-to-panic-or-not-to-panic.html

## SipHash and BuildHasher (ch 8.3)

The default `HashMap` hasher is SipHash: DoS-resistant, trading
some speed for security. A faster/slower hasher can be plugged in
via a type implementing the `BuildHasher` trait.

Related: —
In repo: `projects/collections/src/demos/hashmaps.rs` (comment above the use line)
Book: https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#hashing-functions

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

Related: [Trait](#trait-ch-102), [Monomorphization](#monomorphization-ch-101), [Blanket implementations](#blanket-implementations-ch-102)
In repo: `projects/aggregator/src/lib.rs` (the notify family)
Book: https://doc.rust-lang.org/stable/book/ch10-02-traits.html#trait-bound-syntax

## Trait must be in scope (ch 9.2)

Calling a trait method requires the trait itself to be imported:
`read_to_string` only exists on `File` with `use std::io::Read` in
scope. Inherent methods (like `File::open`) need no import.

Related: [Monomorphization](#monomorphization-ch-101), [Trait](#trait-ch-102)
In repo: `projects/error-handling/src/error_propagation.rs` (module header note)
Book: https://doc.rust-lang.org/stable/book/ch10-02-traits.html#implementing-a-trait-on-a-type
