# cargo test cheatsheet (chapter 11.2)

All outputs below are examples captured from this project's suite
(test names, module names and counts reflect one particular run).

## Command anatomy

```sh
cargo test [cargo-options] -- [test-binary-options]
cargo test --help      # cargo's own options
cargo test -- --help   # the test binary's options (after --)
```

## Parallelism (default: parallel threads)

Tests must not depend on each other or on shared state (working
directory, environment variables) -- the book's example: parallel
tests writing one shared `test-output.txt` file interfere.

```sh
cargo test -- --test-threads=1
```

## Output capture

By default the stdout of PASSING tests is captured (hidden); a
failing test's stdout appears in its failure panel. To see passing
tests' output as well:

```sh
cargo test -- --show-output
```

```
successes:

---- adder::tests::prints_and_passes stdout ----
I got the value 4
```

## Running a subset by name

The module path is part of the test name, so filtering works on
test names, substrings, AND module names. Only one filter argument
is used; the summary reports how many tests were filtered out.

```sh
cargo test it_adds_two   # exact name
```

```
running 1 test
test adder::tests::it_adds_two ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out
```

```sh
cargo test hold          # substring: every matching test
```

```
running 2 tests
test rectangle::tests::larger_can_hold_smaller ... ok
test rectangle::tests::smaller_cannot_hold_larger ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out
```

```sh
cargo test adder         # module name filters its tests
```

```
running 4 tests
test adder::tests::exploration ... ok
test adder::tests::it_adds_two ... ok
test adder::tests::it_works ... ok
test adder::tests::prints_and_passes ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out
```

## Ignoring tests (#[ignore])

`#[ignore]` after `#[test]` excludes the test from the default run.
The book's motivation is expensive tests; in practice it also
quarantines flaky or broken tests and skips tests needing external
resources (run those separately in CI via `-- --ignored`).

```
test adder::tests::expensive_test ... ignored
test result: ok. 8 passed; 0 failed; 1 ignored; ...
```

```sh
cargo test -- --ignored          # only the ignored tests
```

```
running 1 test
test adder::tests::expensive_test ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out
```

```sh
cargo test -- --include-ignored  # everything
```

```
test result: ok. 9 passed; 0 failed; 0 ignored; ...
```
