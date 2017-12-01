# Advent of Code 2017 solutions

Solutions to the 2017 edition of [Advent of Code][advent], as an exercise in 
learning [Rust][rust].

To run the solutions, [install rust][rustinstall] and then build the 
executables.

```shell
# In the repository
$ cargo build
   Compiling adventofcode_2017 v0.1.0 (file:///home/rustacean/adventofcode_2017)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30 secs
$./target/debug/day1
Puzzle #1: …
Puzzle #2: …
```

Or you can run a particular day directly.

```shell
$ cargo run day1
   Compiling adventofcode_2017 v0.1.0 (file:///home/rustacean/Projects/adventofcode/adventofcode_2017)
Finished dev [unoptimized + debuginfo] target(s) in 0.30 secs
 Running `target/debug/day1 day1`
Puzzle #1: …
Puzzle #2: …
```

To run the tests:

```shell
$ cargo test
   Compiling adventofcode_2017 v0.1.0 (file:///home/rustacean/Projects/adventofcode/adventofcode_2017)
Finished dev [unoptimized + debuginfo] target(s) in 0.39 secs
 Running target/debug/deps/day1-100ceb28cd9a31d5

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

 Running target/debug/deps/day1-d40a5742dc301053

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Doc-tests day1

running 1 test
test src/day1/lib.rs - sequence_sum (line 13) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The documentation can be generated with `cargo doc`.

[advent]: http://adventofcode.com/2017/day/1
[rust]: https://www.rust-lang.org/
[rustinstall]: https://www.rust-lang.org/en-US/install.html
