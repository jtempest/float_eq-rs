# Basic usage

So you have attempted to perform a simple floating point calculation in Rust,
perhaps an [old classic]:

```rust
let sum = 0.1 + 0.2;

if sum == 0.3 {
    println!("Decimal math is working as expected!");
} else {
    println!("Something has gone horribly wrong!");
}
```

And it turns out that:

```txt
Something has gone horribly wrong!
```

What's going on? Let's take a closer look at the result of the sum:

```rust
println!("0.1 + 0.2 = {}", sum);
```
```txt
0.1 + 0.2 = 0.30000000000000004
```

That doesn't seem like the right answer at all! It's very close, but why is it
off by a tiny amount?

Well, what's happened is that the `f64` type being used to calculate `sum` has a
*binary* numeric representation, and our inputs are specified as *decimal*
numbers. Neither 0.1 nor 0.2 can be [represented exactly] in binary and so they
have been rounded to the nearest values which can be. The addition is performed
using binary arithmetic and finally is converted back into a decimal
representation to be printed.

We can see how these values have been rounded by printing them with a high
enough precision:

```rust
println!("0.1 -> {:.55}", 0.1);
println!("0.2 -> {:.55}", 0.2);
```
```txt
0.1 -> 0.1000000000000000055511151231257827021181583404541015625
0.2 -> 0.2000000000000000111022302462515654042363166809082031250
```

## Close, so very close

Very few decimal numbers may be exactly represented in binary. Only powers of
two:

```rust
println!("4.0   -> {:.55}", 4.0);
println!("2.0   -> {:.55}", 2.0);
println!("1.0   -> {:.55}", 1.0);
println!("0.5   -> {:.55}", 0.5);
println!("0.25  -> {:.55}", 0.25);
println!("0.125 -> {:.55}", 0.125);
```
```txt
4.0   -> 4.0000000000000000000000000000000000000000000000000000000
2.0   -> 2.0000000000000000000000000000000000000000000000000000000
1.0   -> 1.0000000000000000000000000000000000000000000000000000000
0.5   -> 0.5000000000000000000000000000000000000000000000000000000
0.25  -> 0.2500000000000000000000000000000000000000000000000000000
0.125 -> 0.1250000000000000000000000000000000000000000000000000000
```
And sums of powers of two:

```rust
println!("{:.55}", 0.5 + 0.25);
println!("{:.55}", 4.0 + 0.5 + 0.125);
```
```txt
0.7500000000000000000000000000000000000000000000000000000
4.6250000000000000000000000000000000000000000000000000000
```

Where:

- 4.000 = 2<sup>2</sup>
- 2.000 = 2<sup>1</sup>
- 1.000 = 2<sup>0</sup>
- 0.500 = 2<sup>-1</sup>
- 0.250 = 2<sup>-2</sup>
- 0.125 = 2<sup>-3</sup>

Most decimal numbers are not sums of powers of two[^decimals], so every time we
convert a decimal into floating point there is a high chance that it will be
rounded. The difference between the input value and the converted `f64` floating
point value is known as the initial [roundoff error].

## Accumulating errors

Conversion from decimal to binary is not the only source of error in floating
point arithmetic. If the only problem was the initial roundoff error, then we
could reasonably expect the sum 0.1 + 0.2 to be computed exactly and be equal
to the converted constant 0.3 when we test it with `==`.

It is not:

```rust
let sum = 0.1 + 0.2;

println!("sum -> {:.55}", sum);
println!("0.3 -> {:.55}", 0.3);
```
```txt
sum -> 0.3000000000000000444089209850062616169452667236328125000
0.3 -> 0.2999999999999999888977697537484345957636833190917968750
```

Why aren't they the same? Well, `sum` has been rounded *up* to the nearest
representable `f64` number, and 0.3 has been rounded *down*.

Let's think a bit more about the original inputs. They have both been rounded
*up* compared to their decimal values:

```txt
0.1 -> 0.1000000000000000055511151231257827021181583404541015625
0.2 -> 0.2000000000000000111022302462515654042363166809082031250
```

If we add these rounded values together by hand, we get a number that is just
slightly above 0.3:

```txt
  0.1000000000000000055511151231257827021181583404541015625
+ 0.2000000000000000111022302462515654042363166809082031250
= 0.3000000000000000166533453693773481063544750213623046775
```

