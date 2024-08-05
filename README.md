# Matrix

A skeleton of simple matrix implementation for practice.

## Introduction

Open `src/matrix/ops.rs` and implement basic operations for matrix. The Matrix struct and macros for creating it are already defined. The following ops need to be implemented: **add, sub, scalar multiplication, matrix multiplication, transpose**.

When you are done, run `cargo test`. 8 tests from `test_ops` should pass.

You are free to add anything to `src/matrix/ops.rs` or add modules, but please do not remove or modify implementations in `src/lib.rs`, `src/matrix/mod.rs`, or `src/matrix/example.rs`. Especially, **DO NOT** modify tests in `lib.rs`. But you can add some utils or tests to `mod.rs` if you really need.

## Help

A working implementation as an example is in `example.rs`. If you want to run tests for `example.rs`, comment `mod ops;` and uncomment `mod example;` in `src/matrix/mod.rs`, and then `cargo test`.

That implementation is just my own implementation. It may have an error or inefficiency. If there is any problem, please note me by email or submit an issue to the github repo.

## Usage

### Create

```rust
use matrix::*;

// Convert from Vec<Vec<i32>>
let m1: Matrix = vec![
    vec![0, 0, 0],
    vec![0, 0, 0]
].into();

// Macro 1
let m2 = m![0; 2, 3];

// Macro 2
let m3 = m![
    0, 0, 0;
    0, 0, 0;
];

assert_eq!(m1, m2);
assert_eq!(m1, m3);
```

### Operations (should be implemented)

```rust
use matrix::*;

// Add
let m = m![1, 2, 3; 4, 5, 6] + m![2, 2, 2; 2, 2, 2];
assert_eq!(m, m![3, 4, 5; 6, 7, 8]);

// Sub
let m = m![9, 9; 9, 9] - m![1, 2; 3, 4];
assert_eq!(m, m![8, 7; 6, 5]);

// Scalar multiplication
let m = m![1, 2; 3, 4].scalar_mul(3);
assert_eq!(m, m![3, 6; 9, 12]);

// Multiplication
let m = m![1, 2, 3; 4, 5, 6] * m![2, 2, 2; 2, 2, 2; 2, 2, 2];
assert_eq!(m, m![12, 12, 12; 30, 30, 30]);

// Transpose
let m = m![1, 2, 3; 4, 5, 6].transpose();
assert_eq!(m, m![1, 4; 2, 5; 3, 6]);
```
