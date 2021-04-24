# How to compare custom types

To extend `float_eq` functionality over a new type, you should implement the
relevant traits:

1) [float_eq!] requires [FloatEqUlpsEpsilon] and [FloatEq].

2) If your type is homogeneous, that is if it consists of fields that are all
the same underlying floating point type, you should implement the optional
[FloatEqAll] to enable the `_all` comparison algorithms.

3) [assert_float_eq!] requires the same traits plus [FloatEqDebugUlpsDiff]
and [AssertFloatEq]. If you have implemented [FloatEqAll] you should also
implement [AssertFloatEqAll]

If your type is a non-generic struct or tuple struct that consists entirely of
already supported fields, then the easiest way to implement these traits is to
make use of the `#[derive_float_eq]` helper macro. It is also possible to
`#[derive]` individual traits. If you cannot derive an implementation, then you
will need to implement the traits directly.

## #[derive_float_eq]

If your type is a non-generic struct or tuple struct then you may derive the
relevant traits using this helper macro. Enable the "derive" feature by adding
this to your Cargo.toml:

```toml
[dependencies.float_eq]
version = "0.5"
features = ["derive"]
```

Add [`#[derive_float_eq]`](../../doc/float_eq/attr.derive_float_eq.html) to your
type. The `ulps_epsilon` and `debug_ulps_diff` parameters are required. They are
used to name two new types that match the structure of the type being derived
from. The first is used to provide ULPs tolerance values per field, and the
second is used to provide debug information for the differerence between values
in ULPs.

The `all_epsilon` parameter is optional, and ought to be provided if your type
is homogeneous and consists of fields that are all the same underlying floating
point type. If provided, it will additionally implement the traits required to
use the `_all` variants of checks, using the given epsilon type (usually `f32`
or `f64`). 

```rust
#[derive_float_eq(
    ulps_epsilon = "PointUlps", 
    debug_ulps_diff = "PointDebugUlpsDiff",
    all_epsilon = "f64"
)]
#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}
```