The results of operations on floating point values are rounded to the nearest
representable value[^default]. It just so happens that in this example, the `sum` of the
two rounded values is high enough to be rounded up, whereas 0.3 is low enough to
be rounded down. This is why checking they are equal with `==` returns false.

Nearly every operation on floating point numbers can result in further rounding,
amplifying the effect of previous rounding. You may be able to mitigate this
somewhat by reordering operations to reduce the margin, but it is impossible to
avoid error entirely.

## Distant relatives 

Wait a minute though. If our two input two values are exactly representable as
binary numbers then why was the result of adding them together rounded at all?
Shouldn't `sum` be the same as the answer we calculated by hand? It is somewhat
higher than we expected:

```txt
by hand: 0.3000000000000000166533453693773481063544750213623046775
by f64:  0.3000000000000000444089209850062616169452667236328125000
```

They are different because of the fundamental design of floating point numbers.
The big advantage of floating point types, and the reason that we may be happy
to deal with the small errors inherent in using them, is that they can represent
a literally *astronomically* larger range of absolute values than integer types
can in the same number of bits.

The trade off is that the granularity of floating point numbers changes with
their magnitude. As floating point numbers get larger the *absolute* distance
between adjacent values grows, and as they get smaller it shrinks, whereas the
*relative* ratio between values remains constant. This is in contrast to integer
types where, regardless of magnitude, adjacent values are always the same
*absolute* distance away from each another (one), but their *relative* ratio
changes.

Let's make this more concrete by comparing `i32` and `f32`. Each can represent
4,294,967,296 different values using their 32 bits, but they make very different
choices as to what those values are.

The positive range of `i32` can represent:

```rust
0
1
...
2147483646
2147483647 // i32::MAX
```

The positive range of `f32` can represent:

```rust
0.0
...
0.000000000000000000000000000000000000011754944 // f32::MIN_POSITIVE
0.000000000000000000000000000000000000011754945
...
340282330000000000000000000000000000000.0
340282350000000000000000000000000000000.0 // f32::MAX
∞ // f32::INFINITY                                        
```

As mentioned above, the *absolute* distance between every adjacent pair of `i32`
numbers within its range is always one, but their *relative* ratio changes
depending on their magnitude:

```txt
0 -> 1:
absolute: 1 - 0 = 1
relative: 1 / 0 = ∞

1 -> 2:
absolute: 2 - 1 = 1
relative: 2 / 1 = 2

...

2147483646 -> 2147483647:
absolute: 2147483647 - 2147483646 = 1
relative: 2147483647 / 2147483646 = 1.0000000004656612873077392578125
```

The `f32` number line is a bit more complex. Zero and infinity are special
values at the extremes and the tiny range of subnormal values just above zero
act differently but the vast majority of `f32` values are [normal floating point
numbers] in the range from `f32::MIN_POSITIVE` to `f32::MAX`.

The absolute distance between each pair of adjacent normal `f32` values varies
depending on their size but their relative ratio is always 1.0000001. We can
illustrate this with some help from the [ieee754] crate:

```rust
use ieee754::Ieee754;

let a = f32::MIN_POSITIVE;
let b = a.next();
println!("{} ->\n{}:", a, b);
println!("absolute: {}", b - a);
println!("relative: {}\n", b / a);

let c = f32::MIN_POSITIVE.next();
let d = c.next();
println!("{} ->\n{}:", c, d);
println!("absolute: {}", d - c);
println!("relative: {}\n", d / c);

println!("...\n");

let e = f32::MAX.prev();
let f = f32::MAX;
println!("{} ->\n{}:", e, f);
println!("absolute: {}", f - e);
println!("relative: {}", f / e);
```
```txt
0.000000000000000000000000000000000000011754944 ->
0.000000000000000000000000000000000000011754945:
absolute: 0.000000000000000000000000000000000000000000001
relative: 1.0000001

0.000000000000000000000000000000000000011754945 ->
0.000000000000000000000000000000000000011754946:
absolute: 0.000000000000000000000000000000000000000000001
relative: 1.0000001

...

340282330000000000000000000000000000000 ->
340282350000000000000000000000000000000:
absolute: 20282410000000000000000000000000
relative: 1.0000001
```

### Machine epsilon

