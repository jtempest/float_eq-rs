# Floating point comparison algorithms

Descriptions of the underlying comparison algorithms used by [float_eq].

## Absolute tolerance comparison

```
abs <= tol
```

A check to see how far apart two expressions are by comparing the absolute
difference between them to an absolute tolerance. Mathematically, this is:

```
|a - b| <= tol
```

Equivalent to, using `f32` as an example:

```rust
fn float_eq_abs(a: f32, b: f32, tol: f32) -> bool {
    // the PartialEq check covers equality of infinities
    a == b || (a - b).abs() <= tol
}
```

This is the simplest method of testing the equality of two floats and may be
sufficient if you know the absolute margin of error for your calculation given
the values being tested. However, absolute tolerance tests *do not* work well for
general comparison of floating point numbers, because they do not take into
account that normal values' granularity changes with their magnitude. Thus any
given choice of `tol` is likely to work for one specific exponent's range and
poorly outside of it.

In some circumstances absolute tolerance comparisons are required. If you wish
to compare against zero, an infinity, or subnormal values then the assumptions
that relative tolerance or ULPs based checks make about how neighbouring values
are related to one another break down. Similarly, if the underlying mathematics
of your algorithm is [numerically unstable], for example if it is prone to
[catastrophic cancellation], then you may find that you need to reach for an
absolute tolerance comparison.

## Relative tolerance comparison

```
r1st <= tol
r2nd <= tol
rmax <= tol
rmin <= tol
```

A check to see how far apart two expressions are by comparing the absolute
difference between them to an tolerance that is scaled to the granularity of
one of the inputs. Mathematically, this is:

```
|a - b| <= func(|a|, |b|) * tol
```

Equivalent to, using `f32` as an example:

```rust
fn float_eq_relative(a: f32, b: f32, tol: f32) -> bool {
    // the PartialEq check covers equality of infinities
    a == b || {
        let chosen = func(a.abs(), b.abs());
        (a - b).abs() <= (chosen * tol)
    }
}
```

Where `func` is one of:
- `r1st`: the first input (`a`)
- `r2nd`: the second input (`b`)
- `rmax`: the larger magnitude (aka `rel` for legacy reasons)
- `rmin`: the smaller magnitude

If you are checking for equality versus an expected normal floating point value
then you may wish to calculate the tolerance based on that value and so using
`r1st` or `r2nd` will allow you to select it. If you are generally testing two
normal floating point values then `rmax` is a good general choice. If either
number may also be subnormal or close to zero, then you may need to calculate a
tolerance based on an intermediate value for an absolute tolerance check
instead.

Choice of `tol` will depend on the tolerances inherent in the specific
mathematical function or algorithm you have implemented. Note that a tolerance
of `n * EPSILON` (e.g. `f32::EPSILON`) will test that two expressions are within
`n` representable values of another. However, you should be aware that the
errors inherent in your inputs and calculations are likely to be much greater
than the small rounding errors this form would imply.

## Units in the Last Place (ULPs) comparison

```
ulps <= tol
```

A check to see how far apart two expressions are by comparing the number of
representable values between them. This works by interpreting the bitwise
representation of the input values as integers and comparing the absolute
difference between those. Equivalent to, using `f32` as an example:

```rust
fn float_eq_ulps(a: f32, b: f32, tol: u32) -> bool {
    if a.is_nan() || b.is_nan() {
        false // NaNs are never equal
    } else if a.is_sign_positive() != b.is_sign_positive() {
        a == b // values of different signs are only equal if both are zero.
    } else {
        let a_bits = a.to_bits();
        let b_bits = b.to_bits();
        let max = a_bits.max(b_bits);
        let min = a_bits.min(b_bits);
        (max - min) <= tol
    }
}
```

Thanks to a deliberate quirk in the way the [underlying format] of IEEE floats
was designed, this is a measure of how near two values are that scales with
their relative granularity. Note that `tol` is an unsigned integer, so for
example `ulps <= 4` means *"check that a and b are equal to within a distance of
four or less representable values"*.

ULPs comparisons are very similar to relative tolerance checks, and as such are
useful for testing equality of normal floats but not for comparisons with zero
or infinity. Additionally, because floats use their most significant bit to
indicate their sign, ULPs comparisons are not valid for comparing values with
different signs.

[catastrophic cancellation]: https://en.wikipedia.org/wiki/Catastrophic_cancellation
[float_eq]: http://crates.io/crates/float_eq
[numerically unstable]: https://nhigham.com/2020/08/04/what-is-numerical-stability/
[underlying format]: https://randomascii.wordpress.com/2012/01/23/stupid-float-tricks-2/