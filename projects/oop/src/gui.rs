// ch 18.2: trait objects - abstracting over shared behavior (listings
// 18-3..18-10). A trait object = a pointer to a value + a table of its
// trait methods, resolved at RUNTIME (dynamic dispatch - no inlining,
// a small cost per call). Why dyn needs a pointer: ch 20 (DST). The
// trait-object rules are called dyn compatibility (renamed from
// "object safety" - the old name still roams the wild).

// Listing 18-3: Definition of the Draw trait
pub trait Draw {
    fn draw(&self);
}

// Listing 18-4: Definition of the Screen struct with a components
// field holding a vector of trait objects that implement the Draw trait
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// Listing 18-5: A run method on Screen that calls the draw method on
// each component
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Listing 18-6: the same Screen via generics and trait bounds. The
// contrast: a generic Screen<T> is ONE concrete type per instance --
// all Buttons or all TextFields, monomorphized, statically dispatched.
// The trait-object Screen mixes types in one vector at the price of
// dynamic dispatch. Homogeneous -> generics; heterogeneous -> dyn.
//
// // pub struct Screen<T: Draw> {
// //     pub components: Vec<T>,
// // }
// //
// // impl<T> Screen<T>
// // where
// //     T: Draw,
// // {
// //     pub fn run(&self) {
// //         for component in self.components.iter() {
// //             component.draw();
// //         }
// //     }
// // }

// Listing 18-7: A Button struct that implements the Draw trait:
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // beyond the book: observable output instead of the book's
        // "code to actually draw a button"
        println!(
            "drawing Button '{}' ({}x{})",
            self.label, self.width, self.height
        );
    }
}
