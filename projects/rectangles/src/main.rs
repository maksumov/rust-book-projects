fn main() {
    // Refactored: `rect1` is now a single tuple, so width and height
    // are grouped together and `area` takes one argument.
    // Problem: tuples don't name their elements, so `dimensions.0` and
    // `dimensions.1` are meaningless -- which one is width?
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
