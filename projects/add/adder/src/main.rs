fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));

    // beyond the book: rand is declared per member (the E0432 lesson) --
    // this call is why adder's Cargo.toml carries its own copy
    use rand::Rng;
    let jackpot: i32 = rand::thread_rng().gen_range(1..=100);
    println!("Lucky number: {jackpot}");
}
