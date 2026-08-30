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

    let mut count = 0;
    let coin = Coin::Nickel;

    // Matching by reference (`&coin`): the Quarter arm binds `state`
    // as &UsState and `coin` is not consumed -- a must here, since
    // the `if let` below reuses it.
    // Unlike the earlier `_ => ()` cases, the catch-all arm does
    // real work here: counting everything that is not a quarter.
    match &coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }
    println!("count = {count}");

    // `else` plays the role of match's `_` arm -- this is the exact
    // equivalent of the match above, with the same trade-off:
    // no exhaustiveness check, other patterns ignored deliberately.
    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
    println!("count = {count}");

    // Calls for the three describe_state_quarter variants:
    if let Some(desc) = describe_state_quarter_nested(Coin::Quarter(UsState::Alaska)) {
        println!("{desc}");
    }

    if let Some(desc) = describe_state_quarter_early_return(Coin::Quarter(UsState::Alabama)) {
        println!("{desc}");
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

// Methods work on enums exactly like on structs (chapter 5.3).
// `match self` on `&self` binds by reference -- no ownership taken.
impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // ...
        }
    }
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

// First of three equivalent implementations of the same idea.
// The nested `if let` pushes all the work into the condition's
// body -- with more complex logic it becomes hard to follow how
// the top-level branches relate. The `_early_return` and
// `_let_else` variants below address this step by step.
fn describe_state_quarter_nested(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

// Second variant: `if let` used as a value-producing expression
// (same "if is an expression" idea as the branches project, ch 3.5)
// with an early return in the `else`. The main logic is flat now,
// but the flow is still awkward: one branch produces `state`,
// the other exits the function entirely. `let...else` (next)
// expresses exactly this pattern.
fn describe_state_quarter_early_return(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