This will generate the following two types in addition to implementing the
relevant extension traits. These new types will derive `Debug`, `Clone`, `Copy`
and `PartialEq`:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct PointUlps {
    x: UlpsEpsilon<f64>,
    y: UlpsEpsilon<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PointDebugUlpsDiff {
    x: DebugUlpsDiff<f64>,
    y: DebugUlpsDiff<f64>,
}
```

This enables the use of your type in the `float_eq` macros:

```rust
let a = Point { x: 1.0, y: -2.0 };
let b = Point { x: 1.1, y: -2.2 };

assert_float_eq!(a, b, abs <= Point { x: 0.15, y: 0.25 });
assert_float_eq!(a, b, abs_all <= 0.25);

let c = Point { x: 1.000_000_000_000_000_9, y: -2.000_000_000_000_001_3 };
let eps = f64::EPSILON;

assert_float_eq!(a, c, rmax <= Point { x: 4.0 * eps, y: 5.0 * eps });
assert_float_eq!(a, c, rmax_all <= 5.0 * eps);

assert_float_eq!(a, c, ulps <= PointUlps { x: 4, y: 3 });
assert_float_eq!(a, c, ulps_all <= 4);
```

## Manually #[derive] traits

If you cannot use `#[derive_float_eq]` then you may be able to derive individual
traits manually. Enable the "derive" feature by adding this to your Cargo.toml:

```toml
[dependencies.float_eq]
version = "0.5"
features = ["derive"]
```

### #[derive(UlpsEpsilon)]

Add a `#[float_eq]` attribute and provide `ulps_epsilon`, which will be used as
the name of a new type. This type will be structurally identical to the type
being derived, using the same visibility as the parent type and with identically
named fields that use the derived fields' types wrapped by `UlpsEpsilon`. The
new struct derives `Debug`, `Clone`, `Copy` and `PartialEq`.

```rust
#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsEpsilon)]
#[float_eq(ulps_epsilon = "PointUlps")]
struct Point {
    x: f64,
    y: f64,
}
```

This will generate the following struct:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct PointUlps {
    x: UlpsEpsilon<f64>,
    y: UlpsEpsilon<f64>,
}
```

### #[derive(FloatEq)]

Requires [FloatEqUlpsEpsilon]. Add a `#[float_eq]` attribute and provide
`ulps_epsilon`, which should match the name of the `UlpsEpsilon` type. Two
instances are equal if all fields are equal, and not equal if any are not.

```rust
#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsEpsilon, FloatEq)]
#[float_eq(ulps_epsilon = "PointUlps")]
struct Point {
    x: f64,
    y: f64,
}
```

### #[derive(FloatEqAll)]

Add a `#[float_eq]` attribute and specify `all_epsilon`, which is the type to be
used as [AllEpsilon], usually `f32` or `f64`. Two instances are equal if all
fields are equal, and not equal if any are not.

```
#[derive(Debug, Clone, Copy, PartialEq, FloatEqAll)]
#[float_eq(ulps_epsilon = "PointUlps", all_epsilon = "f64")]
struct Point {
    x: f64,
    y: f64,
}
```

### #[derive(FloatEqDebugUlpsDiff)]

Add a `#[float_eq]` attribute and provide `debug_ulps_diff`, which will be used
as the name of a new type. This type will be structurally identical to the type
being derived, using the same visibility as the parent type and with identically
named fields that use the derived fields' types wrapped by `DebugUlpsDiff`. The
new struct derives `Debug`, `Clone`, `Copy` and `PartialEq`.

```rust
#[derive(Debug, Clone, Copy, PartialEq, FloatEqDebugUlpsDiff)]
#[float_eq(debug_ulps_diff = "PointDebugUlpsDiff")]
struct Point {
    x: f64,
    y: f64,
}
```

This will generate the following struct:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct PointDebugUlpsDiff {
    x: DebugUlpsDiff<f64>,
    y: DebugUlpsDiff<f64>,
}
```

### #derive[(AssertFloatEq)]

Requires [FloatEqUlpsEpsilon], [FloatEq] and [FloatEqDebugUlpsDiff]. Add a
`#[float_eq]` attribute and provide `ulps_epsilon` and `ulps_debug_diff`, which
should match the name of the `UlpsEpsilon` and `DebugUlpsDiff` types. Each
field's epsilon is calculated via a recursive call to the algorithm being used.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(FloatEqUlpsEpsilon, FloatEq, FloatEqDebugUlpsDiff, AssertFloatEq)]
#[float_eq(ulps_epsilon = "PointUlps", debug_ulps_diff = "PointDebugUlpsDiff")]
struct Point {
    x: f64,
    y: f64,
}
```

### #[derive(AssertFloatEqAll)]

Requires [FloatEqUlpsEpsilon], [FloatEq], [FloatEqAll], [FloatEqDebugUlpsDiff]
and [AssertFloatEq]. Add a `#[float_eq]` attribute and provide `ulps_epsilon`,
`ulps_debug_diff`, and `all_epsilon`, which should match the names of the
`UlpsEpsilon`, `DebugUlpsDiff` and `AllEpsilon` types. Each field's epsilon is
calculated via a recursive call to the algorithm being used.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(
    FloatEqUlpsEpsilon, FloatEq, FloatEqAll,
    FloatEqDebugUlpsDiff, AssertFloatEq, AssertFloatEqAll
)]
#[float_eq(
    ulps_epsilon = "PointUlps",
    debug_ulps_diff = "PointUlpsDebugUlpsDiff",
    all_epsilon = "f64",
)]
struct Point {
    x: f64,
    y: f64,
}
```

## Implementing the traits directly

If you cannot derive the implementations of the extension traits, then you may
implement them manually. These implementations will be based on the same `Point`
type as the derive examples:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}
```

### Implementing FloatEqUlpsEpsilon

Types should provide an [UlpsEpsilon] representation for each of their fields:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct PointUlps {
    x: UlpsEpsilon<f64>,
    y: UlpsEpsilon<f64>,
}

impl FloatEqUlpsEpsilon for Point {
    type UlpsEpsilon = PointUlps;
}
```

### Implementing FloatEq

Requires [FloatEqUlpsEpsilon]. Implementation is then usually a matter of
calling through to an underlying [FloatEq] method for each field in turn. If
not, you will need to take a close look at the descriptions of the algorithms on
a method by method basis:

```rust
impl FloatEq for Point {
    type Epsilon = Point;

    fn eq_abs(&self, other: &Self, max_diff: &Point) -> bool {
        self.x.eq_abs(&other.x, &max_diff.x) &&
        self.y.eq_abs(&other.y, &max_diff.y)
    }

    fn eq_rmax(&self, other: &Self, max_diff: &Point) -> bool {
        self.x.eq_rmax(&other.x, &max_diff.x) &&
        self.y.eq_rmax(&other.y, &max_diff.y)
    }

    fn eq_rmin(&self, other: &Self, max_diff: &Point) -> bool {
        self.x.eq_rmin(&other.x, &max_diff.x) &&
        self.y.eq_rmin(&other.y, &max_diff.y)
    }

    fn eq_r1st(&self, other: &Self, max_diff: &Point) -> bool {
        self.x.eq_r1st(&other.x, &max_diff.x) &&
        self.y.eq_r1st(&other.y, &max_diff.y)
    }

