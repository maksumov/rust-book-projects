#[allow(dead_code)]
#[derive(Debug)]
struct Color(i32, i32, i32);
#[allow(dead_code)]
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{black:?}\n{origin:?}");
}
