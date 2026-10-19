# advanced features cheatsheet (chapter 20)

The chapter's lookup page: unsafe superpowers and their
conventions, the trait/type/function machinery, and the four kinds
of macros. Everything below is lived in this project's modules
(`unsafe_rust.rs`, `traits.rs`, `types.rs`, `functions.rs`,
`macros.rs`) and in the `projects/macros` workspace.

## The five superpowers (20.1)

`unsafe` unlocks exactly these -- and does NOT turn off the borrow
checker:

1. dereference a raw pointer (`*const T` / `*mut T`)
2. call an unsafe fn / extern block
3. access a mutable static
4. implement an unsafe trait
5. access union fields

The signature litmus test: can a caller writing only safe code
cause UB with valid arguments? yes -> `unsafe fn` (the proof shifts
to the caller); no -> a safe fn, unsafe as an implementation detail
(`split_at_mut` stays a safe API: `assert!(mid <= len)` before the
block converts potential UB into a panic - a strict upgrade).

## SAFETY conventions (mechanically enforced)

- `// SAFETY:` above every unsafe block and unsafe impl
- `# Safety` doc section on unsafe fns and unsafe traits
- the lints doing the enforcing: `clippy::undocumented_unsafe_blocks`
  (on in this project's main.rs), `missing_safety_doc`

## Statics (20.10..20-11)

Mutable static: reads/writes behind unsafe; the deny-by-default
`static_mut_refs` forbids references (including println's invisible
autoref) -- read through `*(&raw const COUNTER)`, binding the
pointer first (`deref_addrof`'s suggestion would trip
`static_mut_refs` -- the lints contradict). The checked replacement
for counters: `AtomicU32` (plain movs on x86).

## Miri and the miri-clean setup

`cargo +nightly miri run` -- an interpreter catching UB as it
happens; silent about sound code; cannot execute native FFI (cfg
(miri) stub demos keep the run complete). The expansion cache:
macros and builds re-run only when the consumer changes.

## Advanced traits (20.2)

- associated types: the impl pins once (`type Item = u32`), the
  caller never annotates; a generic trait would allow MANY impls
  per type and force annotations everywhere
- default type parameters: `Add<Rhs = Self>` -- `impl Add for
  Point` means Point + Point; `impl Add<Meters> for Millimeters`
  pins the right-hand side to another type
- fully qualified syntax: `<Dog as Animal>::baby_name()` -- the
  one place nothing can be inferred away (E0790 without it)
- supertrait: `trait OutlinePrint: fmt::Display` -- a trait bound
  ON a trait; default methods may lean on capabilities they cannot
  provide themselves
- newtype pattern: a local tuple struct beats the orphan rule
  (`Wrapper(Vec<String>)`), brands units, hides APIs

## Advanced types (20.3)

newtype vs alias: `Kilometers = i32` is a SYNONYM - it mixes with
i32 freely, zero protection; `Millimeters(u32)` is a new type.
The never type `!`: zero values, coerces into any type (the ch 2
guess-game `continue` arm, finally explained). DSTs live behind
pointers, and the pointer gets FAT -- the metadata kind depends on
the DST:

| pointer        | size (x64) | second word        |
|----------------|------------|--------------------|
| `&u8`          | 8          | -- (thin)          |
| `&[u8; 5]`     | 8          | -- (size in TYPE)  |
| `&[u8]`, `&str`| 16         | length             |
| `&dyn Display` | 16         | vtable pointer     |
| `String`       | 24         | not a pointer: ptr + len + cap |

Generic fns get an implicit `T: Sized`; `?Sized` relaxes it (the
only trait allowed in `?Trait`), the parameter going behind a
pointer.

## Advanced functions and closures (20.4)

`fn` (lowercase) is a TYPE -- not the `Fn` trait: `do_twice(f:
fn(i32) -> i32, arg: i32)`. fn items implement all three closure
traits, so generic-Fn APIs accept both; enum variant names ARE
initializers (`(0..20).map(Status::Value)`). Returning closures:
`impl Fn` per single shape; every USE of impl Trait is a DISTINCT
opaque type (two impl-Fn returns cannot share a Vec, E0308) --
`Box<dyn Fn>` is the one-type fix (Thunk again; dyn Fn is a DST,
hence the fat pointer).

## Macros (20.5)

| kind           | declared with           | streams / shape                          |
|----------------|-------------------------|-------------------------------------------|
| declarative    | `macro_rules!`          | token patterns -> replacement             |
| derive         | `#[proc_macro_derive]`  | the annotated item in, impl out           |
| attribute-like | `#[proc_macro_attribute]` | TWO in (attr + item); MUST return the item |
| function-like  | `#[proc_macro]`         | the call's tokens in                      |

Procedural macros need their own crate (`[lib] proc-macro = true`);
the pipeline is `syn` (tokens -> syntax tree) then `quote!`
(templates back into tokens, `#name` as the `$x` idea reborn).
They run at COMPILE time of the consumer -- a `println!` inside
lands in the build log. Workspace mechanics: everything from the
root, `cargo add <dep> --package <crate>` (workspace members are
found by name -- no `--path`).