    fn eq_r2nd(&self, other: &Self, max_diff: &Point) -> bool {
        self.x.eq_r2nd(&other.x, &max_diff.x) &&
        self.y.eq_r2nd(&other.y, &max_diff.y)
    }

    fn eq_ulps(&self, other: &Self, max_diff: &UlpsEpsilon<Point>) -> bool {
        self.x.eq_ulps(&other.x, &max_diff.x) &&
        self.y.eq_ulps(&other.y, &max_diff.y)
    }
}
```

### Implementing FloatEqAll

Select a tolerance type to compare recursively with each field in your type,
likely `f32` or `f64`. Implementation is then usually a matter of calling
through to an underlying [FloatEqAll] method for each field in turn. If not, you
will need to take a close look at the descriptions of the algorithms on a method
by method basis:

```rust
impl FloatEqAll for Point {
    type AllEpsilon = f64;

    fn eq_abs_all(&self, other: &Self, max_diff: &f64) -> bool {
        self.x.eq_abs_all(&other.x, max_diff) &&
        self.y.eq_abs_all(&other.y, max_diff)
    }

    fn eq_rmax_all(&self, other: &Self, max_diff: &f64) -> bool {
        self.x.eq_rmax_all(&other.x, max_diff) &&
        self.y.eq_rmax_all(&other.y, max_diff)
    }

    fn eq_rmin_all(&self, other: &Self, max_diff: &f64) -> bool {
        self.x.eq_rmin_all(&other.x, max_diff) &&
        self.y.eq_rmin_all(&other.y, max_diff)
    }

    fn eq_r1st_all(&self, other: &Self, max_diff: &f64) -> bool {
        self.x.eq_r1st_all(&other.x, max_diff) &&
        self.y.eq_r1st_all(&other.y, max_diff)
    }

    fn eq_r2nd_all(&self, other: &Self, max_diff: &f64) -> bool {
        self.x.eq_r2nd_all(&other.x, max_diff) &&
        self.y.eq_r2nd_all(&other.y, max_diff)
    }

    fn eq_ulps_all(&self, other: &Self, max_diff: &UlpsEpsilon<f64>) -> bool {
        self.x.eq_ulps_all(&other.x, max_diff) &&
        self.y.eq_ulps_all(&other.y, max_diff)
    }
}
```

### Implementing FloatEqDebugUlpsDiff

Types should provide a [DebugUlpsDiff] representation for each of their fields:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct PointDebugUlpsDiff {
    x: DebugUlpsDiff<f64>,
    y: DebugUlpsDiff<f64>,
}

impl FloatEqDebugUlpsDiff for Point {
    type DebugUlpsDiff = PointDebugUlpsDiff;
}
```

### Implementing AssertFloatEq

Requires [FloatEqUlpsEpsilon], [FloatEq] and [FloatEqDebugUlpsDiff].
Implementation is then usually a matter of simply calling through to an
underlying [AssertFloatEq] method for each field in turn. If not, you will need
to take a close look at the descriptions of the algorithms on a method by method
basis:

```rust
impl AssertFloatEq for Point {
    type DebugAbsDiff = Self;
    type DebugEpsilon = Self;

    fn debug_abs_diff(&self, other: &Self) -> Point {
        Point {
            x: self.x.debug_abs_diff(&other.x),
            y: self.y.debug_abs_diff(&other.y),
        }
    }

    fn debug_ulps_diff(&self, other: &Self) -> PointDebugUlpsDiff {
        PointDebugUlpsDiff {
            x: self.x.debug_ulps_diff(&other.x),
            y: self.y.debug_ulps_diff(&other.y),
        }
    }

    fn debug_abs_epsilon(
        &self,
        other: &Self,
        max_diff: &Point
    ) -> Point {
        Point {
            x: self.x.debug_abs_epsilon(&other.x, &max_diff.x),
            y: self.y.debug_abs_epsilon(&other.y, &max_diff.y),
        }
    }

    fn debug_rmax_epsilon(
        &self,
        other: &Self,
        max_diff: &Point
    ) -> Point {
        Point {
            x: self.x.debug_rmax_epsilon(&other.x, &max_diff.x),
            y: self.y.debug_rmax_epsilon(&other.y, &max_diff.y),
        }
    }

    fn debug_rmin_epsilon(
        &self,
        other: &Self,
        max_diff: &Point
    ) -> Point {
        Point {
            x: self.x.debug_rmin_epsilon(&other.x, &max_diff.x),
            y: self.y.debug_rmin_epsilon(&other.y, &max_diff.y),
        }
    }

    fn debug_r1st_epsilon(
        &self,
        other: &Self,
        max_diff: &Point
    ) -> Point {
        Point {
            x: self.x.debug_r1st_epsilon(&other.x, &max_diff.x),
            y: self.y.debug_r1st_epsilon(&other.y, &max_diff.y),
        }
    }

    fn debug_r2nd_epsilon(
        &self,
        other: &Self,
        max_diff: &Point
    ) -> Point {
        Point {
            x: self.x.debug_r2nd_epsilon(&other.x, &max_diff.x),
            y: self.y.debug_r2nd_epsilon(&other.y, &max_diff.y),
        }
    }

    fn debug_ulps_epsilon(
        &self,
        other: &Self,
        max_diff: &PointUlps,
    ) -> PointUlps {
        PointUlps {
            x: self.x.debug_ulps_epsilon(&other.x, &max_diff.x),
            y: self.y.debug_ulps_epsilon(&other.y, &max_diff.y),
        }
    }
}
```

