# How to compare floating point numbers

1) Determine which [comparison algorithm] best suits your purposes.

2) Determine the tolerance required based on the sources of the error in how the
numbers were entered/calculated. 

3) If you need a boolean result, then use [`float_eq!`] or [`float_ne!`]. The
two numbers should be the first and second operands, and then the tolerance is
the value after the `<=`. For example, to compare two `f32` numbers stored in
`a` and `b` using a relative tolerance of `tol` scaled to the magnitude of the
second operand:

```rust
float_eq!(a, b, r2nd <= tol)
```

4) If instead you wish to assert that the two numbers are equal and panic if
they are not, you can use [`assert_float_eq!`] or [`assert_float_ne!`]. The
syntax is the same, and may optionally include formatted text after the
comparisons as with standard Rust asserts:

```rust
assert_float_eq!(a, b, r2nd <= tol);
assert_float_eq!(a, b, r2nd <= tol, "Example context: {}", context);
```

[comparison algorithm]: ../background/float_comparison_algorithms.md
[`assert_float_eq!`]: ../../doc/float_eq/macro.assert_float_eq.html
[`assert_float_ne!`]: ../../doc/float_eq/macro.assert_float_ne.html
[`float_eq!`]: ../../doc/float_eq/macro.float_eq.html
[`float_ne!`]: ../../doc/float_eq/macro.float_ne.html