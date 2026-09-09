// The library half: iterator mechanics as unit tests (the binary
// half keeps the runnable demos; see main.rs).

// The contract every iterator implements (std's Iterator trait):
//
//   pub trait Iterator {
//       type Item;
//
//       fn next(&mut self) -> Option<Self::Item>;
//
//       // methods with default implementations elided
//   }
//
// `type Item` is an associated type (covered in chapter 20): the
// type this iterator yields. `next` is the only REQUIRED method --
// one item at a time wrapped in Some, then None when exhausted.
// It takes &mut self: advancing the iterator mutates its internal
// position -- hence `mut v1_iter` below. `iter()` yields immutable
// references (Some(&1), not Some(1)); `into_iter` yields owned
// values, `iter_mut` mutable references.

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
}
