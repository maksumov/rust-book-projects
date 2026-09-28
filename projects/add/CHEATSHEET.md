# cargo workspace cheatsheet (chapter 14.3)

All outputs below are examples captured from this workspace (add).
The single-package workflow (doc/package/publish) lives in
projects/art/CHEATSHEET.md.

## Anatomy

```toml
# add/Cargo.toml -- a VIRTUAL manifest: [workspace] without [package].
# It owns the shared Cargo.lock and target/ but is not a crate itself,
# and has no edition -- which is why the resolver version must be set
# explicitly (a virtual manifest cannot infer it; the legacy default
# would be "1").
[workspace]
resolver = "3"
# cargo new inside a workspace appends the new package here by itself.
members = ["add_one", "adder"]
```

Members are regular packages (edition 2024). Inside a member,
dependencies on siblings are still explicit path dependencies:

```toml
# adder/Cargo.toml
[dependencies]
add_one = { path = "../add_one" }
```

## Shared artifacts

One lockfile, one build directory -- both at the workspace root:

```sh
grep -c 'name = "rand"' Cargo.lock   # -> 1: both members share the entry
find . -maxdepth 2 -name target -not -path "./target" | wc -l   # -> 0
```

rand is declared by BOTH members but recorded (and downloaded,
compiled) once. If members require incompatible versions of the same
dependency, cargo resolves as few versions as possible.

## Selecting packages

```sh
cargo run -p adder
```

```
Hello, world! 10 plus one is 11!
Lucky number: 62
```

Workspace-wide `cargo test` runs every member's suite in its own
section (then doc-tests); `-p` narrows it to one member -- adder's
section disappears entirely:

```sh
cargo test -p add_one
```

```
     Running unittests src/lib.rs (target/debug/deps/add_one-...)
running 2 tests
test result: ok. 2 passed; ...
running 0 tests
test result: ok. 0 passed; ...   # doc-tests of add_one only
```

Publishing works the same way: each member is published separately,
`cargo publish -p <name>`.

## Membership is NOT a dependency (E0432)

Removing `rand` from adder/Cargo.toml (keeping the code that uses it)
fails the build -- being in the same workspace does not link anything:

```
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:7:9
  |
7 |     use rand::Rng;
  |         ^^^^ use of unresolved module or unlinked crate `rand`
  |
  = help: if you wanted to use a crate named `rand`, use `cargo add rand` to add it to your `Cargo.toml`

error[E0433]: cannot find module or crate `rand` in this scope
 --> adder/src/main.rs:8:24
  |
8 |     let jackpot: i32 = rand::thread_rng().gen_range(1..=100);
  |                        ^^^^ use of unresolved module or unlinked crate `rand`
```

## Beyond the book (pointers)

- `[workspace.dependencies]` in the root + `rand = { workspace = true }`
  in members: declare a dependency once for the whole workspace --
  exercise 3 in exercises/14_workspace.
- Running `cargo build` from inside a member still writes to the
  shared root target/ (exercise 4).
