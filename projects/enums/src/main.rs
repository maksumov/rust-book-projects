use std::net::{Ipv4Addr, Ipv6Addr};

// An enum is a type that can be exactly one of its named variants.
// Each variant can hold a different type of data. Using std's
// Ipv4Addr/Ipv6Addr mirrors how std::net::IpAddr is defined.
#[allow(dead_code)]
#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// Variants can take any form: no data (Quit), named fields (Move,
// struct-like), a single value (Write), or a tuple (ChangeColor).
// The book's alternative -- four separate structs -- loses here:
// each would be a distinct type, while the enum is a single type
// that a function can accept as one argument.
#[allow(dead_code)]
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // Enums can have methods, just like structs (chapter 5.3).
        // `call` borrows the instance (&self); the variant's data
        // is accessible through `self` (used with `match` in 6.2).
        println!("Message {self:?} is called!")
    }
}

fn route(ip_addr: IpAddr) {
    println!("Routing {ip_addr:?} IP address!")
}

fn main() {
    // Nested functions are legal in Rust; the book defines `plus_one`
    // inside `main` for listing compactness.

    // Exhaustive again: restoring the `None` arm fixes the
    // "pattern `None` not covered" error. Rust checks match
    // exhaustiveness at compile time -- no case can be forgotten.
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let home = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    let loopback = IpAddr::V6("::1".parse().expect("invalid Ipv6Addr"));

    route(home);
    route(loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    option_demo();

    println!(
        "Quarter is {} cents",
        value_in_cents(Coin::Quarter(UsState::Alaska))
    );

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("plus_one(five) = {six:?}, plus_one(None) = {none:?}");

    // Catch-all arm: `other` binds the value of any case not covered
    // above (here: 9) and passes it on. Type inference makes the whole
    // chain u8: `9` is an untyped literal, `move_player` expects u8.
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // `_` is a catch-all that does NOT bind: the value is ignored,
    // so there is no unused-variable risk and no need to name it.
    // (`_ => ()` would do nothing at all.)
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    // `_ => ()`: the unit value -- do nothing for all other rolls.
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(_num_spaces: u8) {}
    fn reroll() {}

    // Verbose pattern: only one arm matters, yet exhaustiveness
    // forces the `_ => ()` arm. `if let` (next) shortens exactly this.
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // `if let` is sugar for a match with a single arm plus `_ => ()`:
    // less typing, no exhaustiveness noise. Trade-off: losing match
    // exhaustiveness checking -- the compiler no longer verifies
    // that other cases were considered deliberately.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

// `Option<T>` is an enum: Some(T) or None. Rust has no null --
// absence of a value is explicit in the type system.
// Note: `Some(5)` is Option<i32>, not i32 -- different types,
// they can't be mixed without unpacking (fixed with match below).
fn option_demo() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None; // type annotation required

    println!("{some_number:?}, {some_string:?}, {absent_number:?}");

    // Fixed with `match`: unpack `Option<i8>` before arithmetic.
    // Both arms are required (exhaustiveness); `None` must be
    // handled explicitly -- that's the point of Option<T>.
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = match y {
        Some(i) => x + i,
        None => 0,
    };
    println!("sum = {sum}");
}

#[allow(dead_code)]
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // ...
}

// `match` compares a value against patterns; the first matching
// arm wins. Arms must cover every possible variant (exhaustiveness).
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,

        // The pattern binds the variant's inner value to `state`.
        // Multiple statements in an arm: use a block; the last
        // expression is the arm's value.
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
