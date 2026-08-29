fn main() {
    let s = no_dangle();

    println!("{s}");
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
