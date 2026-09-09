// What a closure does with captured values determines which Fn
// traits it implements (additively):
//   1. FnOnce -- may move captures out of its body.
//   2. FnMut -- may mutate them.
//   3. Fn -- only reads them (or captures nothing).
// APIs pick the weakest bound they need.

// `height` is never read: dead code analysis ignores the derived
// Debug impl, so without this allow the demo warns.
#[allow(unused)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// sort_by_key bounds its key closure with FnMut: it calls it once
// per item. This closure captures nothing, so it qualifies.
pub fn demo_sort_by_key() {
    println!("\n*** fn traits: sort by key ***");

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
}

// Two ways to count closure calls: pushing an owned String out of
// the body makes the closure FnOnce-only (rejected by sort_by_key);
// mutating a captured counter keeps it FnMut (accepted).
pub fn demo_sort_counter() {
    println!("\n*** fn traits: sort counter ***");

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // Uncomment next lines to see the error:
    // error[E0507]: cannot move out of `value`,
    // a captured variable in an `FnMut` closure
    // let mut sort_operations = vec![];
    // let value = String::from("closure called");
    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });
    // println!("{list:#?}");

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}

// A function name works where an Fn trait is expected, if no
// capture is needed:
pub fn demo_fn_name_instead() {
    println!("\n*** fn traits: function name ***");

    let maybe: Option<Vec<i32>> = None;
    let v = maybe.unwrap_or_else(Vec::new);
    println!("fallback vector: {v:?}");
}