The absolute distance between adjacent numbers for a floating point type is
determined by multiples of that type's *machine epsilon*. This is the distance
between adjacent values in the range 1.0 to 2.0. For `f64` this is the constant
`f64::EPSILON`. We can scale it for every other range of powers of two to
determine the absolute distance between numbers in those ranges:

- 0.25 to 0.5 contains 8,388,608 values, all a distance of `0.25 * f64::EPSILON` apart.
- 0.5  to 1.0 contains 8,388,608 values, all a distance of `0.5 * f64::EPSILON` apart.
- 1.0  to 2.0 contains 8,388,608 values, all a distance of `f64::EPSILON` apart.
- 2.0  to 4.0 contains 8,388,608 values, all a distance of `2.0 * f64::EPSILON` apart.
- 4.0  to 8.0 contains 8,388,608 values, all a distance of `4.0 * f64::EPSILON` apart.

This is why our addition was unexpectedly rounded up:

- `0.1` is in the range 0.0625 to 0.125, which are all `0.0625 f64::EPSILON` apart.
- `0.2` is in the range 0.125 to 0.25, which are all `0.125 * f64::EPSILON` apart.
- `0.1 + 0.2` is in the range 0.25 to 0.5, which are all `0.25 * f64::EPSILON` apart.

The input values 0.1 and 0.2 were both rounded up to the nearest values in their
respective ranges. The result of adding them together is within a range with
a lower granularity and was not exactly representable, so it needed to be
rounded to the nearest value that is.

## It gets much worse

But that's not the end of the story.

There are far more significant sources of error in most numerical calculations
than the roundoff errors we have described so far. If the inputs are real world
values, then there is almost certainly some [measurement error] in how they were
collected. Algorithms like physics simulations that use discrete steps to
approximate continuous functions introduce [truncation error]. Even the
underyling mathematics may amplify existing error if it is [numerically
unstable], for example when dividing large numbers by much smaller ones or if
values undergo [catastrophic cancellation].

I'll say that again, because it's so important: in most real world programs,
floating point roundoff error is so small as to be insignificant compared to
other sources of error. Floating point numerics may be inappropriate for values
that have strict absolute precision requirements, such as currency, but in
general there is no need to shy away from using them because of their rounding
behaviour.

Every numerical algorithm is unique and there is no one size fits all solution
or set of defaults to account for the error inherent in them and hide it away.
In fact, there is an entire field of active mathematical research concerned with
computational error, [Numerical Analysis]. What we can do, however, is learn to
reason about these sources of error and provide tools for taking it into account
and communicating our thoughts to future readers and maintainers.

## Close enough

Bearing that in mind, let's return to our original comparison. Now that we know
why the exact `==` comparison failed, we can instead check that the difference
between the expected and actual values lies within a margin of error, also known
as the tolerance. This is what the [`float_eq!`] macro is for.

### Absolute tolerance comparison

The simplest algorithm to check if two floating point numbers are equal is an
[absolute tolerance comparison]. This tests that the absolute difference between
two values lies within a specified tolerance and is invoked with the syntax
`abs <= tol`. We have calculated that in our very simple example the values may
differ by at most `0.25 * f64::EPSILON`:

```rust
use float_eq::float_eq;

let sum = 0.1 + 0.2;

if float_eq!(sum, 0.3, abs <= 0.25 * f64::EPSILON) {
    println!("Floating point math is working as expected!");
} else {
    println!("Something has gone horribly wrong!");
}
```
```txt
Floating point math is working as expected!
```

### Relative tolerance comparison

Hurray! However, manually scaling the tolerance to the range of the operands is
not very elegant. Fortunately, given that we know that we are comparing two
normal numbers, we can use a [relative tolerance comparison] to scale the
tolerance for us. The second operand is our expected value, so we may choose to
use `r2nd <= tol`. With a relative tolerance comparison, the tolerance should be
as if we were testing a value in the range 1.0 to 2.0, so `f64::EPSILON`
indicates we are expecting our operands to be no more than one representable
value apart from each other[^relative]:

```rust
# use float_eq::float_eq;
# let sum = 0.1 + 0.2;
if float_eq!(sum, 0.3, r2nd <= f64::EPSILON) {
    println!("Floating point math is working as expected!");
} else {
    println!("Something has gone horribly wrong!");
}
```
```txt
Floating point math is working as expected!
```

