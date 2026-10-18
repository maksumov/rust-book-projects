// ch 20.2: advanced traits (listings 20-13..20-24). Associated
// types (the Iterator Item), default type parameters and operator
// overloading (Add), fully qualified syntax for name collisions,
// supertraits (OutlinePrint: Display), the newtype pattern around
// the orphan rule (Wrapper).

// Listing 20-13: std's Iterator SKETCHED - non-runnable, the real
// trait lives in std (redefining it here would shadow it). `type
// Item` is a placeholder the IMPL pins; next returns
// Option<Self::Item>; the CALLER never annotates it - ch 13's
// sum/for/collect all ran on this without ever spelling Item.
//
// // pub trait Iterator {
// //     type Item;
// //
// //     fn next(&mut self) -> Option<Self::Item>;
// // }

use std::ops::Add;

// The section's unnumbered Counter impl - the runnable half: the
// placeholder is pinned HERE (type Item = u32); every call below
// discovers it. Counter is an old acquaintance from ch 13.
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// Listing 20-14: why not generics? Hypothetical too - a LEGAL trait
// definition but a different CONTRACT: with Iterator<T> a type could
// implement it MANY times (Iterator<u8> AND Iterator<String> for one
// Counter), so every call site would have to say which impl it means.
// Associated types pin one Item per impl: the impl decides, the
// caller discovers.
//
// // pub trait Iterator<T> {
// //     fn next(&mut self) -> Option<T>;
// // }

pub fn demo_associated_types() {
    println!("\n*** demo of advanced traits: associated types ***");

    let mut counter = Counter::new();
    println!("first next: {:?}", counter.next());

    // No annotations anywhere: Item was pinned once in the impl -
    // take/sum (all of ch 13's adapters) just work on our Counter.
    let sum: u32 = Counter::new().take(3).sum();
    println!("sum of the first three: {}", sum);
}

// Listing 20-15: overloading + by implementing std::ops::Add - new
// operators cannot be invented, only the fixed std::ops set is
// overloadable. Add carries ANOTHER associated type (Output) plus
// one DEFAULT TYPE PARAMETER - the book's unnumbered definition:
//
// // trait Add<Rhs = Self> {
// //     type Output;
// //
// //     fn add(self, rhs: Rhs) -> Self::Output;
// // }
//
// Rhs defaults to Self: a plain `impl Add for Point` means
// Point + Point.
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Listing 20-16: overriding the default Rhs - adding Meters to
// Millimeters, the unit conversion living inside the impl.
// `impl Add<Meters>` pins the right-hand side to ANOTHER type: the
// operator crosses units. Millimeters(u32) / Meters(u32) are thin
// wrappers - the book's first naming of the NEWTYPE PATTERN (it
// returns full-circle in 20-24 as an orphan-rule workaround).
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

pub fn demo_operator_overloading() {
    println!("\n*** demo of advanced traits: operator overloading ***");

    // Listing 20-15: the book's assert - the + operator now speaks
    // Point.
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    println!(
        "Point {{ x: 1, y: 0 }} + Point {{ x: 2, y: 3 }} = {:?}",
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 }
    );

    // Listing 20-16: Rhs pinned to Meters; 500mm + 1m = 1500mm.
    let total = Millimeters(500) + Meters(1);
    println!("500mm + 1m = {}mm", total.0);
}
