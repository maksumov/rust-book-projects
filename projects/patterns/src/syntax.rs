// ch 19.3: the pattern syntax catalog (listings 19-11..19-29),
// clustered by SURPRISE rather than by page: the shadowing traps,
// destructuring, the ways of ignoring, guards, and @ bindings.

pub fn demo_shadowing_traps() {
    println!("\n*** demo of patterns: the shadowing traps (predict first!) ***");

    // Listing 19-11: PREDICT - what does the second arm print, and
    // which y is it? The answer: "Matched, y = 5" - Some(y) declares
    // a NEW y in the arm's scope, shadowing the outer 10. It binds
    // the value INSIDE the Some; it does not compare against the
    // outer y.
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    // Listing 19-26: a match guard - an if AFTER the pattern; the arm
    // needs BOTH to fire. There is no way to spell "x is even" as a
    // pattern:
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    // Listing 19-27: the cure - a match guard. Bind under a FRESH
    // name (n), compare against the OUTER y in the guard: guards
    // are not patterns, so their y is the real y.
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    // Listing 19-28: trap two - precedence. The guard applies to the
    // WHOLE or-pattern: (1 | 3 | 5) if y - and on stable there is no
    // way to guard ONE alternative:
    //
    // // 1 | 3 | (5 if y) => ...   // error[E0658]: guard patterns are
    // //                            // experimental (issue #129967)
    //
    // PREDICT: x matches 1... but y is false, so the arm fails:
    let x = 1;
    let y = false;

    match x {
        1 | 3 | 5 if y => println!("yes"),
        _ => println!("no"),
    }
}

pub fn demo_destructuring() {
    println!("\n*** demo of patterns: literals, ranges, destructuring ***");

    // Matching literals, several at once (|), and ranges (..=);
    // ranges only for chars and numbers - the compiler checks they
    // are not empty:
    let x = 5;
    match x {
        1 | 2 => println!("one or two"),
        3..=7 => println!("three through seven"),
        _ => println!("something else"),
    }

    // Listings 19-12/19-13: destructuring a struct in a let - the
    // explicit form renames the fields; the shorthand takes the field
    // names (the common form). One construction per let: destructuring
    // MOVES the struct.
    struct Point {
        x: i32,
        y: i32,
    }
    let Point { x: a, y: b } = Point { x: 0, y: 7 };
    let Point { x, y } = Point { x: 0, y: 7 };
    println!("explicit: {a}, {b}; shorthand: {x}, {y}");

    // Listing 19-14: literal values INSIDE a struct pattern - testing
    // some fields while binding others. PREDICT for (0, 0): which
    // arm wins? (The first - match stops at the first hit.)
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }

    // Listing 19-23: .. in a struct pattern - take x, drop the rest:
    let Point { x, .. } = Point { x: 0, y: 7 };
    println!("x only: {x}");

    // Listing 19-15: an enum arm per variant SHAPE - none, struct-like
    // (shorthand fields), tuple-like (positional). Listing 19-16 nests
    // a second enum inside:
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message {
        ChangeColor(Color),
        Write(String),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        Message::Write(text) => println!("Text message: {text}"),
    }

    // beyond the book: exercise the remaining arms (and keep the
    // dead-code analysis quiet - pattern arms alone do not count):
    let msg = Message::ChangeColor(Color::Rgb(255, 0, 0));
    if let Message::ChangeColor(Color::Rgb(r, g, b)) = msg {
        println!("Change color to red {r}, green {g}, and blue {b}");
    }

    let msg = Message::Write(String::from("hello patterns"));
    if let Message::Write(text) = msg {
        println!("Text message: {text}");
    }
}

pub fn demo_ignoring() {
    println!("\n*** demo of patterns: the ways of ignoring ***");

    // Listing 19-17: _ as a whole function parameter - the signature
    // keeps its shape (a trait impl may need it), the body ignores:
    fn second_only(_: i32, y: i32) -> i32 {
        y
    }
    println!("second only: {}", second_only(3, 4));

    // Listing 19-18: a nested _ - test the SHAPE (two Somes) while
    // ignoring the contents:
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing customized value"),
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {setting_value:?}");

    // Listing 19-20: a leading underscore silences the unused-variable
    // warning (the book contrasts _x with a plain y that warns; here
    // only the silenced form lives, keeping the build at zero):
    let _x = 5;

    // Listings 19-21/19-22: _x vs _ - the OWNERSHIP difference.
    // _s BINDS the value - and a binding moves an owning type:
    //
    // // if let Some(_s) = s {
    // //     println!("found a string");
    // // }
    // // println!("{s:?}");
    //
    // error[E0382]: borrow of moved value: `s`
    //   = note: move occurs because `_s` has type `String`
    //
    // A bare _ never binds - s stays usable (live below):
    let s = Some(String::from("Hello!"));
    // The pattern IS the lesson here (19-22: _ never binds, unlike
    // _s) - is_some() would erase it.
    #[allow(clippy::redundant_pattern_matching)]
    if let Some(_) = s {
        println!("found a string");
    }
    println!("s still usable: {s:?}");

    // Listing 19-19: ignoring by position - underscores in place
    // (same single-arm match as 19-24, same allow story):
    let numbers = (2, 4, 8, 16, 32);

    #[allow(clippy::match_single_binding)]
    match numbers {
        (first, _, third, _, fifth) => {
            println!("odd positions: {first}, {third}, {fifth}")
        }
    }

    // Listing 19-24 (verbatim): .. ignores the middle of a tuple.
    // Clippy's match_single_binding fires even on the book's own
    // form - the lint's critique (a single irrefutable arm makes
    // the match redundant) is fair, but the listing is the listing:
    #[allow(clippy::match_single_binding)]
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // Listing 19-25: .. must be unambiguous - which of the middle
    // values should second bind to? It does not compile:
    //
    // // match numbers {
    // //     (.., second, ..) => {
    // //         println!("Some numbers: {second}")
    // //     }
    // // }
    //
    // error: `..` can only be used once per tuple pattern
}

pub fn demo_at_bindings() {
    println!("\n*** demo of patterns: @ - test and bind at once ***");

    // Listing 19-29: id @ 3..=7 checks the range AND captures the
    // value. The second arm shows the contrast: a bare range tests
    // but binds nothing (the arm's code cannot name the value);
    // the third binds but tests nothing.
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id @ 3..=7 } => {
            println!("Found an id in range: {id}")
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range (value not bound)")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
}
