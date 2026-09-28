# Chapter 14: Cargo Workspaces — Exercises

1. From the book: add an `add_two` library crate to `projects/add` the
   same way `add_one` was created (`cargo new add_two --lib`), make
   `adder` depend on it, and print "plus two" alongside "plus one".
2. Run the tests of a single member (`cargo test -p add_two`) and
   compare the output sections with the workspace-wide `cargo test`.
3. beyond the book: hoist `rand` to `[workspace.dependencies]` in the
   root and inherit it in both members via `rand = { workspace = true }`
   -- the real-world deduplication pattern the book does not cover.
4. beyond the book: run `cargo build` from inside `adder/` and confirm
   the artifacts still land in the shared `add/target`.

Source: https://doc.rust-lang.org/stable/book/ch14-03-cargo-workspaces.html

From the book:

> For additional practice, add an `add_two` crate to this workspace in a
> similar way as the `add_one` crate!
