// ch 20.3: advanced types. The newtype pattern's other uses (type
// safety, abstraction - the opening prose, no listing), type
// aliases (listings 20-25..20-26), the never type ! (listing
// 20-27), dynamically sized types and the Sized trait (no listing
// - the debt promised in gui.rs back in ch 18).

use std::collections::HashMap;

// The section opens PROSE-ONLY (no listing): the newtype pattern's
// uses beyond the orphan rule (20-24). Type safety: 20-16's
// Millimeters - a fn taking Millimeters rejects Meters and plain
// u32 at COMPILE time (Kilometers in the next section shows the
// ALIAS that does NOT buy this). Abstraction: a public API unlike
// the inner type's - the book's People example, live below: callers
// add names and never learn that ids exist; being a tuple struct
// with a private field, the HashMap is invisible OUTSIDE this
// module (the demo runs inside it - the hiding is structural, not
// demonstrated). A lightweight ch 18 encapsulation.
struct People(HashMap<i32, String>);

impl People {
    fn add_name(&mut self, name: &str) {
        // the id is internal: callers speak only names
        let id = self.0.len() as i32;
        self.0.insert(id, String::from(name));
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

pub fn demo_newtype_uses() {
    println!("\n*** demo of advanced types: newtype safety and abstraction ***");

    let mut people = People(HashMap::new());
    people.add_name("Ada");
    people.add_name("Grace");
    println!("names added: {}", people.len());
}

// Listing 20-25: the LONG type in every signature - perfectly
// legal, just tiresome and error-prone to write and read:
// a boxed closure that is Send and owns everything it touches
// ('static). Real bodies instead of the book's --snip-- ones.
fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    f();
}

fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    Box::new(|| println!("hi"))
}

// Listing 20-26: the alias makes it manageable. A thunk is code
// deferred to a later time - the right name for a stored closure
// (ch 13 stored closures; an old word from TS as well).
type Thunk = Box<dyn Fn() + Send + 'static>;

// (unnumbered) The alias with a PARTIALLY APPLIED generic: std::io
// declares `type Result<T> = std::result::Result<T, std::io::Error>;`
// - the error type baked in, one parameter left. The Write trait's
// signatures shrink from Result<usize, Error> to Result<usize>,
// and the alias keeps the full API: ? and every Result method
// still work - an alias is just another name for the SAME type.

pub fn demo_type_aliases() {
    println!("\n*** demo of advanced types: type aliases ***");

    // (unnumbered) The counter-example to the newtype above: an
    // alias is a SYNONYM, not a new type - Kilometers and i32 mix
    // freely, zero compile-time protection (Millimeters would have
    // rejected this). Items may live inside a fn - the book itself
    // declares the alias in main (as with the extern block in 20-8).
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("i32 + Kilometers = {}", x + y);

    // Listing 20-25: the long type spelled out.
    takes_long_type(returns_long_type());

    // Listing 20-26: the same value through the alias.
    let f: Thunk = returns_long_type();
    f();
    println!("thunk ran");
}

// Listing 20-27: the never type ! - zero values, the type of code
// that never produces one. The guess-game match (lived in full in
// ch 2) mixes `Ok(num) => num` (u32) with `Err(_) => continue` (!):
// legal because ! coerces into ANY type - continue jumps, it never
// yields a value. The same coercion runs Option::unwrap (the
// panic! arm) and an endless `loop` (a loop without break IS a !
// expression). A diverging fn:
//
// // fn bar() -> ! {
// //     panic!();
// // }
//
// And the match that must NOT mix - without ! there is no coercion,
// plain E0308:
//
// // let guess = match guess.trim().parse() {
// //     Ok(_) => 5,
// //     Err(_) => "hello",   // error[E0308]: mismatched types
// // };

pub fn demo_never_type() {
    println!("\n*** demo of advanced types: the never type ***");

    // The 20-27 pattern, compact: the Err arm continues, so the
    // match's type is settled by the Ok arm alone.
    for a in ["42", "not a number", "7"] {
        let n: u32 = match a.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("parsed: {n}");
    }
}

// Dynamically sized types (unnumbered): str - NOT &str - is the
// DST: its size is known only at runtime, so a bare str value is
// impossible. The golden rule: DSTs live behind a pointer. &str is
// a FAT pointer - address + length - so ITS size is known at
// compile time. Traits are DSTs too - hence dyn Trait behind
// &/Box (the gui.rs header debt from ch 18: why Screen held
// Box<dyn Draw>). Every generic fn gets an implicit T: Sized; the
// ?Sized relaxation ("may or may not be Sized" - legal only for
// this one trait) puts the parameter behind a pointer:
//
// // let s1: str = "Hello there!";
// // error[E0277]: the size for values of type `str` cannot be
// //               known at compilation time
//
// // fn generic<T>(t: T) {}           // implicitly T: Sized
// // fn generic<T: ?Sized>(t: &T) {}  // relaxed, t behind a ref

pub fn demo_dst() {
    println!("\n*** demo of advanced types: DSTs and fat pointers ***");

    use std::fmt::Display;
    use std::mem::size_of;

    // A pointer to a SIZED type is THIN (one word - the address).
    // A pointer to a DST is FAT (two words - address + metadata),
    // and the metadata DEPENDS ON THE DST KIND: a length for str /
    // [T], a VTABLE pointer for dyn Trait (the method table of the
    // concrete impl - the real reason dyn Trait lives behind a
    // pointer, ch 18). The metadata is part of the POINTER VALUE,
    // not of the allocation: Box<str> stays fat (16) - the heap
    // chunk carries no length header. Note &[u8; 5]: an array is
    // SIZED (the 5 lives in the type), so its pointer is thin -
    // drop the length from the type and &[u8] doubles to 16. And
    // String is not a fat pointer at all: an owning STRUCT of ptr
    // + len + capacity (24), which is why &String is thin again.
    println!(
        "&u8: {}, &[u8; 5]: {}, &[u8]: {}, &str: {}",
        size_of::<&u8>(),
        size_of::<&[u8; 5]>(),
        size_of::<&[u8]>(),
        size_of::<&str>()
    );
    println!(
        "Box<str>: {}, String: {}, &String: {}, &dyn Display: {}",
        size_of::<Box<str>>(),
        size_of::<String>(),
        size_of::<&String>(),
        size_of::<&dyn Display>()
    );
}
