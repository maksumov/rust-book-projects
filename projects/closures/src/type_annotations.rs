use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

pub fn demo_first() {
    println!("\n*** type annotations: first demo ***");

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

pub fn demo_second() {
    fn add_one_function(x: u32) -> u32 {
        x + 1
    }
    let add_one_closure_annotated = |x: u32| -> u32 { x + 1 };
    let add_one_closure_inferred = |x| x + 1;

    println!("\n*** type annotations: second demo ***");
    println!("add_one_function: {}", add_one_function(42));
    println!(
        "add_one_closure_annotated: {}",
        add_one_closure_annotated(42)
    );
    println!("add_one_closure_inferred: {}", add_one_closure_inferred(42));
}

pub fn demo_third() {
    println!("\n*** type annotations: third demo (inference lock) ***");

    // The first call locks the closure's parameter and return types;
    // a second call with a different type does not compile.
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    println!("first call returns: {s}");

    // Uncomment to see the inference lock in action:
    //
    // let n = example_closure(5);
    //
    // error[E0308]: mismatched types
    //   --> src/type_annotations.rs:57:29
    //    |
    // 57 |     let n = example_closure(5);
    //    |             --------------- ^ expected `String`, found integer
    //    |             |
    //    |             arguments to this function are incorrect
    //    |
    // note: expected because the closure was earlier called with an argument of type `String`
    //   --> src/type_annotations.rs:54:29
    //    |
    // 54 |     let s = example_closure(String::from("hello"));
    //    |             --------------- ^^^^^^^^^^^^^^^^^^^^^ expected because this argument is of type `String`
    //    |             |
    //    |             in this closure call
    // note: closure parameter defined here
    //   --> src/type_annotations.rs:53:28
    //    |
    // 53 |     let example_closure = |x| x;
    //    |                            ^
    // help: try using a conversion method
    //    |
    // 57 |     let n = example_closure(5.to_string());
    //    |                              ++++++++++++
}
