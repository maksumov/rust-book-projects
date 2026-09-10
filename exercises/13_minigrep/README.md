# Chapter 13: Improving Our I/O Project — Exercises

1. Migrate `search_case_insensitive` in `projects/minigrep/src/lib.rs` to iterator adapters (`lines().filter(...).collect()`), as the book suggests.
2. Return `impl Iterator<Item = &'a str>` from the search functions instead of `Vec<&'a str>` (drop `collect`, update the tests), and make `run` compile again. Verify on a large file: matches must print as they are found, not after a full collect.
3. beyond the book: make `run` itself iterator-native -- print via a consuming adapter (`for_each`) instead of the for loop. The further frontier (not required): stream the file itself with `BufReader::lines()` instead of reading it whole.

Source: https://doc.rust-lang.org/stable/book/ch13-03-improving-our-io-project.html

From the book (the originals these stages build on):

> Feel free to make the same change to use iterator methods in the `search_case_insensitive` function as well.

> For a further improvement, return an iterator from the `search` function by removing the call to `collect` and changing the return type to `impl Iterator<Item = &'a str>` so that the function becomes an iterator adapter. Note that you'll also need to update the tests! Search through a large file using your `minigrep` tool before and after making this change to observe the difference in behavior.

The `run` requirement in stage 2 and all of stage 3 go beyond the book.
