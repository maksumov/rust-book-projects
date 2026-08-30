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
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        // `dbg!` prints file:line and the expression to *stderr*
        // and returns the value, so it can be used inline in expressions.
        width: dbg!(30 * scale),
        height: 50,
    };

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
}
