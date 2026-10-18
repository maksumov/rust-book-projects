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

use std::{fmt, ops::Add};

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

// Listing 20-17: two traits declaring the same method name, plus an
// inherent method with that name too - all legal; the calls in the
// demo below decide which one runs.
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

pub fn demo_name_collisions() {
    println!("\n*** demo of advanced traits: method name collisions ***");

    let person = Human;

    // Listing 20-18: no ambiguity for the compiler - the INHERENT
    // method wins silently; the trait impls are not even reachable
    // through plain method syntax.
    person.fly();

    // Listing 20-19: reaching a trait impl: the trait names the fn,
    // the receiver pins the type. No fully qualified syntax needed
    // yet - with a self receiver, that much disambiguation suffices.
    Pilot::fly(&person);
    Wizard::fly(&person);
}

// Listing 20-20: the collision moves to associated functions - no
// self receiver to lean on. Dog's own baby_name and Animal's; the
// plain call below picks the inherent one again - Spot, though the
// puppy was the goal.
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

pub fn demo_fully_qualified_syntax() {
    println!("\n*** demo of advanced traits: fully qualified syntax ***");

    // Listing 20-20: the inherent associated fn wins - "Spot".
    println!("A baby dog is called a {}", Dog::baby_name());

    // Listing 20-21: naming the trait does NOT help without a
    // receiver - WHICH impl of Animal? The compiler refuses:
    //
    // error[E0790]: cannot call associated function on trait without
    // specifying the corresponding `impl` type
    //  = help: use the fully-qualified path to the only available
    //          implementation
    //          <Dog as Animal>::baby_name()
    //
    // println!("A baby dog is called a {}", Animal::baby_name());

    // Listing 20-22: fully qualified syntax - "treat Dog as Animal
    // for this call". The general form:
    // <Type as Trait>::function(receiver_if_method, args...);
    // every part Rust can infer is omittable - this is the one
    // spot where nothing can be inferred away.
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

// Listing 20-23: a SUPERTRAIT bound - OutlinePrint works only on
// types that already implement Display (`trait OutlinePrint:
// fmt::Display`), because its default method calls self.to_string()
// - a method that exists only via Display. An impl for a Display-
// less type is refused with E0277 - twice, at the impl and at the
// call site:
//
// // error[E0277]: `Point` doesn't implement `std::fmt::Display`
// //   = help: the trait `std::fmt::Display` is not implemented
// //           for `Point`
// //   note: required by a bound in `OutlinePrint`
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// The section's fix, live: Display for the SAME Point from 20-15 -
// and the OutlinePrint impl itself is EMPTY: all the work lives in
// the trait's default method.
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

pub fn demo_supertrait() {
    println!("\n*** demo of advanced traits: supertraits ***");

    let p = Point { x: 1, y: 3 };
    p.outline_print();
}

// Listing 20-24: the newtype pattern around the ORPHAN RULE: neither
// Display nor Vec is ours, so impl Display for Vec<String> is
// forbidden - but Wrapper, a local tuple struct around it, IS ours,
// and the impl lands on the wrapper. Zero runtime cost: the wrapper
// is elided at compile time. The downside: Wrapper has none of
// Vec's methods - delegate them manually, or implement Deref to get
// them all (ch 15.2), or keep the surface restricted on purpose.
// Full circle: the name first appeared on Millimeters in 20-16.
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn demo_newtype_pattern() {
    println!("\n*** demo of advanced traits: the newtype pattern ***");

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
