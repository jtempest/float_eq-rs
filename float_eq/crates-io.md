# float_eq

[![crate](https://img.shields.io/crates/v/float_eq.svg)](https://crates.io/crates/float_eq)
[![documentation](https://docs.rs/float_eq/badge.svg)](https://docs.rs/float_eq)
[![Travis status](https://travis-ci.com/jtempest/float_eq-rs.svg?branch=master)](https://travis-ci.com/github/jtempest/float_eq-rs)
[![Coverage Status](https://coveralls.io/repos/github/jtempest/float_eq-rs/badge.svg?branch=master)](https://coveralls.io/github/jtempest/float_eq-rs?branch=master)

Compare IEEE floating point values for equality.

Comparing floating point values for equality is notoriously difficult,
getting it right requires careful reasoning and iteration. This API provides
a variety of comparison algorithms and debugging tools to help make the
process more intuitive and your choices explicit and clear to future
maintainers.

# Background

Given how widely algorithmic requirements can vary, `float_eq` explores the
idea that there are no generally sensible default margins for comparisons.
This is in contrast to the approach taken by many other crates, which often
provide default epsilon values in checks or implicitly favour particular
algorithms. The author's hope is that by exposing the inherent complexity
in a uniform way, programmers will find it easier to develop an intuition
for effective use of floats.

This work builds on the definitions in Knuth's The Art Of Computer Programming,
(Vol. 2, Seminumerical Algorithms, Third Edition, section 4.2.2), and *that* 
Random ASCII article on [floating point comparison].

## Usage

Add this to your cargo.toml:

```
[dependencies]
float_eq = "0.5"
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
assert!(float_ne!(4.0f32, 4.1, rmax <= ROUNDING_ERROR));
```

And asserts:

```rust
const RECIP_REL_EPSILON: f32 = 0.000_366_210_94; 
assert_float_eq!(0.1f32.recip(), 10.0, r2nd <= RECIP_REL_EPSILON);

assert_float_ne!(0.0f32, 0.000_1, abs <= 0.000_05, ulps <= 4);
```

Checks are invoked by name and with a threshold, so for example `abs <= 0.000_05`
should be read as *"an absolute epsilon comparison with a maximum difference of
less than or equal to `0.000_05`"*. Similarly, `rmax`, `rmin`, `r1st` and `r2nd`
provide a variety of kinds of relative epsilon comparison with thresholds that
scale to the granularity of one or input value or the other and `ulps` is an
ULPs based comparison that takes advantage of the underlying bitwise 
representation. See the [API documentation] for a long form introduction
to the different kinds of checks, their uses and limitations.

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
basis, and types that implement `FloatEqAll` may be compared with a uniformly
applied epsilon value across all fields:

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
assert_float_eq!(4.0f32, 4.000_008, rmax <= 0.000_001);
```

Panics with this error message:

```
thread 'main' panicked at 'assertion failed: `float_eq!(left, right, rmax <= ε)`
        left: `4.0`,
       right: `4.000008`,
    abs_diff: `0.000008106232`,
   ulps_diff: `Some(17)`,
    [rmax] ε: `0.000004000008`', assert_failure.rs:15:5
```

The message shows the values of the expressions being compared and the
difference between them both in absolute terms and in terms of ULPs. The 
`[rmax] ε` line shows the epsilon value that the absolute difference was
compared against after being appropriately scaled.

## Optional features

This crate can be used without the standard library (`#![no_std]`) by disabling
the default `std` feature. Use this in `Cargo.toml`:

```
[dependencies.float_eq]
version = "0.5"
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

[API documentation]: https://docs.rs/float_eq
[floating point comparison]: https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
[open an issue]: https://github.com/jtempest/float_eq-rs/issues/
[`almost`]: https://crates.io/crates/almost
[`approx`]: https://crates.io/crates/approx
[`assert_float_eq`]: https://crates.io/crates/assert_float_eq
[`efloat`]: https://crates.io/crates/efloat
[`float-cmp`]: https://crates.io/crates/float-cmp
