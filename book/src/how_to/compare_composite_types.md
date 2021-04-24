# How to compare composite types

1) Composite types such as arrays, tuples and structs may be compared by
specifying a per-field tolerance in an instance of the same type:

```rust
let a = [1.0, -2.0, 3.0];
let b = [-1.0, 2.0, 3.5];
assert_float_eq!(a, b, abs <= [2.0, 4.0, 0.5]);

let c = Complex32 { re: 2.0, im: 4.000_002 };
let d = Complex32 { re: 2.000_000_5, im: 4.0 };
assert_float_eq!(c, d, rmax <= Complex32 { re: 0.000_000_25, im: 0.000_000_5 });
assert_float_eq!(c, d, ulps <= Complex32Ulps { re: 2, im: 4 });
```

2) Homogeneous types may also support the `_all` variants of the checks, which
allow you to specify a single tolerance to use across all fields:

```rust
assert_float_eq!(a, b, abs_all <= 4.0);

assert_float_eq!(c, d, rmax_all <= 0.000_000_5);
assert_float_eq!(c, d, ulps_all <= 4);
```

3) Checks may be extended over new types by implementing the [extension traits].

[extension traits]: ./compare_custom_types.md