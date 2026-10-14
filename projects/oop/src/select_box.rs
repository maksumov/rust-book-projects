// ch 18.2, the consumer side: this module lives in the BINARY crate --
// the library (declared in lib.rs) does not know SelectBox exists.
// That is the whole point of trait objects: the library was written
// before this type, and Screen still works with it.

use oop::gui::{Button, Draw, Screen};

// Listing 18-8: another crate using gui and implementing Draw
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // beyond the book: observable output instead of the book's
        // "code to actually draw a select box"
        println!(
            "drawing SelectBox ({}x{}) with {} options",
            self.width,
            self.height,
            self.options.len()
        );
    }
}

pub fn demo_screen() {
    println!("\n*** demo of trait objects: a Screen of mixed components ***");

    // Listing 18-9: values of DIFFERENT types in one vector - SelectBox
    // from this crate, Button from the library.
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // Listing 18-10: a String does not implement Draw - caught at
    // COMPILE time (the duck-typing contrast: statically checked):
    //
    // // let screen = Screen {
    // //     components: vec![Box::new(String::from("Hi"))],
    // // };
    //
    // error[E0277]: the trait bound `String: Draw` is not satisfied
    //   = note: required for the cast from `Box<String>` to `Box<dyn Draw>`
}