### ULPs based comparison

If both numbers are normal *and* the same sign, which is often the case, we can
use an [ULPs comparison], another form of relative check. This uses a property
of the [underlying representation] of floating point numbers which means that
when we interpret their bits as unsigned integers, the adjacent floats are the
adjacent integer values above and below. By using `ulps <= tol` to invoke one of
these checks, the tolerance is the maximum number of representable values apart
they may be regardless of their magnitude. In our example, we know they may be
at most one representable `f64` value apart:

```rust
# use float_eq::float_eq;
# let sum = 0.1 + 0.2;
if float_eq!(sum, 0.3, ulps <= 1) {
    println!("Floating point math is working as expected!");
} else {
    println!("Something has gone horribly wrong!");
}
```
```txt
Floating point math is working as expected!
```

## Asserting ourselves

The `float_eq` library also includes [`assert_float_eq!`] to accompany the
boolean [`float_eq!`] operator. To illustrate their use and show some more
advanced comparison techniques, we will define a very simple numerical
integrator. This function takes an initial value and step size, then advances
the value by an arbitrary number of steps:

```rust
fn integrate(initial: f64, step: f64, count: usize) -> f64 {
    let mut sum = initial;
    for _ in 0..count {
        sum += step;
    }
    sum
}
```

Say we want to unit test a number of different sets of input to this algorithm,
we might build ourselves a wrapper for it:

```rust
fn test_integrate(initial: f64, step: f64, count: usize, expected: f64) {
    let actual = integrate(initial, step, count);
    assert_float_eq!(actual, expected, r2nd <= f64::EPSILON);
}
```

Note that the syntax for assert, `r2nd <= tol` is the same as for the boolean
form, and that you may use any of the same algorithms if you wish. Arbitrarily,
we have begun with the same tolerance as for our first example comparison. This
means that the equivalent of our first simple comparison will pass this test
just fine:

```rust
test_integrate(0.2, 0.1, 1, 0.3);
```

### Drifting away

Let's add some tests which use an increasing step size. Our first two are within
the existing expected margin of error:

```rust
test_integrate(0.0, 0.1, 1, 0.1);
test_integrate(0.0, 0.1, 10, 1.0);
```

But when we look at 100 steps, we find that our test fails:

```rust
test_integrate(0.0, 0.1, 100, 10.0);
```
```txt
thread 'main' panicked at 'assertion failed: `float_eq!(left, right, r2nd <= t)`
        left: `9.99999999999998`,
       right: `10.0`,
    abs_diff: `0.000000000000019539925233402755`,
   ulps_diff: `Some(11)`,
    [r2nd] t: `0.000000000000002220446049250313`', src\main.rs:15:9
```

This assert form prints out the values of the operands, like the standard Rust
asserts, but it also provides additional context information to help us make
sense of why it failed. Here, t is the tolerance:

- `abs_diff` is the absolute difference between the two values: 
`(left - right).abs()`.
- `ulps_diff` is the difference in ULPs between the two values, the count of
representable values they are apart. It may be `None` if two numbers have
different signs.
- `[r2nd] t` is the tolerance as scaled to the range of `right` (the second
operand).

We can see that our actual and expected values are eleven ULPs from one another.
That means that our margin of `f64::EPSILON`, equivalent to one ULP, is
inadequate. This is because we are performing `count` number of additions, and
each one of those provides an answer accurate to within 0.5 ULPs, so they have
accumulated more error than a single step or ten steps would. We might reason
that our tolerance should therefore be some function of the number of steps,
perhaps `(count * 0.5) * f64::EPSILON`:

```rust
fn test_integrate(initial: f64, step: f64, count: usize, expected: f64) {
    let actual = integrate(initial, step, count);
    assert_float_eq!(
        actual,
        expected,
        r2nd <= ((count as f64) * 0.5) * f64::EPSILON
    );
}
```

### Absolute zero

Let's take a look at another failure case, when we count backwards to zero:

```rust
test_integrate(10.0, -0.1, 100, 0.0);
```
```txt
thread 'main' panicked at 'assertion failed: `float_eq!(left, right, r2nd <= t)`
        left: `0.000000000000018790524691780774`,
       right: `0.0`,
    abs_diff: `0.000000000000018790524691780774`,
   ulps_diff: `Some(4401468191289638912)`,
    [r2nd] t: `0.0`', src\main.rs:15:9
```

