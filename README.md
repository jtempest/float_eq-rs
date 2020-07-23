<!--
    THIS FILE IS GENERATED FROM crates-io.md AND LICENSE.md.
    DO NOT EDIT IT DIRECTLY.
-->

# float_eq

[![crate](https://img.shields.io/crates/v/float_eq.svg)](https://crates.io/crates/float_eq)
[![documentation](https://docs.rs/float_eq/badge.svg)](https://docs.rs/float_eq)
[![Travis status](https://travis-ci.com/jtempest/float_eq-rs.svg?branch=master)](https://travis-ci.com/github/jtempest/float_eq-rs)
[![Coverage Status](https://coveralls.io/repos/github/jtempest/float_eq-rs/badge.svg?branch=master)](https://coveralls.io/github/jtempest/float_eq-rs?branch=master)

Explicitly bounded comparison of floating point numbers.

Comparing floating point values for equality is *really hard*. To get it
right requires careful thought and iteration based on the needs of each
specific algorithm's inputs and error margins. This API provides a toolbox
of components to make your options clear and your choices explicit to
future maintainers.

## Background

Given how widely algorithmic requirements can vary, `float_eq` explores the
idea that there are no generally sensible default margins for comparisons.
This is in contrast to the approach taken by many other crates, which often
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
float_eq = "0.4"
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
assert!(float_eq!(1000.0f32, 1000.0002, ulps <= 4));

const ROUNDING_ERROR: f32 = 0.000_345_266_98;
assert!(float_ne!(4.0f32, 4.1, rel <= ROUNDING_ERROR));
```

And asserts:

```rust
const RECIP_REL_EPSILON: f32 = 0.000_366_210_94; 
assert_float_eq!(0.1f32.recip(), 10.0, rel <= RECIP_REL_EPSILON);

assert_float_ne!(0.0f32, 0.000_1, abs <= 0.000_05, ulps <= 4);
```

Where `rel <= ROUNDING_ERROR` should be read as *"a relative epsilon comparison
with a maximum difference of less than or equal to `ROUNDING_ERROR`"*, and
similarly for `abs` and `ulps`. See the [API documentation] for a long form
introduction to the different kinds of checks, their uses and limitations.

## Combining checks

If more than one check is specified by a comparison then they are performed
in order from left to right. If any check is true, then the two values are
considered equal. For example, this expression:

```rust
float_eq!(a, b, abs <= 0.000_01, ulps <= 4)
```

Is equivalent to:

```rust
float_eq!(a, b, abs <= 0.000_01) || float_eq!(a, b, ulps <= 4)
```

This allows you to build comparison expressions as needed, only paying for what
you use.

## Composite types

Composite types that implement `FloatEq` may be compared on a field-by-field
basis using the `abs`, `rel`, and `ulps` comparisons, and types that implement
`FloatEqAll` may be compared with a uniformly applied epsilon value using the
`abs_all`, `rel_all` and `ulps_all` variants:

```rust
let a = Complex32 { re: 2.0, im: 4.000_002 };
let b = Complex32 { re: 2.000_000_5, im: 4.0 };

assert_float_eq!(a, b, ulps <= Complex32Ulps { re: 2, im: 4 });
assert_float_eq!(a, b, ulps_all <= 4);
```

Arrays of size 0 to 32 (inclusive) are supported:

```rust
let a = [1.0, -2.0, 3.0];
let b = [-1.0, 2.0, 3.5];
assert_float_eq!(a, b, abs <= [2.0, 4.0, 0.5]);
assert_float_eq!(a, b, abs_all <= 4.0);
```

As are tuples up to size 12 (inclusive):

```rust
let a = (1.0f32, 2.0f64);
let b = (1.5f32, -2.0f64);
assert_float_eq!(a, b, abs <= (0.5, 4.0));
```

There are also blanket trait impls for comparing mutable and immutable reference
types, the contents of `Cell`, `RefCell`, `Rc`, `Arc` and `Box` instances, as
well as for slices, `Option`, `Vec`, `VecDeque`, `LinkedList`, `BTreeMap` and
`HashMap`.

Types that also implement `AssertFloatEq`/`AssertFloatEqAll` may be used in the
assert forms.

## Derivable

If the optional `"derive"` feature is enabled, all of the traits may be 
implemented using `#[derive]`. The easiest way to do so is to make use of the 
`#[derive_float_eq]` helper macro:

```rust
#[derive_float_eq(
    ulps_epsilon = "PointUlps",
    debug_ulps_diff = "PointUlpsDebugUlpsDiff",
    all_epsilon = "f64"
)]
#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

let a = Point { x: 1.0, y: -2.0 };
let c = Point { 
    x: 1.000_000_000_000_000_9, 
    y: -2.000_000_000_000_001_3
};
assert_float_eq!(a, c, ulps <= PointUlps { x: 4, y: 3 });
assert_float_eq!(a, c, ulps_all <= 4);
```

## Error messages

Assertion failure output tries to provide useful context information without
going overboard. For example, this call:

```rust
assert_float_eq!(4.0f32, 4.000_008, rel <= 0.000_001);
```

Panics with this error message:

```
thread 'check' panicked at 'assertion failed: `float_eq!(left, right, rel <= ε)`
        left: `4.0`,
       right: `4.000008`,
    abs_diff: `0.000008106232`,
   ulps_diff: `Some(17)`,
     [rel] ε: `0.000004000008`', assert_failure.rs:15:5
```

The message provides `abs_diff` and `ulps_diff` regardless of which kinds of
checks are chosen. The `[rel] ε` line gives the epsilon value that `abs_diff` is
checked against in the comparison, which has been scaled based on the size of
the inputs. Absolute epsilon and ULPs based checks would provide different
output, see the [API documentation] for more details.

## Optional features

This crate can be used without the standard library (`#![no_std]`) by disabling
the default `std` feature. Use this in `Cargo.toml`:

```
[dependencies.float_eq]
version = "0.4"
default-features = false
```

Other optional features:
- **derive** — provides custom derive macros for all traits.
- **num** — blanket trait impls for `num::Complex` where it is instanced with a
  compatible type.

## Related efforts

The [`approx`] and [`float-cmp`] crates provide a similar style of general
comparison operations, whereas [`assert_float_eq`] focuses specifically on
assertions. The [`almost`] crate instead divides its API into algorithms 
comparing against zero and non-zero values. In contrast, [`efloat`] takes the
approach of tracking the error bounds of values as operations are applied.

## Contributing 

Constructive feedback, suggestions and contributions welcomed, please
[open an issue].

## Changelog

Release information is available in [CHANGELOG.md](CHANGELOG.md).

## Future plans

- Checks that use a precision relative to the minimum of the two input values,
  or to the first or second operand.

- Investigate supporting `#[derive]` for enums and generic struct types.

[API documentation]: https://docs.rs/float_eq
[floating point comparison]: https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
[open an issue]: https://github.com/jtempest/float_eq-rs/issues/
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