<!--
    THIS FILE IS GENERATED FROM crates-io.md AND LICENSE.md.
    DO NOT EDIT IT DIRECTLY.
-->

# float_eq

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
float_eq = "0.1"
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

See the [API documentation] for a long form introduction to the different kinds
of checks, their uses and limitations. Comparison of new types is supported by 
implementing the `FloatEq` and `FloatDiff` traits.

## Features

This crate can be used without the standard library (`#![no_std]`) by disabling
the default `std` feature. Use this in `Cargo.toml`:

```
[dependencies.float_eq]
version = "0.1"
default-features = false
```

## Related efforts

There are a number of existing crates that implement these kinds of comparisons
if you're looking for a more mature solution or simply a different approach.
The [`approx`], [`float-cmp`] and [`almost`] crates all provide a similar style
of general comparison operations, whereas [`assert_float_eq`] focuses
specifically on assertions. In contrast, [`efloat`] comes at the problem from a
different angle, instead tracking the error bounds of values as operations are
applied.

## Changelog

Release information is available in [CHANGELOG.md](CHANGELOG.md).

## Future plans

- `#[derive]` support for comparison of custom types that are composed of 
  already comparable floating point values.

- Support for `num::Complex`.

- Investigate the safety guarantees of the ulps check. Currently, it doesn't
  act like the default floating point checks when it comes to NaNs and other
  special values.

- More exhaustive testing. Tests currently cover all basic functionality, but
  there are lots of edge cases that aren't being tested yet.

- Benchmark performance, especially the implications of chaining multiple tests.

## Contributing 

Constructive feedback, suggestions and contributions welcomed, please
[open an issue].

[API documentation]: https://docs.rs/float_eq
[floating point comparison]: https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
[open an issue]: https://github.com/jtempest/float_eq-rs/issues/new
[`almost`]: https://crates.io/crates/almost
[`approx`]: https://crates.io/crates/approx
[`assert_float_eq`]: https://crates.io/crates/assert_float_eq
[`efloat`]: https://crates.io/crates/efloat
[`float-cmp`]: https://crates.io/crates/float-cmp

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in float_eq by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>