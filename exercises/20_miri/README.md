# Chapter 20: Miri — Exercises

1. Setup: `rustup +nightly component add miri` (accept the `rust-src`
   prompt — Miri builds its own std from source on the first run; a
   couple of minutes, then cached).
2. Green run: `cargo +nightly miri run` in projects/advanced — all the
   sound unsafe passes silently (raw pointers to a live local,
   split_at_mut with its asserted invariant, the statics). The first
   lesson: Miri says nothing about sound code.
3. The book's listing 20-7, live: a scratch bin with
   from_raw_parts_mut over an arbitrary address (0x01234). Read the
   diagnostic — what exactly is flagged? (The comment in
   unsafe_rust.rs promised "dangling" — verify the wording.)
4. Use-after-free: `Box::into_raw`, drop via `Box::from_raw`, then read
   through the stale raw pointer.
5. Out of bounds: a slice's `as_mut_ptr`, `ptr.add(len + 1)`, read.
6. beyond the book — aliasing rules: `&raw mut x`, then take `&x`, use
   both. rustc compiles it happily; Miri explains (Stacked Borrows: the
   shared ref invalidated the raw pointer). This is WHY `&raw` exists —
   raw pointers have rules too.
7. beyond the book — a data race: two scoped threads incrementing
   through `&raw mut COUNTER`. Unsafe code allows the race; Miri still
   detects it — ch 16's guarantees, seen from the other side.
8. FFI limitation (already met in advanced): the minimal extern "C"
   call aborts with `unsupported operation: can't call foreign
   function` — and Miri's own note: "it does not indicate a bug in the
   program". Explain why an interpreter cannot run native code, and
   which libc functions it still emulates with built-in shims — then
   study how advanced stays miri-clean: `#[cfg(miri)]` stub demos.
   Optional: compare with `-Zmiri-tree-borrows`, the alternative
   aliasing model.

Source: https://doc.rust-lang.org/stable/book/ch20-01-unsafe-rust.html
