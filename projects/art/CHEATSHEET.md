# cargo package cheatsheet (chapter 14)

All outputs below are examples captured from this project (art,
listings 14-3..14-6 plus the beyond-the-book extensions).

## Documentation (ch 14.2)

```sh
cargo doc --open   # rustdoc HTML -> target/doc, opens in browser
```

- `///` documents the item after it; `//!` documents the containing
  item (crate or module) -- both support Markdown.
- `# Examples` blocks are compiled and RUN by `cargo test` as
  doctests -- the third test section (after unit and integration).
  Other conventional sections: `# Panics`, `# Errors`, `# Safety`.
- Re-exports (`pub use`) show up on the crate front page.

```sh
cargo test -q 2>&1 | tail -4
```

```
running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

all doctests ran in 0.12s; merged doctests compilation took 0.12s
```

## Packaging without publishing (ch 14.2 + beyond the book)

Note: `cargo package`/`cargo publish` refuse a dirty git tree
("files ... not yet checked into git") -- capture from a clean
tree, or pass `--allow-dirty`.

### cargo package --list: what goes into the .crate

In a git repository the .crate ships ALL git-tracked files of the
package directory by default (hence .cargo_vcs_info.json -- cargo's
embedded git info). This is also why cargo refuses a dirty tree:
the package must match a committed state. The list below evolved
with this project (all outputs real):

Before any metadata -- the warning names everything missing:

```
warning: manifest has no description, license, license-file, documentation, homepage or repository
  |
  = note: see https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info
.cargo_vcs_info.json
Cargo.lock
Cargo.toml
Cargo.toml.orig
src/lib.rs
src/main.rs
```

After description/license were added the warning narrowed to the
remaining optional fields, and CHEATSHEET.md -- by then a tracked
file -- joined the list:

```
warning: manifest has no documentation, homepage or repository
  |
  = note: see https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info
.cargo_vcs_info.json
CHEATSHEET.md
Cargo.lock
Cargo.toml
Cargo.toml.orig
src/lib.rs
src/main.rs
```

The cheatsheet is documentation, not crate content, so it is kept
out via package.exclude (a Cargo Reference field, beyond the book):

```toml
exclude = ["CHEATSHEET.md"]
```

```
warning: manifest has no documentation, homepage or repository
  |
  = note: see https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info
.cargo_vcs_info.json
Cargo.lock
Cargo.toml
Cargo.toml.orig
src/lib.rs
src/main.rs
```

### cargo publish --dry-run: pack + verify, no upload

No token, no `cargo login`, nothing reaches crates.io. Steps:
index check -> packaging -> verify (compiles the crate FROM the
packaged copy in target/package/) -> abort before upload.

```sh
cargo publish --dry-run 2>&1 | tail -12
```

```
    Updating crates.io index
warning: crate art@0.1.0 already exists on crates.io index
warning: manifest has no description, license, license-file, documentation, homepage or repository
  |
  = note: see https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info
   Packaging art v0.1.0 (...)
    Packaged 6 files, 4.0KiB (1.8KiB compressed)
   Verifying art v0.1.0 (...)
   Compiling art v0.1.0 (.../target/package/art-0.1.0)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.26s
   Uploading art v0.1.0 (...)
warning: aborting upload due to dry run
```

Two live findings in one run: the metadata warning, and proof the
name `art` is taken on crates.io (held since 2018 by an "adaptive
radix trie" crate) -- publishing under this name is impossible
regardless.

### The fix: metadata + publish = false (beyond the book)

```toml
description = "A library for modeling artistic concepts."
license = "MIT OR Apache-2.0"
publish = false # learning crate: keep off the crates.io namespace
```

`publish = false` is the idiomatic guard for crates that must
never reach a registry (learning projects, workspace-internal
crates). It blocks even the dry run -- comment it out temporarily
to see a fully clean `cargo publish --dry-run`:

```
error: `art` cannot be published.
`package.publish` must be set to `true` or a non-empty list in Cargo.toml to publish.
```

## Release profiles (ch 14.1)

```sh
cargo build            # Finished `dev` profile [unoptimized + debuginfo]
cargo build --release  # Finished `release` profile [optimized]
```

Defaults are overridable per profile in Cargo.toml
(`[profile.dev]` / `[profile.release]`); `opt-level` spans 0-3
(dev: 0 = fast compiles; release: 3 = fast code). A customization
example lives in this repo: `projects/panic` sets
`panic = 'abort'` in `[profile.release]`.

## Publishing for real -- the rules (reference only)

- Names are global, first-come-first-served, and PERMANENT: no
  deletions, unlimited versions. Publish only what you intend to
  maintain.
- Required metadata: `description` + `license` (SPDX identifiers;
  the Rust community default is the dual `MIT OR Apache-2.0`).
- `cargo login` stores the API token in `~/.cargo/credentials.toml`
  (a secret: revoke and rotate if leaked).
- New version = bump `version` per semver: breaking change ->
  major, new feature -> minor, fix -> patch; then `cargo publish`.
- A broken version cannot be deleted, only yanked:

```sh
cargo yank --vers 1.0.1        # new deps blocked; existing Cargo.lock fine
cargo yank --vers 1.0.1 --undo # reverse the yank
```

## Installing binaries (ch 14.4)

```sh
cargo install ripgrep   # compiles from source -> ~/.cargo/bin/rg
```

Only packages with a binary target (src/main.rs or a configured
[[bin]]) can be installed; binaries land in `$HOME/.cargo/bin`
(rustup default) -- keep it on `$PATH`. This is a convenience for
Rust tools, not a replacement for a system package manager.

## Custom subcommands (ch 14.5)

Any executable named `cargo-something` on `$PATH` becomes
`cargo something`; custom commands are listed by `cargo --list`.
Combined with `cargo install`, distributed crates can extend
Cargo itself.

## Workspaces (ch 14.3)

Covered by a dedicated cheatsheet in `projects/add/CHEATSHEET.md`
(virtual manifests, shared Cargo.lock/target, -p selection, E0432).
