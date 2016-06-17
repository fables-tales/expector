# Expector
> A Matcher Library for Rust

## Usage

1. In your `Cargo.toml` add `expector=0.1.0`
2. `cargo build`
3. `extern crate expector`
4. `use expector::prelude::*`

Example:
```
let a = 5;
let b = 3;
expect(a).to(eq(b));
//panic "Expector match failed"
```

## Ontology

- `expectation`: a target and a matcher

## Structure

- `expector`: structs and traits for expectations
- `expector::prelude`: user facing API for expector crate
- `expector::matchers`: structs and traits for matchers
- `expector::matchers::prelude`: user facing API for matchers,
  required by `expector::prelude`

## Local Development

### Prerequisites

This is a crate built in and for Rust projects. to use this project,
you'll need to install [rust]. [rustup] is the preferred way to
install Rust.

[rust]: https://www.rust-lang.org/
[rustup]: https://www.rustup.rs/

### Up and Running

1. Fork and clone this repository
2. `cd expector`
3. `cargo build`
