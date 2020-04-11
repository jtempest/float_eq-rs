# float_eq

[![crate](https://img.shields.io/crates/v/float_eq.svg)](https://crates.io/crates/float_eq)
[![documentation](https://docs.rs/float_eq/badge.svg)](https://docs.rs/float_eq)
[![Travis status](https://travis-ci.com/jtempest/float_eq-rs.svg?branch=master)](https://travis-ci.com/github/jtempest/float_eq-rs)
[![Coverage Status](https://coveralls.io/repos/github/jtempest/float_eq-rs/badge.svg?branch=master)](https://coveralls.io/github/jtempest/float_eq-rs?branch=master)

Explicit and deliberate comparison of IEEE floating point numbers.

Comparing floating point values for equality is *really hard*. To get it
right requires careful thought and iteration based on the needs of each
specific algorithm's inputs and error margins. This API provides a toolbox
of components to make your options clear and your choices explicit to
future maintainers.

## Background

Given how widely algorithmic requirements can vary, `float_eq` explores the
idea that there are no generally sensible default margins for comparisons.
This is in contrast to the approach taken by many existing crates, which often
provide default epsilon values in checks or implicitly favour particular
algorithms. The author's hope is that by exposing the inherent complexity in
a uniform way, programmers will find it easier to develop an intuition for how
to write effective comparisons. The trade-off is that each individual
comparison requires more iteration time and thought.

And yes, this is yet another crate built on the principles described in *that*
Random ASCII [floating point comparison] article, which is highly recommended
background reading 🙂.

## Usage

Add this to your cargo.toml:

```
[dependencies]
float_eq = "0.2"
```

and, if you're using the 2015 edition, this to your crate root:

```rust
extern crate float_eq;
```

then, you can import items with `use`:

```rust
use float_eq::{assert_float_eq, float_eq};
```

## Comparisons

This crate provides boolean comparison operations:

```rust
assert!(float_eq!(1000.0_f32, 1000.0002, ulps <= 4));

// f32::EPSILON.sqrt()
const ROUNDING_ERROR: f32 = 0.00034526698;
assert!(float_ne!(4.0_f32, 4.1, rel <= ROUNDING_ERROR));
```

And asserts:

```rust
// 1.5 * 2_f32.powi(-12), as per SSE intrinsics documentation
const RECIP_REL_EPSILON: f32 = 0.00036621094; 
let recip = 0.1_f32.recip();
assert_float_eq!(recip, 10.0, rel <= RECIP_REL_EPSILON);

assert_float_ne!(0.0_f32, 0.0001, abs <= 0.00005, ulps <= 4);
```

Arrays of compatible types are also supported, from size 0 to 32 (inclusive):

```rust
assert_float_eq!([1.0000001_f32, 2.0], [1.0, 2.0], ulps <= 1);
```

See the [API documentation] for a long form introduction to the different kinds
of checks, their uses and limitations. Comparison of new types is supported by 
implementing the `FloatEq` trait. Asserts may be supported by additionally 
implementing the `FloatDiff` and `FloatEqDebug` traits, which provide additional
debugging context info.

## Optional Features

This crate can be used without the standard library (`#![no_std]`) by disabling
the default `std` feature. Use this in `Cargo.toml`:

```
[dependencies.float_eq]
version = "0.2"
default-features = false
```

Other optional features:
- **num** — implements `FloatEq`, `FloatEqDebug` and `FloatDiff` for 
  `num::Complex` where it is instanced with a compatible type.

## Related efforts

There are a number of existing crates that implement these kinds of comparisons
if you're looking for a more mature solution or simply a different approach.
The [`approx`], [`float-cmp`] and [`almost`] crates all provide a similar style
of general comparison operations, whereas [`assert_float_eq`] focuses
specifically on assertions. In contrast, [`efloat`] comes at the problem from a
different angle, instead tracking the error bounds of values as operations are
applied.

## Contributing 

Constructive feedback, suggestions and contributions welcomed, please
[open an issue].

## Changelog

Release information is available in [CHANGELOG.md](CHANGELOG.md).

## Future plans

- `#[derive]` support for comparison of custom types that are composed of 
  already comparable floating point values.

- Further support for basic Rust language components like tuples and containers
  of compatible types like `Vec`, likely using `PartialEq`'s support as a guide.

- Investigate the safety guarantees of the ulps check. Currently, it doesn't
  act like the default floating point checks when it comes to NaNs and other
  special values.

- More exhaustive testing. Tests currently cover all basic functionality, but
  there are lots of edge cases that aren't being tested yet.

- Benchmark performance, especially the implications of chaining multiple tests.

[API documentation]: https://docs.rs/float_eq
[floating point comparison]: https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
[open an issue]: https://github.com/jtempest/float_eq-rs/issues/new
[`almost`]: https://crates.io/crates/almost
[`approx`]: https://crates.io/crates/approx
[`assert_float_eq`]: https://crates.io/crates/assert_float_eq
[`efloat`]: https://crates.io/crates/efloat
[`float-cmp`]: https://crates.io/crates/float-cmp
