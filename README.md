# rust-book-projects

Code-along projects and exercises from
[The Rust Programming Language](https://doc.rust-lang.org/book/).

## Chapter map

| Chapter | Topic | In this repo | Notes |
|---|---|---|---|
| 1 | Getting Started | `projects/hello_world`, `projects/hello_cargo` | |
| 2 | Programming a Guessing Game | `projects/guessing_game` | |
| 3 | Common Programming Concepts | `projects/functions`, `projects/branches`, `projects/loops` | + `exercises/03_common_concepts` |
| 4 | Understanding Ownership | `projects/ownership` | incl. slices |
| 5 | Using Structs to Structure Related Data | `projects/structs`, `projects/rectangles` | |
| 6 | Enums and Pattern Matching | `projects/enums` | incl. `let...else` |
| 7 | Managing Growing Projects | `projects/backyard`, `projects/restaurant` | modules; first lib crate |
| 8 | Common Collections | `projects/collections` | + `exercises/08_collections` |
| 9 | Error Handling | `projects/panic`, `projects/error-handling` | 9.3 guidelines anchored in both |
| 10 | Generic Types, Traits, and Lifetimes | `projects/aggregator`, `projects/lifetimes` | 10.0–10.1 skipped (prior knowledge); terms in `GLOSSARY.md` |
| 11 | Writing Automated Tests | `projects/testing` | + `CHEATSHEET.md` (cargo test flags) |
| 12 | An I/O Project: minigrep | `projects/minigrep` | + `exercises/12_minigrep` |
| 13 | Functional Language Features: Closures and Iterators | `projects/closures`, `projects/iterators` | 13.3 improves `projects/minigrep`; + `exercises/13_minigrep` |
| 14 | More About Cargo and Crates.io | `projects/my_crate`, `projects/art`, `projects/add` | doctests as tests; `#![warn(missing_docs)]`, `publish = false` — beyond the book; + `CHEATSHEET.md` (cargo package commands); + `exercises/14_publishing`; workspaces (14.3): + `CHEATSHEET.md` in `projects/add`, `exercises/14_workspace` |
| 15 | Smart Pointers | `projects/smart-pointers` | lib+bin split; + `CHEATSHEET.md` (pointer decision table, weak links, Rustonomicon pointer); terms in `GLOSSARY.md` (ch 15 audit: 8 entries) |
| 16 | Fearless Concurrency | `projects/concurrency` | terms in `GLOSSARY.md` (ch 16 audit: 7 entries); + `exercises/16_deadlock` |
| 17 | Fundamentals of Asynchronous Programming | `projects/hello-async` | first external dependency in the repo (`trpl`); bin-only by design (keeps dead-code tracking) |

## Other artifacts

- `GLOSSARY.md` — terms and concepts collected while studying
- `exercises/` — self-study exercise scaffolds (per chapter)
