# Chapter 14: Publishing to a Registry — Exercises

All stages run against a local registry (cargo-http-registry), so the
crates.io namespace stays untouched and everything is disposable. Note:
`art` is taken on crates.io (an "adaptive radix trie" since 2018), but a
local registry has its own namespace -- the name is free there.

Work on a COPY of the crate (e.g. cp -r projects/art /tmp/art): cargo
refuses to package a dirty git tree, and the repo stays clean.

1. Start a local registry: `cargo install cargo-http-registry` (a live
   demo of ch 14.4 -- the binary lands in ~/.cargo/bin), then
   `cargo-http-registry /tmp/registry`. Read the port from
   /tmp/registry/config.json and register it in ~/.cargo/config.toml:
   `[registries.local] index = "sparse+http://127.0.0.1:PORT/index"`.
2. In the copy, replace `publish = false` with the allow-list
   `publish = ["local"]`, then `cargo login --registry local` (any
   string works as the token) and `cargo publish --registry local`:
   pack -> verify -> upload, for real this time.
3. Consume it: create a scratch binary crate, depend on the published
   crate with an explicit `registry = "local"` in [dependencies], and
   call `mix` through the re-exported API.
4. Practice versioning: republish 0.1.0 unchanged (read the error),
   bump to 0.1.1 and republish, then to 0.2.0 -- a minor bump for a
   doc-comment change is a semver judgment call; justify it.
5. beyond the book: try `cargo yank --registry local --vers 0.1.1` --
   the minimal registry may not implement it; compare the outcome with
   the book's description of yanking on crates.io.
6. beyond the book: with the allow-list in place, run plain
   `cargo publish` and observe the guard message naming the allowed
   registries.

Source: https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html
The registry mechanics are beyond the book:
https://doc.rust-lang.org/cargo/reference/registries.html
