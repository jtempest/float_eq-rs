# How to manually implement the traits

This article will explain how to enable a new type for use with the [float_eq!],
[float_ne!], [assert_float_eq!] and [assert_float_ne!] macros. If your type is
a struct or tuple struct and is not generic, then you may wish to derive these
traits instead, see [How to derive the traits].

The [float_eq!] and [float_ne!] macros require [FloatEqUlpsTol] and [FloatEq]
and may optionally use [FloatEqAll]. Likewise, [assert_float_eq!] and 
[assert_float_ne!] require [FloatEqDebugUlpsDiff] and [AssertFloatEq] and may
optionally use [AssertFloatEqAll].

## The Point type

By way of example, we will implement the traits for this Point type:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}
```

## Implementing FloatEqUlpsTol

Provide an [UlpsTol] representation for each of the fields:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct PointUlps {
    x: UlpsTol<f64>,
    y: UlpsTol<f64>,
}

impl FloatEqUlpsTol for Point {
    type UlpsTol = PointUlps;
}
```

### Implementing FloatEq

Requires [FloatEqUlpsTol]. Implementation is usually a matter of calling through
to an underlying [FloatEq] method for each field in turn. If not, you will need
to take a close look at the descriptions of the algorithms on a method by method
basis:

```rust
impl FloatEq for Point {
    type Tol = Point;

    fn eq_abs(&self, other: &Self, tol: &Point) -> bool {
        self.x.eq_abs(&other.x, &tol.x) &&
        self.y.eq_abs(&other.y, &tol.y)
    }

    fn eq_rmax(&self, other: &Self, tol: &Point) -> bool {
        self.x.eq_rmax(&other.x, &tol.x) &&
        self.y.eq_rmax(&other.y, &tol.y)
    }

    fn eq_rmin(&self, other: &Self, tol: &Point) -> bool {
        self.x.eq_rmin(&other.x, &tol.x) &&
        self.y.eq_rmin(&other.y, &tol.y)
    }

    fn eq_r1st(&self, other: &Self, tol: &Point) -> bool {
        self.x.eq_r1st(&other.x, &tol.x) &&
        self.y.eq_r1st(&other.y, &tol.y)
    }

    fn eq_r2nd(&self, other: &Self, tol: &Point) -> bool {
        self.x.eq_r2nd(&other.x, &tol.x) &&
        self.y.eq_r2nd(&other.y, &tol.y)
    }

    fn eq_ulps(&self, other: &Self, tol: &UlpsTol<Point>) -> bool {
        self.x.eq_ulps(&other.x, &tol.x) &&
        self.y.eq_ulps(&other.y, &tol.y)
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
    type AllTol = f64;

    fn eq_abs_all(&self, other: &Self, tol: &f64) -> bool {
        self.x.eq_abs_all(&other.x, tol) &&
        self.y.eq_abs_all(&other.y, tol)
    }

    fn eq_rmax_all(&self, other: &Self, tol: &f64) -> bool {
        self.x.eq_rmax_all(&other.x, tol) &&
        self.y.eq_rmax_all(&other.y, tol)
    }

    fn eq_rmin_all(&self, other: &Self, tol: &f64) -> bool {
        self.x.eq_rmin_all(&other.x, tol) &&
        self.y.eq_rmin_all(&other.y, tol)
    }

    fn eq_r1st_all(&self, other: &Self, tol: &f64) -> bool {
        self.x.eq_r1st_all(&other.x, tol) &&
        self.y.eq_r1st_all(&other.y, tol)
    }

    fn eq_r2nd_all(&self, other: &Self, tol: &f64) -> bool {
        self.x.eq_r2nd_all(&other.x, tol) &&
        self.y.eq_r2nd_all(&other.y, tol)
    }

    fn eq_ulps_all(&self, other: &Self, tol: &UlpsTol<f64>) -> bool {
        self.x.eq_ulps_all(&other.x, tol) &&
        self.y.eq_ulps_all(&other.y, tol)
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

Requires [FloatEqUlpsTol], [FloatEq] and [FloatEqDebugUlpsDiff]. Implementation
is then usually a matter of simply calling through to an underlying
[AssertFloatEq] method for each field in turn. If not, you will need to take a
close look at the descriptions of the algorithms on a method by method basis:

```rust
impl AssertFloatEq for Point {
    type DebugAbsDiff = Self;
    type DebugTol = Self;

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

