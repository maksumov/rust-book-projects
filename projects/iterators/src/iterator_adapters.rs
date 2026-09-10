// The laziness proof: creation and consumption are separate, and
// the side-effect probe shows WHEN the work actually happens.

pub fn demo() {
    println!("\n*** iterator adapters: laziness probe ***");

    let v = vec![1, 2, 3];
    let producer = v.iter().map(|x| {
        println!("mapping {x}"); // the probe
        x + 1
    });

    println!("iterator created, nothing mapped yet");
    let consumer: Vec<_> = producer.collect();
    println!("collected: {consumer:?}");
}

// beyond the book
//
// Infinite iterators exist precisely because of laziness: (1..)
// never materializes; take(5) caps the consumption.
pub fn demo_infinite() {
    println!("\n*** iterator adapters: infinite source ***");

    let evens = (1..).map(|x| x * 2);

    for v in evens.take(5) {
        println!("{v}");
    }
}
