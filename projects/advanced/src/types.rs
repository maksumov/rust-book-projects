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
