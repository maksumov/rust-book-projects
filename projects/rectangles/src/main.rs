struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

// Refactored: named fields fix the tuple problem -- `rectangle.width`
// is self-documenting, unlike `dimensions.0`.
// Note: `&Rectangle` borrows the value (chapter 4), so `main` keeps
// ownership and could still use `rect1` after the call.
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
