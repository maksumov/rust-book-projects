pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }

    // beyond the book: a live use for the rand dependency -- the
    // invariant holds for random inputs, not just a hand-picked one
    #[test]
    fn works_for_random_inputs() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let x: i32 = rng.gen_range(-1000..1000);
            assert_eq!(add_one(x), x + 1);
        }
    }
}
