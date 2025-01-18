# BayesTest Rust

Bayesian A/B testing calculations for Rust

Based on [this post](https://www.evanmiller.org/bayesian-ab-testing.html) by Evan Miller

Also available in [C++](https://github.com/ankane/bayestest-cpp)

[![Build Status](https://github.com/ankane/bayestest-rust/actions/workflows/build.yml/badge.svg)](https://github.com/ankane/bayestest-rust/actions)

## Installation

Add this line to your application’s `Cargo.toml` under `[dependencies]`:

```toml
bayestest = "0.2"
```

## Getting Started

- [Binary outcomes](#binary-outcomes), like conversion rate
- [Count data](#count-data), like number of sales per salesperson

## Binary Outcomes

```rust
use bayestest::BinaryTest;

let mut test = BinaryTest::new();
test.add(participants_a, conversions_a);
test.add(participants_b, conversions_b);
let probabilities = test.probabilities();
```

Supports up to 4 variants

## Count Data

```rust
use bayestest::CountTest;

let mut test = CountTest::new();
test.add(events_a, exposure_a);
test.add(events_b, exposure_b);
let probabilities = test.probabilities();
```

Supports up to 3 variants

## History

View the [changelog](https://github.com/ankane/bayestest-rust/blob/master/CHANGELOG.md)

## Contributing

Everyone is encouraged to help improve this project. Here are a few ways you can help:

- [Report bugs](https://github.com/ankane/bayestest-rust/issues)
- Fix bugs and [submit pull requests](https://github.com/ankane/bayestest-rust/pulls)
- Write, clarify, or fix documentation
- Suggest or add new features

To get started with development:

```sh
git clone https://github.com/ankane/bayestest-rust.git
cd bayestest-rust
cargo test
```
