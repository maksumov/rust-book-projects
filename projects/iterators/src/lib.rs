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

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        // We aren't allowed to use v1_iter after the call to sum,
        // because sum takes ownership of the iterator we call it on:
        // println!("{v1_iter:?}");
        // error[E0382]: borrow of moved value: `v1_iter`

        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_producer_and_consumer() {
        let v: Vec<i32> = vec![1, 2, 3];

        // v.iter().map(|x| x + 1);
        // warning: unused `Map` that must be used
        //   --> src/lib.rs:56:9
        //    |
        // 56 |         v.iter().map(|x| x + 1);
        //    |         ^^^^^^^^^^^^^^^^^^^^^^^
        //    |
        //    = note: iterators are lazy and do nothing unless consumed
        //    = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
        // help: use `let _ = ...` to ignore the resulting value
        //    |
        // 56 |         let _ = v.iter().map(|x| x + 1);
        //    |         +++++++

        let producer = v.iter().map(|x| x + 1);

        let consumer: Vec<i32> = producer.collect();

        assert_eq!(consumer, vec![2, 3, 4]);
    }
}
