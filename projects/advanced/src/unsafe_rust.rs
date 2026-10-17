// ch 20.1: unsafe Rust (listings 20-1..20-12). The five superpowers:
// dereference a raw pointer, call an unsafe fn, access a mutable
// static, implement an unsafe trait, access union fields. unsafe does
// NOT turn off the borrow checker - it unlocks only these five; keep
// the blocks small and wrap them in safe abstractions (split_at_mut
// below). Miri (cargo +nightly miri run) checks unsafe at runtime.

use std::slice;

// Listings 20-10/20-11: used by demo_static below.
static HELLO_WORLD: &str = "Hello, world!";
// (The modern replacement for a numeric counter: `static COUNTER:
// AtomicU32` - checked, and plain movs for loads/stores on x86.)
static mut COUNTER: u32 = 0;

pub fn demo_raw_pointers_deref() {
    println!("\n*** demo of unsafe: raw pointers created safely, dereferenced unsafely ***");

    // Listings 20-1..20-3: creating raw pointers is SAFE - via the
    // raw borrow operators &raw const / &raw mut (a recent syntax; the
    // old `&num as *const i32` still appears in older code). The
    // asterisk is part of the TYPE, not the deref operator. Both point
    // at the same memory - an & and &mut pair to one location would
    // not compile (the {:p} output proves it). Dereferencing is the
    // unsafe part (20-3) - hence the block below.

    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    // Listing 20-2: a raw pointer to an ARBITRARY address - possible
    // but almost always wrong (segfault territory); shown because casts
    // like this appear in the wild:
    //
    // // let address = 0x012345usize;
    // // let r = address as *const i32;

    unsafe {
        println!("r1 is: {} and the raw pointer refers to: {:p}", *r1, r1);
        println!("r2 is: {} and the raw pointer refers to: {:p}", *r2, r2);
    }
}

pub fn demo_unsafe_fn() {
    println!("\n*** demo of unsafe: unsafe fns and the safe abstraction over them ***");

    // Listing 20-4: the split_at_mut USE - a perfectly safe call. (The
    // book's minimal unsafe fn - fn dangerous() {} - needs an unsafe
    // block to call; an unsafe fn's BODY grants no superpowers, unsafe
    // ops inside still require their own block.) Note that calling
    // slice::from_raw_parts_mut and ptr.add below ARE unsafe-fn calls -
    // that is what the block in the implementation is for.

    let mut v = [1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    println!("a is {:?}", a);
    println!("b is {:?}", b);
}

// Listing 20-5: safe Rust CANNOT do this - the borrow checker sees
// two mutable borrows of the same slice and cannot prove the parts
// do not overlap:
//
// // fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
// //     (&mut values[..mid], &mut values[mid..])
// // }
//
// error[E0499]: cannot borrow `*values` as mutable more than once
//   = help: use `.split_at_mut(position)` to obtain two mutable
//   non-overlapping sub-slices

// Listing 20-6: the unsafe implementation. A slice IS pointer +
// length: as_mut_ptr hands the raw pointer out, from_raw_parts_mut
// rebuilds a slice from (ptr, len). What makes this SOUND: the
// asserted mid <= len keeps ptr.add(mid) inside the slice, so the
// halves cannot overlap. The punchline: split_at_mut remains a SAFE
// fn - the unsafe block is an implementation detail, every caller
// stays in safe Rust.
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Listing 20-7: the same from_raw_parts_mut on an ARBITRARY address
// is undefined behavior - we own nothing there; Miri flags it as a
// dangling pointer:
//
// // let address = 0x01234usize;
// // let r = address as *mut i32;
// // let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

pub fn demo_ffi() {
    println!("\n*** demo of unsafe: FFI via an unsafe block ***");

    // Listing 20-8: an extern block DECLARES a foreign fn - the signature
    // only, no body. The definition lives in the system libc (every Rust
    // binary links it); at LINK time the unmangled C symbol `abs` resolves
    // there. The ABI is "C" (the calling convention). If the name did not
    // exist, the failure would be a link error, not a runtime one. (The
    // block lives inside the function: extern blocks are items and may
    // be declared in any score.)
    unsafe extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C's abs: {}", abs(-3));
    }
}

pub fn demo_ffi_safe() {
    println!("\n*** demo of unsafe: FFI via the safe keyword ***");

    // Listing 20-9: one keyword changes the contract - a `safe fn` decl
    // drops the unsafe block requirement: the call below is a plain
    // expression. `safe` is a PROMISE by the author, not something the
    // compiler can prove. A DIFFERENT symbol (labs - abs for long) is
    // used here: C symbols are program-global, so redeclaring `abs`
    // with another safety would be a conflicting declaration.
    unsafe extern "C" {
        safe fn labs(input: i64) -> i64;
    }

    println!("Absolute value of -5 according to C's labs: {}", labs(-5));
}

pub fn demo_static() {
    println!("\n*** demo of unsafe: statics, from immutable to mutable ***");

    // Listing 20-10: an immutable static (declared at the top of this
    // module). Unlike a const - inlined at every use - a static has ONE
    // fixed address for the whole program.
    println!("HELLO_WORLD's value is: {HELLO_WORLD}");

    // Listing 20-11: a mutable static - reads and writes are unsafe, the
    // compiler cannot prove single-threaded access. The book's inline
    // *(&raw const COUNTER) trips clippy::deref_addrof, whose suggested
    // `COUNTER` would in turn trip static_mut_refs (deny by default) - the
    // lints contradict. Binding the pointer first dodges both: creation is
    // SAFE, the deref is not (same split as demo_raw_pointers). Note:
    // add_to_count's `COUNTER += inc` needs no raw dance - a direct place
    // write; only REFERENCE creation (explicit, or println's invisible
    // autoref) is denied.
    let counter_ptr = &raw const COUNTER;

    unsafe {
        // SAFETY: This is only called from a single thread in `main`.
        println!("COUNTER: {}", *counter_ptr);

        add_to_count(3);
        println!("COUNTER: {}", *counter_ptr);

        // Uncommenting trips the deny-by-default static_mut_refs:
        // error: creating a shared reference to mutable static
        //  = note: `#[deny(static_mut_refs)]` (part of `#[deny(rust_2024_compatibility)]`) on by default
        // println!("COUNTER: {}", COUNTER);
    }
}

/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
