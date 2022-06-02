# How to derive the traits

This article will explain how to enable a new type for use with the [float_eq!],
[float_ne!], [assert_float_eq!] and [assert_float_ne!] macros. However, deriving
the necessary traits is currently only possible if this type is a struct or
tuple struct and is not generic. If not, or if you do not wish to enable the
derive feature, see [How to manually implement the traits].

## Enabling the derive feature

Enable the optional "derive" feature in your Cargo.toml:

```toml
[dependencies.float_eq]
version = "1"
features = ["derive"]
```

## Deriving the required traits

Add [`#[derive_float_eq]`](../../doc/float_eq/attr.derive_float_eq.html) to the
new type. For example:

```rust
#[derive_float_eq(
    ulps_tol = "PointUlps", 
    ulps_tol_derive = "Clone, Copy, Debug, PartialEq",
    debug_ulps_diff = "PointDebugUlpsDiff",
    debug_ulps_diff_derive = "Clone, Copy, Debug, PartialEq"
)]
#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}
```

The parameters:
- `ulps_tol`: required, will name a new type to provide per-field [ULPs] tolerances.
- `ulps_tol_derive`: optional, provides a list of traits to derive on the `ulps_tol` type.
- `debug_ulps_diff`: required, will name a new type used to display per-field [ULPs] differences.
- `debug_ulps_diff_derive`: optional, provides a list of traits to derive on the `debug_ulps_diff` type.

This will implement two new types:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct PointUlps {
    x: UlpsTol<f64>,
    y: UlpsTol<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PointDebugUlpsDiff {
    x: DebugUlpsDiff<f64>,
    y: DebugUlpsDiff<f64>,
}
```

This will also implement [FloatEqUlpsTol], [FloatEq], [FloatEqDebugUlpsDiff] and
[AssertFloatEq] for your type. You may now compare it as a composite type:

```rust
let a = Point { x: 1.0, y: -2.0 };
let b = Point { x: 1.1, y: -2.2 };
let c = Point { x: 1.000_000_000_000_000_9, y: -2.000_000_000_000_001_3 };
let eps = f64::EPSILON;

assert_float_eq!(a, b, abs <= Point { x: 0.15, y: 0.25 });
assert_float_eq!(a, c, rmax <= Point { x: 4.0 * eps, y: 5.0 * eps });
assert_float_eq!(a, c, ulps <= PointUlps { x: 4, y: 3 });
```

## Enabling the `_all` variants of checks

If your type is homogeneous, that is if it consists of fields that are all the
same underlying floating point type, then you may provide the optional `all_tol`
parameter to `#[derive_float_eq]` with that underlying type (usually `f32` or
`f64`). This will enable the `_all` variants of checks for it:

```rust
#[derive_float_eq(
    ulps_tol = "PointUlps", 
    ulps_tol_derive = "Clone, Copy, Debug, PartialEq",
    debug_ulps_diff = "PointDebugUlpsDiff",
    debug_ulps_diflf_derive = "Clone, Copy, Debug, PartialEq",
    all_tol = "f64"
)]
#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}
```

This will additionally implement the [FloatEqAll] and [AssertFloatEqAll] for
your type. You may now compare it using the `_all` check variants:

```rust
let a = Point { x: 1.0, y: -2.0 };
let b = Point { x: 1.1, y: -2.2 };
let c = Point { x: 1.000_000_000_000_000_9, y: -2.000_000_000_000_001_3 };
let eps = f64::EPSILON;

assert_float_eq!(a, b, abs_all <= 0.25);
assert_float_eq!(a, c, rmax_all <= 5.0 * eps);
assert_float_eq!(a, c, ulps_all <= 4);
```

## Deriving individual traits

The `#[derive_float_eq]` macro is recommended but if required you may implement
the traits by individually deriving them. The `#[float_eq]` attribute should be
used to pass through the relevant parameters. For example:

```rust
#[derive(
    Debug, PartialEq, Clone, Copy, FloatEqUlpsTol, FloatEq,
    FloatEqDebugUlpsDiff, AssertFloatEq
)]
#[float_eq(
    ulps_tol = "PointUlps", 
    ulps_tol_derive = "Clone, Copy, Debug, PartialEq",
    debug_ulps_diff = "PointDebugUlpsDiff",
    debug_ulps_diff_derive = "Clone, Copy, Debug, PartialEq"
)]
struct Point {
    x: f64,
    y: f64,
}
```

The [float_eq!] and [float_ne!] macros require [FloatEqUlpsTol] and [FloatEq]
and may optionally use [FloatEqAll]. Likewise, [assert_float_eq!] and 
[assert_float_ne!] require [FloatEqDebugUlpsDiff] and [AssertFloatEq] and may
optionally use [AssertFloatEqAll].

| Trait                  | Requires                      | Parameters                                  |
|------------------------|-------------------------------|---------------------------------------------|
| [FloatEqUlpsTol]       |                               | `ulps_tol`, `ulps_tol_derive`               |
| [FloatEq]              | FloatEqUlpsTol                |                                             |
| [FloatEqAll]           | FloatEq                       | `all_tol`                                   |
| [FloatEqDebugUlpsDiff] |                               | `debug_ulps_diff`, `debug_ulps_diff_derive` | 
| [AssertFloatEq]        | FloatEq, FloatEqDebugUlpsDiff |                                             |
| [AssertFloatEqAll]     | AssertFloatEq, FloatEqAll     | `all_tol`                                   |

[float_eq!]: ../../doc/float_eq/macro.float_eq.html
[float_ne!]: ../../doc/float_eq/macro.float_ne.html
[assert_float_eq!]: ../../doc/float_eq/macro.assert_float_eq.html
[assert_float_ne!]: ../../doc/float_eq/macro.assert_float_ne.html
[AssertFloatEq]: ../../doc/float_eq/trait.AssertFloatEq.html
[AssertFloatEqAll]: ../../doc/float_eq/trait.AssertFloatEqAll.html
[FloatEq]: ../../doc/float_eq/trait.FloatEq.html
[FloatEqAll]: ../../doc/float_eq/trait.FloatEqAll.html
[FloatEqDebugUlpsDiff]: ../../doc/float_eq/trait.FloatEqDebugUlpsDiff.html
[FloatEqUlpsTol]: ../../doc/float_eq/trait.FloatEqUlpsTol.html
[How to manually implement the traits]: ./manually_implement_the_traits.md
[ULPs]: ../background/float_comparison_algorithms.md#units-in-the-last-place-ulps-comparison