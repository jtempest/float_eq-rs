# float_eq

Compare IEEE floating point primitives, structs and collections for equality.

This crate provides an API with a focus on making the choices of comparison 
algorithm(s) and tolerances intuitive to implementers and maintainers, and of
providing clear output for debugging and development iteration.

This readme is a quick tour of the crate. For introductory material, guides and
discussion see [the float_eq guide].

## Usage

Add this to your cargo.toml:

```
[dependencies]
float_eq = "1"
```

And, if you're using the 2015 edition, this to your crate root:

```rust
extern crate float_eq;
```

Then, you can import items with `use`:

```rust
use float_eq::{assert_float_eq, float_eq};
```

## Comparisons

This crate provides boolean comparison operations:

```rust
if (float_eq!(y_pos, 0.0, abs <= 0.000_1)) {
    //...
}
```

And asserts:

```rust
const RECIP_REL_TOL: f32 = 0.000_366_210_94;
assert_float_eq!(x.recip(), 10.0, r2nd <= RECIP_REL_TOL);
```

Using absolute tolerance, relative tolerance or ULPs based [comparison
algorithms].

## Composite types

Composite types may implement the provided extension traits to be compared on a
field-by-field basis:

```rust
let a = Complex32 { re: 2.0, im: 4.000_002 };
let b = Complex32 { re: 2.000_000_5, im: 4.0 };

assert_float_eq!(a, b, ulps <= ComplexUlps32 { re: 2, im: 4 });
```

...and if they are homogeneous, with a uniformly applied tolerance across all
fields:

```rust
assert_float_eq!(a, b, ulps_all <= 4);
```

Arrays of any size are supported:

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
assert_float_eq!(a, b, r2nd <= (0.5, 2.0));
```

Many standard and core types like `Vec` are supported:

```rust
let a = vec![1.0, -2.0, 3.0];
let b = vec![-1.0, 2.0, 3.5];
assert_float_eq!(a, b, rmax <= vec![2.0, 2.0, 0.25]);
assert_float_eq!(a, b, rmax_all <= 2.0);
```

There are blanket trait impls for comparing mutable and immutable reference
types, the contents of `Cell`, `RefCell`, `Rc`, `Arc` and `Box` instances, as
well as for slices, `Option`, `Vec`, `VecDeque`, `LinkedList`, `BTreeMap` and
`HashMap`.

## Derivable

The extension traits may be derived for non-generic structs and tuple structs:

```rust
#[derive_float_eq(
    ulps_tol = "PointUlps",
    ulps_tol_derive = "Clone, Copy, Debug, PartialEq",
    debug_ulps_diff = "PointUlpsDebugUlpsDiff",
    debug_ulps_diff_derive = "Clone, Copy, Debug, PartialEq",
    all_tol = "f64"
)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
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

Asserts provide additional useful context information. For example:

```rust
assert_float_eq!(4.0f32, 4.000_008, rmax <= 0.000_001);
```

Panics with this error message:

```
thread 'main' panicked at 'assertion failed: `float_eq!(left, right, rmax <= t)`
        left: `4.0`,
       right: `4.000008`,
    abs_diff: `0.000008106232`,
   ulps_diff: `Some(17)`,
    [rmax] t: `0.000004000008`', assert_failure.rs:15:5
```

Where `[rmax] t` shows the tolerance value that the absolute difference was
compared against after being appropriately scaled.

## Optional features

This crate can be used without the standard library (`#![no_std]`) by disabling
the default `std` feature. Use this in `Cargo.toml`:

```
[dependencies.float_eq]
version = "1"
default-features = false
```

Other optional features:
- **derive** — provides custom derive macros for all traits.
- **num** — blanket trait impls for `num::Complex` where it is instanced with a
  compatible type.

## Related efforts

The [`approx`], [`float-cmp`], [`assert_float_eq`] and [`is_close`] crates provide
similar floating point comparison capabilities to `float_eq`. The [`almost`] crate
divides its API into comparison of floats against zero and non-zero values. The
[`efloat`] crate provides an `f32` equivalent type that tracks the maximum
possible error bounds that may have occured due to rounding.

The [`ieee754`] crate is not a comparison library, but provides useful
functionality for decomposing floats into their component parts, iterating over
representable values and working with ULPs directly, amoung other things.

## Contributing 

Constructive feedback, suggestions and contributions welcomed, please
[open an issue].

## Changelog

Release information is available in [CHANGELOG.md](CHANGELOG.md).

[comparison algorithms]: https://jtempest.github.io/float_eq-rs/book/background/float_comparison_algorithms.html
[open an issue]: https://github.com/jtempest/float_eq-rs/issues/
[the float_eq guide]: https://jtempest.github.io/float_eq-rs/book/introduction.html
[`almost`]: https://crates.io/crates/almost
[`approx`]: https://crates.io/crates/approx
[`assert_float_eq`]: https://crates.io/crates/assert_float_eq
[`efloat`]: https://crates.io/crates/efloat
[`float-cmp`]: https://crates.io/crates/float-cmp
[`ieee754`]: https://crates.io/crates/ieee754
[`is_close`]: https://docs.rs/is_close/latest/is_close/
