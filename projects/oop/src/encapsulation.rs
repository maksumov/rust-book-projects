// ch 18.1: characteristics of object-oriented languages (listings
// 18-1..18-2). Objects: structs/enums + impl blocks (the Gang of Four
// definition). Encapsulation: private by default, pub as the API --
// the invariant here: `average` is cached and can never go stale.

// Listings 18-1..18-2: private fields behind a public API; every mutator
// calls update_average, so the cached average is always in sync.
#[derive(Default)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

    // Beyond the book: it never constructs an AveragedCollection, so no
    // constructor is shown; a demo needs one. new()/default() join
    // add/remove/average as the public API -- the fields stay
    // unreachable from outside.
    pub fn new() -> AveragedCollection {
        AveragedCollection::default()
    }
}

pub fn demo_averaged_collection() {
    println!("\n*** demo of OOP characteristics: encapsulation ***");

    let mut c = AveragedCollection::new();
    c.add(10);
    c.add(20);
    c.add(30);
    println!("average after adds: {}", c.average()); // 20.0

    c.remove();
    println!("average after remove: {}", c.average()); // 15.0
}
