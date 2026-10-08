# smart pointers decision cheatsheet (chapter 15)

Which standard-library smart pointer to reach for, and what changes
when one is involved. All outputs below are examples captured from
this project's demos (plus one scratch capture for the panic).

## The decision table

| type            | owners | borrows allowed    | checked         | threads |
|-----------------|--------|--------------------|-----------------|---------|
| `Box<T>`        | one    | immutable or mutable | compile time  | any     |
| `Rc<T>`         | many   | immutable only     | compile time    | single  |
| `RefCell<T>`    | one    | immutable or mutable | RUNTIME (panic) | single |
| `Weak<T>`         | none (non-owning) | via `upgrade()` -> `Option<Rc<T>>` | — | single |

The chapter's combination: `Rc<RefCell<T>>` -- many owners AND
mutation, borrowing rules enforced at runtime.

## Compile time vs runtime

With references, `Box` and `Rc`, violating the borrow rules is a
COMPILER error (E0596 cannot borrow as mutable, E0382 use of moved
value -- both kept as comments in this project's modules). `RefCell`
moves the same rules to runtime -- the code compiles and panics
instead. Captured from a scratch test (the 15-23 scenario):

```
thread 'tests::double_borrow_mut' (214834) panicked at src/lib.rs:7:25:
RefCell already borrowed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The trade-off: certain memory-safe programs that static analysis
must reject become writable -- at the price of discovering violations
later (possibly in production) and a small runtime cost.

## One mutation, every owner (listing 15-24)

`*value.borrow_mut() += 10;` through ONE handle of an
`Rc<RefCell<i32>>` shared by three lists (before/after prints are
beyond the book -- it prints only "after"):

```
a before = Cons(RefCell { value: 5 }, Nil)
b before = Cons(RefCell { value: 3 }, Cons(RefCell { value: 5 }, Nil))
c before = Cons(RefCell { value: 4 }, Cons(RefCell { value: 5 }, Nil))
...
a after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
```

## Reference counting (listing 15-19)

```
count after creating a = 1
count after creating b = 2
count after creating c = 3
count after c goes out of scope = 2
```

No manual decrement anywhere: `Rc::clone` up, scope end down (the
decrement IS Rc's Drop -- the "counters, not memory" case).

## Making pointers visible (beyond the book)

`{:p}` (the `Pointer` trait) prints addresses:

```
x = 5, living at 0x7ffd1ae5637c     <- the reference target: the stack slot
y points to 0x557a05ee0d90, and *y = 5   <- the Box: another region (heap)
```

A plain reference shows the pointee's own address; a `Box` shows a
heap region. `MyBox` cannot participate: `Pointer` is opt-in and our
wrapper does not implement it (nor does `{:?}` reveal anything -- Debug
on a reference transparently formats the pointee).

## Weak links (15.6): the reference-cycle antidote

Captured from demo_weak_tree (listing 15-29) -- branch lives in an
inner scope, leaf holds a weak parent link to it:

```
branch strong = 1, weak = 1      <- the weak link counts... but not as an owner
leaf parent = Some(...)          <- upgrade() works inside the scope
leaf parent = None               <- after the scope: the value is DROPPED
leaf strong = 1, weak = 0
```

upgrade() returning Option is the whole safety story: a weak link
never keeps the value alive -- and never dangles either.

## Going further: implementing your own smart pointers

The book's own pointer at the end of this chapter: The Rustonomicon
(https://doc.rust-lang.org/nomicon/) -- the manual of unsafe Rust:
raw pointers, aliasing invariants, safe APIs over unsafe internals.
Exactly the toolkit behind Box/Rc/RefCell (raw-pointer fields,
unsafe wrapped in safe methods). Best read AFTER ch 20 (Unsafe
Rust) -- it assumes the whole book.

## Coming with ch 16

`Arc` = thread-safe `Rc`; `Mutex` = thread-safe `RefCell`; `RwLock`
is the reader/writer refinement.
