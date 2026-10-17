// ch 20.1: unsafe Rust (listings 20-1..20-12). The five superpowers:
// dereference a raw pointer, call an unsafe fn, access a mutable
// static, implement an unsafe trait, access union fields. unsafe does
// NOT turn off the borrow checker - it unlocks only these five; keep
// the blocks small and wrap them in safe abstractions (split_at_mut
// below). Miri (cargo +nightly miri run) checks unsafe at runtime.

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
