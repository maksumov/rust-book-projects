#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods live inside an `impl` block of the type.
// `&self` is short for `self: &Rectangle`: the method borrows the
// instance, so `rect1` remains usable after the call (chapter 4).
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // A method can take additional parameters besides `self`.
    // `other: &Rectangle` borrows the argument, like a regular function.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated functions have no `self`: they don't read or modify
    // an instance. Called via the type itself: `Rectangle::square(25)`.
    // Often used as constructors; `Self` is an alias for the type
    // of the `impl` block.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        // `dbg!` prints file:line and the expression to *stderr*
        // and returns the value, so it can be used inline in expressions.
        width: dbg!(15 * scale),
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // `::` syntax -- like `String::from` (chapter 4): the function
    // belongs to the type, not to an instance.
    let square = Rectangle::square(25);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // `#[derive(Debug)]` generates a Debug implementation for free.
    // `{:?}` prints compactly on one line, `{:#?}` pretty-prints multiline.
    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}");

    // `dbg!` takes ownership of its argument, so pass a reference
    // to keep using `rect1` afterwards (nothing else uses it here,
    // but the habit matters).
    dbg!(&rect1);
    dbg!(&square);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold square? {}", rect1.can_hold(&square));
}
