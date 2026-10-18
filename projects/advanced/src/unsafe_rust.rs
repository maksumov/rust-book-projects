// ch 20.1: unsafe Rust (listings 20-1..20-12). The five superpowers:
// dereference a raw pointer, call an unsafe fn, access a mutable
// static, implement an unsafe trait, access union fields. unsafe does
// NOT turn off the borrow checker - it unlocks only these five; keep
// the blocks small and wrap them in safe abstractions (split_at_mut
// below). Miri (cargo +nightly miri run) checks unsafe at runtime.
// The deep dive into unsafe is the Rustonomicon - https://doc.rust-lang.org/nomicon/

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

    // SAFETY: both r1 and r2 were created from num, which is still
    // alive; nothing else borrows num in this block.
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

    // SAFETY: mid <= len is asserted, so ptr.add(mid) stays inside the
    // slice - the halves are disjoint and within the one allocation.
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Miri is an interpreter: it cannot execute native libc code - the
// abs call aborts with "unsupported operation" (Miri's own note: "it
// does not indicate a bug in the program"). Under the miri cfg the
// FFI demos shrink to a stub, keeping the miri run complete; the
// real bodies compile only without it.
#[cfg(miri)]
#[allow(rust_analyzer::inactive_code)]
pub fn demo_ffi() {
    println!(
        "\n*** demo of unsafe: FFI - skipped under miri, an interpreter cannot run native code ***"
    );
}

// Listing 20-7: the same from_raw_parts_mut on an ARBITRARY address
// is undefined behavior - we own nothing there; Miri flags it as a
// dangling pointer:
//
// // let address = 0x01234usize;
// // let r = address as *mut i32;
// // let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

#[cfg(not(miri))]
pub fn demo_ffi() {
    println!("\n*** demo of unsafe: FFI via an unsafe block ***");

    // Listing 20-8: an extern block DECLARES a foreign fn - the signature
    // only, no body. The definition lives in the system libc (every Rust
    // binary links it); at LINK time the unmangled C symbol `abs` resolves
    // there. The ABI is "C" (the calling convention). If the name did not
    // exist, the failure would be a link error, not a runtime one. (The
    // block lives inside the function: extern blocks are items and may
    // be declared in any scope.)
    unsafe extern "C" {
        fn abs(input: i32) -> i32;
    }

    // SAFETY: the declaration matches libc's abs - i32 in, i32 out, no
    // preconditions; any argument is a valid input.
    unsafe {
        println!("Absolute value of -3 according to C's abs: {}", abs(-3));
    }
}

#[cfg(miri)]
#[allow(rust_analyzer::inactive_code)]
pub fn demo_ffi_safe() {
    println!(
        "\n*** demo of unsafe: FFI - skipped under miri, an interpreter cannot run native code ***"
    );
}

#[cfg(not(miri))]
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
    // SAFETY: COUNTER is only touched on this single-threaded call path
    // (the contract the fn's doc comment above states).
    unsafe {
        COUNTER += inc;
    }
}

// Listing 20-12: the book's minimal unsafe trait - a marker of a
// promise the compiler cannot check. Deliberately abstract: the
// MECHANISM, not a real contract; the real incarnations follow.
/// # Safety
///
/// Implementors must uphold whatever invariant `method` relies on.
unsafe trait Foo {
    fn method(&self);
}

// SAFETY: i32 carries no invariant beyond its bits.
unsafe impl Foo for i32 {
    fn method(&self) {
        println!("Foo::method on i32: {}", self);
    }
}

// The real unsafe trait every program meets: Send. A raw pointer is
// !Send + !Sync by default - the compiler cannot prove who owns the
// pointee, so a wrapper holding one refuses to cross threads:
//
// error[E0277]: `*const i32` cannot be sent between threads safely
//  = note: `*const i32` cannot be sent... within this `{closure}`,
//          required by this bound in `spawn`
struct OwnedI32 {
    p: *const i32,
}

// SAFETY: OwnedI32 exclusively OWNS the allocation behind p (created
// in new, freed in Drop); moving the wrapper moves the ownership -
// no other access path exists, so a move to another thread is sound.
unsafe impl Send for OwnedI32 {}

impl OwnedI32 {
    fn new(v: i32) -> Self {
        OwnedI32 {
            p: Box::into_raw(Box::new(v)),
        }
    }

    fn get(&self) -> i32 {
        // SAFETY: p is a valid, exclusively owned allocation created
        // in new(); it stays valid until Drop runs.
        unsafe { *self.p }
    }
}

impl Drop for OwnedI32 {
    fn drop(&mut self) {
        // SAFETY: reconstructing the Box retakes exactly the ownership
        // new() handed to into_raw; drop runs exactly once.
        unsafe { drop(Box::from_raw(self.p as *mut i32)) };
    }
}

// Deliberately NOT implemented: `unsafe impl Sync for OwnedI32` would
// promise concurrent &-access is fine - it is not (exercises/20_miri
// task 7 shows Miri catching the race). The compiler cannot verify
// either promise; that is exactly why both are unsafe.

/// Every bit pattern is a valid value of the type (bytemuck's Pod in
/// miniature - the REASON for an unsafe trait to exist).
///
/// # Safety
///
/// Implementors promise: any bits are a valid value (no niche like
/// bool's 0/1-only rule) and size/alignment match the types
/// `reinterpret` pairs them with.
unsafe trait AnyBits {}

// SAFETY: every u32 bit pattern is a valid u32.
unsafe impl AnyBits for u32 {}
// SAFETY: every f32 bit pattern is a valid f32 (a signaling NaN is
// fine to read).
unsafe impl AnyBits for f32 {}

// A SAFE API standing on the trait's promise - the split_at_mut
// pattern again: assert the checkable part, unsafe for the promised
// part, the fn itself stays safe.
fn reinterpret<A: AnyBits, B: AnyBits>(src: A) -> B {
    assert_eq!(std::mem::size_of::<A>(), std::mem::size_of::<B>());
    // SAFETY: AnyBits PROMISES every bit pattern is a valid value of
    // both types and the sizes match (asserted above); the impls also
    // match alignments. A lying impl would make this safe fn UB - the
    // impl is where the promise lives. (ptr::read, not transmute:
    // transmute rejects generics whose size is not compile-time fixed.)
    unsafe { std::ptr::read(&src as *const A as *const B) }
}

pub fn demo_unsafe_trait() {
    println!("\n*** demo of unsafe: unsafe traits - the impl is the promise ***");

    // Listing 20-12: calling an unsafe trait's method is SAFE - the
    // unsafe sits at the impl, not at the call.
    42i32.method();

    // The Send promise in action: the spawn below is E0277 without
    // the unsafe impl (captured above), a plain move with it.
    let owned = OwnedI32::new(42);
    let handle = std::thread::spawn(move || owned.get());
    println!("read in another thread: {}", handle.join().unwrap());

    // The contract-bearing trait: safe code reinterpreting bits,
    // sound only while every impl keeps its promise.
    let f: f32 = reinterpret(0x3FC0_0000u32);
    println!("0x3FC00000 reinterpreted: {}", f);
}

// Unions (the fifth superpower, no listing in the book): like a
// struct, but exactly ONE field is valid at a time - which one is on
// the programmer. Reading a union field is unsafe (it may be garbage
// of the other branch); the niche is C interoperability. Details:
// the Rustonomicon (link in the module header).
