#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
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
        area(&rect1)
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

// Refactored: named fields fix the tuple problem -- `rectangle.width`
// is self-documenting, unlike `dimensions.0`.
// Note: `&Rectangle` borrows the value (chapter 4), so `main` keeps
// ownership and could still use `rect1` after the call.
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