    fn debug_abs_tol(
        &self,
        other: &Self,
        tol: &Point
    ) -> Point {
        Point {
            x: self.x.debug_abs_tol(&other.x, &tol.x),
            y: self.y.debug_abs_tol(&other.y, &tol.y),
        }
    }

    fn debug_rmax_tol(
        &self,
        other: &Self,
        tol: &Point
    ) -> Point {
        Point {
            x: self.x.debug_rmax_tol(&other.x, &tol.x),
            y: self.y.debug_rmax_tol(&other.y, &tol.y),
        }
    }

    fn debug_rmin_tol(
        &self,
        other: &Self,
        tol: &Point
    ) -> Point {
        Point {
            x: self.x.debug_rmin_tol(&other.x, &tol.x),
            y: self.y.debug_rmin_tol(&other.y, &tol.y),
        }
    }

    fn debug_r1st_tol(
        &self,
        other: &Self,
        tol: &Point
    ) -> Point {
        Point {
            x: self.x.debug_r1st_tol(&other.x, &tol.x),
            y: self.y.debug_r1st_tol(&other.y, &tol.y),
        }
    }

    fn debug_r2nd_tol(
        &self,
        other: &Self,
        tol: &Point
    ) -> Point {
        Point {
            x: self.x.debug_r2nd_tol(&other.x, &tol.x),
            y: self.y.debug_r2nd_tol(&other.y, &tol.y),
        }
    }

    fn debug_ulps_tol(
        &self,
        other: &Self,
        tol: &PointUlps,
    ) -> PointUlps {
        PointUlps {
            x: self.x.debug_ulps_tol(&other.x, &tol.x),
            y: self.y.debug_ulps_tol(&other.y, &tol.y),
        }
    }
}
```

### Implementing AssertFloatEqAll

Requires [FloatEqUlpsTol], [FloatEq], [FloatEqAll], [FloatEqDebugUlpsDiff]
and [AssertFloatEq]. Implementation is then usually a matter of simply calling
through to an underlying [AssertFloatEqAll] method for each field in turn. If
not, you will need to take a close look at the descriptions of the algorithms on
a method by method basis:

```rust
impl AssertFloatEqAll for Point {
    type AllDebugTol = Self;

    fn debug_abs_all_tol(
        &self,
        other: &Self,
        tol: &Self::AllTol
    ) -> Self::AllDebugTol {
        Point {
            x: self.x.debug_abs_all_tol(&other.x, tol),
            y: self.y.debug_abs_all_tol(&other.y, tol),
        }
    }

    fn debug_rmax_all_tol(
        &self,
        other: &Self,
        tol: &Self::AllTol
    ) -> Self::AllDebugTol {
        Point {
            x: self.x.debug_rmax_all_tol(&other.x, tol),
            y: self.y.debug_rmax_all_tol(&other.y, tol),
        }
    }

    fn debug_rmin_all_tol(
        &self,
        other: &Self,
        tol: &Self::AllTol
    ) -> Self::AllDebugTol {
        Point {
            x: self.x.debug_rmin_all_tol(&other.x, tol),
            y: self.y.debug_rmin_all_tol(&other.y, tol),
        }
    }

    fn debug_r1st_all_tol(
        &self,
        other: &Self,
        tol: &Self::AllTol
    ) -> Self::AllDebugTol {
        Point {
            x: self.x.debug_r1st_all_tol(&other.x, tol),
            y: self.y.debug_r1st_all_tol(&other.y, tol),
        }
    }

    fn debug_r2nd_all_tol(
        &self,
        other: &Self,
        tol: &Self::AllTol
    ) -> Self::AllDebugTol {
        Point {
            x: self.x.debug_r2nd_all_tol(&other.x, tol),
            y: self.y.debug_r2nd_all_tol(&other.y, tol),
        }
    }

    fn debug_ulps_all_tol(
        &self,
        other: &Self,
        tol: &UlpsTol<Self::AllTol>,
    ) -> UlpsTol<Self::AllDebugTol> {
        PointUlps {
            x: self.x.debug_ulps_all_tol(&other.x, tol),
            y: self.y.debug_ulps_all_tol(&other.y, tol),
        }
    }
}
```

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
[UlpsTol]: ../../doc/float_eq/type.UlpsTol.html
[How to derive the traits]: ./derive_the_traits.html