Wow! There are a couple of things to note about this failure. The first is that
because we are comparing versus zero, scaling our epsilon doesn't work - zero
times anything is zero, so the margin does not take into account the step count.
Even more noticable though is just *how far away* our actual value is in terms
of representable numbers.

There are two reasons why this has happened. The first is that, as mentioned
above, zero is a special number and does not have the same properties as normal
floating point numbers. The second is that our final subtraction leaves us with
a result many orders of magnitude smaller than the previous sum total, resulting
in a [catastrophic cancellation].

The solution here is to use a more sophisticated margin for our tolerance, one
that takes into account the nature of the calculation itself. It needs to be an
absolute test, since we may be comparing versus zero. It should also scale
relative to the largest intermediate value in the calculation and take into
account the potential rounding errors from our repeated addition:

```rust
fn test_integrate(initial: f64, step: f64, count: usize, expected: f64) {
    let actual = integrate(initial, step, count);
    let half_count = (count as f64) * 0.5;
    let tol = f64::EPSILON * half_count * f64::max(initial.abs(), actual.abs());
    assert_float_eq!(actual, expected, abs <= tol);
}
```

This further illustates that there is no one right or general way to express the
tolerances of numeric algorithms. Every comparison will be based on the specific
calculation being performed and frequently the particular inputs.

### Custom messages

The assert macros may include a custom message in the same manner as the
standard Rust asserts:

```rust
assert_float_eq!(
    actual,
    expected,
    r2nd <= f64::EPSILON,
    "\nWhere: initial: {}, step: {}, count:{}",
    initial,
    step,
    count
);
```
```txt
thread 'main' panicked at 'assertion failed: `float_eq!(left, right, r2nd <= t)`
        left: `9.99999999999998`,
       right: `10.0`,
    abs_diff: `0.000000000000019539925233402755`,
   ulps_diff: `Some(11)`,
    [r2nd] t: `0.000000000000002220446049250313`:
Where: initial: 0, step: 0.1, count:100', src\main.rs:14:9
```

## Staying afloat

Hopefully that's given you some flavour of the issues that crop up when
implementing numerical methods and how `float_eq` may aid you when they do. At
this point you may be interested in learning how to perform some [specific
tasks], reading some more general [background] explanations or browsing the [API
documentation].

----

[^decimals]: They are in fact all exact sums of multiples of powers of ten.

[^default]: By default. Other rounding modes such as always rounding up, down or
toward zero are available.

[^relative]: In general, `n * fXX::EPSILON` as a relative margin means "at most
n representable values apart", for example you might use a margin of
`4.0 * f64::EPSILON`.

[absolute tolerance comparison]: ../background/float_comparison_algorithms.html#absolute-tolerance-comparison
[API documentation]: ../api_documentation.md
[background]: ../background.md
[catastrophic cancellation]: https://en.wikipedia.org/wiki/Catastrophic_cancellation
[ieee754]: https://crates.io/crates/ieee754
[measurement error]: https://en.wikipedia.org/wiki/Observational_error
[normal floating point numbers]: https://en.wikipedia.org/wiki/Normal_number_(computing)
[Numerical Analysis]: https://en.wikipedia.org/wiki/Numerical_analysis
[numerically unstable]: https://nhigham.com/2020/08/04/what-is-numerical-stability/
[old classic]: https://0.30000000000000004.com/
[relative tolerance comparison]: ../background/float_comparison_algorithms.html#relative-tolerance-comparison
[represented exactly]: https://www.exploringbinary.com/why-0-point-1-does-not-exist-in-floating-point/
[roundoff error]: https://en.wikipedia.org/wiki/Round-off_error
[specific tasks]: ../how_to.md
[truncation error]: https://en.wikipedia.org/wiki/Truncation_error
[ULPs comparison]: ../background/float_comparison_algorithms.html#units-in-the-last-place-ulps-comparison
[underlying representation]: https://randomascii.wordpress.com/2012/01/23/stupid-float-tricks-2/
[`assert_float_eq!`]: ../../doc/float_eq/macro.assert_float_eq.html
[`float_eq!`]: ../../doc/float_eq/macro.float_eq.html