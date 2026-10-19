# Chapter 20: Exploring the Derive Macro's AST — Exercises

The derive macro in `projects/macros` receives the annotated code
as a parsed syntax tree (`syn::DeriveInput`). These exercises make
that tree observable, input side and output side.

1. Input side, the full tree: `cargo add syn --features
   extra-traits --package hello_macro_derive` — syn types
   implement `Debug` only under this feature (kept out of
   production builds by default). Add `dbg!(&ast);` after the
   `syn::parse` call in `hello_macro_derive` and build `pancakes`:
   the tree lands in the BUILD log — proc macros run at compile
   time. Compare with the book's 20-41 sketch (ident "Pancakes").
2. Targeted views: `dbg!(&ast.ident)`, `dbg!(&ast.data)`,
   `dbg!(&ast.generics)` — which fields describe enums, generics,
   visibility? Cross-check with the `syn` docs for `DeriveInput`.
3. The caching gotcha: cargo does not re-run the macro unless the
   CONSUMER code changes — touch `pancakes/src/main.rs` (or edit
   it) to see the dbg! output again.
4. Output side: `cargo install cargo-expand` (needs nightly), then
   `cargo expand --package pancakes` — the generated
   `impl HelloMacro for Pancakes` shown expanded in place.
5. Restore the clean state afterwards: the `extra-traits` feature
   and `dbg!` are study scaffolding, not project code.

Source: https://doc.rust-lang.org/stable/book/ch20-05-macros.html

Beyond the book (the chapter shows the 20-41 sketch; these
experiments make it live).
