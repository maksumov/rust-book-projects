// ch 15.2: Deref -- treating smart pointers like regular references
// (listings 15-6..15-13).

pub fn demo_deref_operator() {
    println!(
        "\n*** demo of using the dereference operator to follow a reference to an i32 value ***"
    );

    // Listing 15-6: y is an arrow to x -- assertions must follow the
    // reference with * to reach the value (5 == &5 would not compile).
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Beyond the book: making the reference itself visible. {:?} on &i32
    // transparently formats the pointee (prints 5, not the arrow); the
    // Pointer trait's {:p} is the dedicated address formatter.
    println!("x = {x}, living at {:p}", &x);
    println!("y points to {:p}, and *y = {}", y, *y); // same address as &x
    assert_eq!(format!("{y:p}"), format!("{:p}", &x)); // y really is the arrow to x
}
