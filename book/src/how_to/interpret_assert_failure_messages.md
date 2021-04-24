# How to interpret assert failure messages

Assertion failure messages provide context information that hopefully helps
in determining how a check failed. For example, this line:

```rust
assert_float_eq!(4.0f32, 4.000_008, rmax <= 0.000_001);
```

Panics with this error message:

```text
thread 'main' panicked at 'assertion failed: `float_eq!(left, right, rmax <= t)`
        left: `4.0`,
       right: `4.000008`,
    abs_diff: `0.000008106232`,
   ulps_diff: `Some(17)`,
    [rmax] t: `0.000004000008`', assert_failure.rs:15:5
```

Where:
- **rmax <= t** - indicates the type of comparison which was carried out.
- **left** - the value of the first operand.
- **right** - the value of the second operand.
- **abs_diff** - the absolute difference between `left` and `right`.
- **ulps_diff** - the difference between `left` and `right` in ULPs. If it is
None, that is because they have different signs.
- **[rmax] t** - the tolerance used in the comparison against the relevant
difference, here `abs_diff`, *after* it has been scaled relative to an operand,
in this case `max(left, right)` since it is `rmax`.