### Implementing AssertFloatEqAll

Requires [FloatEqUlpsEpsilon], [FloatEq], [FloatEqAll], [FloatEqDebugUlpsDiff]
and [AssertFloatEq]. Implementation is then usually a matter of simply calling
through to an underlying [AssertFloatEqAll] method for each field in turn. If
not, you will need to take a close look at the descriptions of the algorithms on
a method by method basis:

```rust
impl AssertFloatEqAll for Point {
    type AllDebugEpsilon = Self;

    fn debug_abs_all_epsilon(
        &self,
        other: &Self,
        max_diff: &Self::AllEpsilon
    ) -> Self::AllDebugEpsilon {
        Point {
            x: self.x.debug_abs_all_epsilon(&other.x, max_diff),
            y: self.y.debug_abs_all_epsilon(&other.y, max_diff),
        }
    }

    fn debug_rmax_all_epsilon(
        &self,
        other: &Self,
        max_diff: &Self::AllEpsilon
    ) -> Self::AllDebugEpsilon {
        Point {
            x: self.x.debug_rmax_all_epsilon(&other.x, max_diff),
            y: self.y.debug_rmax_all_epsilon(&other.y, max_diff),
        }
    }

    fn debug_rmin_all_epsilon(
        &self,
        other: &Self,
        max_diff: &Self::AllEpsilon
    ) -> Self::AllDebugEpsilon {
        Point {
            x: self.x.debug_rmin_all_epsilon(&other.x, max_diff),
            y: self.y.debug_rmin_all_epsilon(&other.y, max_diff),
        }
    }

    fn debug_r1st_all_epsilon(
        &self,
        other: &Self,
        max_diff: &Self::AllEpsilon
    ) -> Self::AllDebugEpsilon {
        Point {
            x: self.x.debug_r1st_all_epsilon(&other.x, max_diff),
            y: self.y.debug_r1st_all_epsilon(&other.y, max_diff),
        }
    }

    fn debug_r2nd_all_epsilon(
        &self,
        other: &Self,
        max_diff: &Self::AllEpsilon
    ) -> Self::AllDebugEpsilon {
        Point {
            x: self.x.debug_r2nd_all_epsilon(&other.x, max_diff),
            y: self.y.debug_r2nd_all_epsilon(&other.y, max_diff),
        }
    }

    fn debug_ulps_all_epsilon(
        &self,
        other: &Self,
        max_diff: &UlpsEpsilon<Self::AllEpsilon>,
    ) -> UlpsEpsilon<Self::AllDebugEpsilon> {
        PointUlps {
            x: self.x.debug_ulps_all_epsilon(&other.x, max_diff),
            y: self.y.debug_ulps_all_epsilon(&other.y, max_diff),
        }
    }
}
```

[AllEpsilon]: ../../doc/float_eq/trait.FloatEqAll.html#associatedtype.AllEpsilon
[AssertFloatEq]: ../../doc/float_eq/trait.AssertFloatEq.html
[AssertFloatEqAll]: ../../doc/float_eq/trait.AssertFloatEqAll.html
[assert_float_eq!]: ../../doc/float_eq/macro.assert_float_eq.html
[DebugUlpsDiff]: ../../doc/float_eq/type.DebugUlpsDiff.html
[float_eq!]: ../../doc/float_eq/macro.float_eq.html
[FloatEq]: ../../doc/float_eq/trait.FloatEq.html
[FloatEqAll]: ../../doc/float_eq/trait.FloatEqAll.html
[FloatEqDebugUlpsDiff]: ../../doc/float_eq/trait.FloatEqDebugUlpsDiff.html
[FloatEqUlpsEpsilon]: ../../doc/float_eq/trait.FloatEqUlpsEpsilon.html
[UlpsEpsilon]: ../../doc/float_eq/type.UlpsEpsilon.html