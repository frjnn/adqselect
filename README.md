# floydrivest

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Gethseman](https://circleci.com/gh/Gethseman/floydrivest.svg?style=shield)](https://app.circleci.com/pipelines/github/Gethseman/adqselect)
[![codecov](https://codecov.io/gh/Gethseman/floydrivest/branch/master/graph/badge.svg)](https://codecov.io/gh/Gethseman/adqlselect)

A lightweight crate that brings to Rust an `nth_element` implementation that leverages Andrei Alexandrescu's __adaptive quickselect__ algorithm. Also available on [crates.io](https://crates.io/crates/adqselect).

## Installation 

Be sure that your `Cargo.toml` looks somewhat like this:
```toml
[dependencies]
adqselect = "0.1.0"
```
## Usage

Bring the crate into scope:

```rust
extern crate adqselect;

use adqselect::nth_element;
```
then  simply call `nth_element` on a vector.

```rust
let mut v = vec![10, 7, 9, 7, 2, 8, 8, 1, 9, 4];
nth_element(&mut v, 3, &mut Ord::cmp);

assert_eq!(v[3], 7);
```

This implementation also handles generic data types as long as they satisfy the `PartialEq` and `PartialOrd` traits.

## Implementation

Link to the [original paper: Fast Deterministic Selection](https://arxiv.org/abs/1606.00484) by Andrei Alexandrescu.

## Performance

The algorithm is based on a refined version of Median of Medians and it guarantees linear deterministic time complexity.

## Benchmarks

Here are some benchmarks against other crates: [floydrivest](https://crates.io/crates/floydrivest), [order-stat](https://crates.io/crates/order-stat), [kth](https://crates.io/crates/kth) and [pdqlselect](https://crates.io/crates/pdqselect).
