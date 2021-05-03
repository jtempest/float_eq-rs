use crate::{AssertFloatEq, AssertFloatEqAll, FloatEq, FloatEqAll, UlpsTol};

/// Checks if two floating point expressions are equal to each other.
///
/// See the top level documentation for a list of available [comparison algorithms].
///
/// # Examples
/// ```
/// # use float_eq::float_eq;
/// # use std::f32;
/// let a: f32 = 4.0;
/// let b: f32 = 4.000_002_5;
///
/// assert!(float_eq!(a, b, ulps <= 10));
/// assert!(float_eq!(a, 3.999_999_6, rmax <= 2.0 * f32::EPSILON));
/// assert!(float_eq!(a - b, 0.0, abs <= 0.000_01));
/// ```
///
/// [comparison algorithms]: index.html#comparison-algorithms
/// [from left to right]: index.html#combining-checks
#[macro_export]
macro_rules! float_eq {
    ($a:expr, $b:expr, $($eq:ident <= $tol:expr),+) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                false $(|| $crate::FloatEqCmp::$eq(a_val, b_val, &$tol))+
            }
        }
    });
    ($a:expr, $b:expr, $($eq:ident <= $tol:expr),+,) => ({
        $crate::float_eq!($a, $b $(, $eq <= $tol)+)
    })
}

/// Checks if two floating point expressions are not equal to each other.
///
/// See the top level documentation for a list of available [comparison algorithms].
///
/// # Examples
/// ```
/// # use float_eq::float_ne;
/// # use std::f32;
/// let a: f32 = 4.0;
/// let b: f32 = 4.1;
///
/// assert!(float_ne!(a, b, ulps <= 10));
/// assert!(float_ne!(a, b, rmax <= 2.0 * f32::EPSILON));
/// assert!(float_ne!(a - b, 0.0, abs <= 0.000_01));
/// ```
///
/// [comparison algorithms]: index.html#comparison-algorithms
/// [from left to right]: index.html#combining-checks
#[macro_export]
macro_rules! float_ne {
    ($a:expr, $b:expr, $($eq:ident <= $tol:expr),+) => ({
        !$crate::float_eq!($a, $b $(, $eq <= $tol)+)
    });
    ($a:expr, $b:expr, $($eq:ident <= $tol:expr),+,) => ({
        !$crate::float_eq!($a, $b $(, $eq <= $tol)+)
    });
}

/// Asserts that two floating point expressions are equal to each other.
///
/// See the top level documentation for a list of available [comparison algorithms].
///
/// On panic, this macro will print the values of the expressions with their debug
/// representations, with [additional information] from the comparison operations.
/// Like [`assert!`], this macro has a second form, where a custom panic message can
/// be provided.
///
/// # Examples
/// ```
/// # use float_eq::assert_float_eq;
/// # use std::f32;
/// let a: f32 = 4.0;
/// let b: f32 = 4.000_002_5;
///
/// assert_float_eq!(a, b, ulps <= 10);
/// assert_float_eq!(a, 3.999_999_6, rmax <= 2.0 * f32::EPSILON);
/// assert_float_eq!(a - b, 0.0, abs <= 0.000_01, "Checking that {} == {}", a, b);
/// ```
///
/// [`assert!`]: https://doc.rust-lang.org/std/macro.assert.html
/// [additional information]: https://jtempest.github.io/float_eq-rs/book/how_to/interpret_assert_failure_messages.html
/// [comparison algorithms]: index.html#comparison-algorithms
/// [from left to right]: index.html#combining-checks
#[macro_export]
macro_rules! assert_float_eq {
    // the order of these rules matters a *lot* for the format string functionality
    // to work, otherwise we end up consuming the general case too early.
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr, $eq3:ident <= $tol_3:expr) => ({
        match (&$left, &$right, &$tol_1, &$tol_2, &$tol_3) {
            (left_val, right_val, tol_1_val, tol_2_val, tol_3_val) => {
                if !$crate::float_eq!(
                    *left_val,
                    *right_val,
                    $eq1 <= *tol_1_val,
                    $eq2 <= *tol_2_val,
                    $eq3 <= *tol_3_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_eq!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2), " <= t, ", stringify!($eq3), r#" <= t)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertFloatEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertFloatEq::debug_ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpOpTol::$eq2(&*left_val, &*right_val, &*tol_2_val),
                        concat!("[", stringify!($eq3), "]"),
                        $crate::FloatCmpOpTol::$eq3(&*left_val, &*right_val, &*tol_3_val)
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr) => ({
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                if !$crate::float_eq!(
                    *left_val,
                    *right_val,
                    $eq1 <= *tol_1_val,
                    $eq2 <= *tol_2_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_eq!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2), r#" <= t)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertFloatEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertFloatEq::debug_ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpOpTol::$eq2(&*left_val, &*right_val, &*tol_2_val),
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr) => ({
        match (&$left, &$right, &$tol_1) {
            (left_val, right_val, tol_1_val) => {
                if !$crate::float_eq!(
                    *left_val,
                    *right_val,
                    $eq1 <= *tol_1_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_eq!(left, right, ", stringify!($eq1), r#" <= t)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} t: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertFloatEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertFloatEq::debug_ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $($eq:ident <= $tol:expr,)+) => ({
        $crate::assert_float_eq!($left, $right $(, $eq <= $tol)+)
    });
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr, $eq3:ident <= $tol_3:expr, $($arg:tt)+) => ({
        match (&$left, &$right, &$tol_1, &$tol_2, &$tol_3) {
            (left_val, right_val, tol_1_val, tol_2_val, tol_3_val) => {
                if !$crate::float_eq!(
                    *left_val,
                    *right_val,
                    $eq1 <= *tol_1_val,
                    $eq2 <= *tol_2_val,
                    $eq3 <= *tol_3_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_eq!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2), " <= t, ", stringify!($eq3), r#" <= t)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertFloatEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertFloatEq::debug_ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpOpTol::$eq2(&*left_val, &*right_val, &*tol_2_val),
                        concat!("[", stringify!($eq3), "]"),
                        $crate::FloatCmpOpTol::$eq3(&*left_val, &*right_val, &*tol_3_val),
                        format_args!($($arg)+)
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr, $($arg:tt)+) => ({
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                if !$crate::float_eq!(
                    *left_val,
                    *right_val,
                    $eq1 <= *tol_1_val,
                    $eq2 <= *tol_2_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_eq!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2), r#" <= t)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertFloatEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertFloatEq::debug_ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpOpTol::$eq2(&*left_val, &*right_val, &*tol_2_val),
                        format_args!($($arg)+)
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $($arg:tt)+) => ({
        match (&$left, &$right, &$tol_1) {
            (left_val, right_val, tol_1_val) => {
                if !$crate::float_eq!(
                    *left_val,
                    *right_val,
                    $eq1 <= *tol_1_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_eq!(left, right, ", stringify!($eq1), r#" <= t)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} t: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertFloatEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertFloatEq::debug_ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        format_args!($($arg)+)
                    )
                }
            }
        }
    });
}

/// Asserts that two floating point expressions are not equal to each other.
///
/// See the top level documentation for a list of available [comparison algorithms].
///
/// On panic, this macro will print the values of the expressions with their debug
/// representations, with [additional information] from the comparison operations.
/// Like [`assert!`], this macro has a second form, where a custom panic message can
/// be provided.
///
/// # Examples
/// ```
/// # use float_eq::assert_float_ne;
/// # use std::f32;
/// let a: f32 = 4.0;
/// let b: f32 = 4.1;
///
/// assert_float_ne!(a, b, ulps <= 10);
/// assert_float_ne!(a, b, rmax <= 2.0 * f32::EPSILON);
/// assert_float_ne!(a - b, 0.0, abs <= 0.000_01, "Checking that {} != {}", a, b);
/// ```
///
/// [`assert!`]: https://doc.rust-lang.org/std/macro.assert.html
/// [additional information]: https://jtempest.github.io/float_eq-rs/book/how_to/interpret_assert_failure_messages.html
/// [comparison algorithms]: index.html#comparison-algorithms
/// [from left to right]: index.html#combining-checks
#[macro_export]
macro_rules! assert_float_ne {
    // the order of these rules matters a *lot* for the format string functionality
    // to work, otherwise we end up consuming the general case too early.
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr, $eq3:ident <= $tol_3:expr) => ({
        match (&$left, &$right, &$tol_1, &$tol_2, &$tol_3) {
            (left_val, right_val, tol_1_val, tol_2_val, tol_3_val) => {
                if !$crate::float_ne!(
                    *left_val,
                    *right_val,
                    $eq1 <= *tol_1_val,
                    $eq2 <= *tol_2_val,
                    $eq3 <= *tol_3_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_ne!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2), " <= t, ", stringify!($eq3), r#" <= t)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertFloatEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertFloatEq::debug_ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpOpTol::$eq2(&*left_val, &*right_val, &*tol_2_val),
                        concat!("[", stringify!($eq3), "]"),
                        $crate::FloatCmpOpTol::$eq3(&*left_val, &*right_val, &*tol_3_val)
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr) => ({
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                if !$crate::float_ne!(
                    *left_val,
                    *right_val,
                    $eq1 <= *tol_1_val,
                    $eq2 <= *tol_2_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_ne!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2), r#" <= t)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertFloatEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertFloatEq::debug_ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpOpTol::$eq2(&*left_val, &*right_val, &*tol_2_val),
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr) => ({
        match (&$left, &$right, &$tol_1) {
            (left_val, right_val, tol_1_val) => {
                if !$crate::float_ne!(
                    *left_val,
                    *right_val,
                    $eq1 <= *tol_1_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_ne!(left, right, ", stringify!($eq1), r#" <= t)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} t: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertFloatEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertFloatEq::debug_ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $($eq:ident <= $tol:expr,)+) => ({
        $crate::assert_float_ne!($left, $right $(, $eq <= $tol)+)
    });
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr, $eq3:ident <= $tol_3:expr, $($arg:tt)+) => ({
        match (&$left, &$right, &$tol_1, &$tol_2, &$tol_3) {
            (left_val, right_val, tol_1_val, tol_2_val, tol_3_val) => {
                if !$crate::float_ne!(
                    *left_val,
                    *right_val,
                    $eq1 <= *tol_1_val,
                    $eq2 <= *tol_2_val,
                    $eq3 <= *tol_3_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_ne!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2), " <= t, ", stringify!($eq3), r#" <= t)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertFloatEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertFloatEq::debug_ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpOpTol::$eq2(&*left_val, &*right_val, &*tol_2_val),
                        concat!("[", stringify!($eq3), "]"),
                        $crate::FloatCmpOpTol::$eq3(&*left_val, &*right_val, &*tol_3_val),
                        format_args!($($arg)+)
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $eq2:ident <= $tol_2:expr, $($arg:tt)+) => ({
        match (&$left, &$right, &$tol_1, &$tol_2) {
            (left_val, right_val, tol_1_val, tol_2_val) => {
                if !$crate::float_ne!(
                    *left_val,
                    *right_val,
                    $eq1 <= *tol_1_val,
                    $eq2 <= *tol_2_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_ne!(left, right, ", stringify!($eq1), " <= t, ", stringify!($eq2), r#" <= t)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} t: `{:?}`,
{:>10} t: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertFloatEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertFloatEq::debug_ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpOpTol::$eq2(&*left_val, &*right_val, &*tol_2_val),
                        format_args!($($arg)+)
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $tol_1:expr, $($arg:tt)+) => ({
        match (&$left, &$right, &$tol_1) {
            (left_val, right_val, tol_1_val) => {
                if !$crate::float_ne!(
                    *left_val,
                    *right_val,
                    $eq1 <= *tol_1_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_ne!(left, right, ", stringify!($eq1), r#" <= t)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} t: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::AssertFloatEq::debug_abs_diff(&*left_val, &*right_val),
                        $crate::AssertFloatEq::debug_ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpTol::$eq1(&*left_val, &*right_val, &*tol_1_val),
                        format_args!($($arg)+)
                    )
                }
            }
        }
    });
}

/// Asserts that two floating point expressions are equal to each other.
///
/// See the top level documentation for a list of available [comparison algorithms].
///
/// On panic, this macro will print the values of the expressions with their debug
/// representations, with [additional information] from the comparison operations.
/// Like [`assert!`], this macro has a second form, where a custom panic message can
/// be provided.
///
/// Unlike [`assert_float_eq!`], `debug_assert_float_eq!` statements are only enabled in
/// non optimized builds by default. See [`debug_assert_eq!`] for more details.
///
/// # Examples
/// ```
/// # use float_eq::debug_assert_float_eq;
/// # use std::f32;
/// let a: f32 = 4.0;
/// let b: f32 = 4.000_002_5;
///
/// debug_assert_float_eq!(a, b, ulps <= 10);
/// debug_assert_float_eq!(a, 3.999_999_6, rmax <= 2.0 * f32::EPSILON);
/// debug_assert_float_eq!(a - b, 0.0, abs <= 0.000_01, "Checking that {} == {}", a, b);
/// ```
///
/// [`assert!`]: https://doc.rust-lang.org/std/macro.assert.html
/// [`debug_assert_eq!`]: https://doc.rust-lang.org/std/macro.debug_assert_eq.html
/// [additional information]: https://jtempest.github.io/float_eq-rs/book/how_to/interpret_assert_failure_messages.html
/// [comparison algorithms]: index.html#comparison-algorithms
/// [from left to right]: index.html#combining-checks
#[macro_export]
macro_rules! debug_assert_float_eq {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_float_eq!($($arg)*); })
}

/// Asserts that two floating point expressions are not equal to each other.
///
/// See the top level documentation for a list of available [comparison algorithms].
///
/// On panic, this macro will print the values of the expressions with their debug
/// representations, with [additional information] from the comparison operations.
/// Like [`assert!`], this macro has a second form, where a custom panic message can
/// be provided.
///
/// Unlike [`assert_float_ne!`], `debug_assert_float_ne!` statements are only enabled in
/// non optimized builds by default. See [`debug_assert_ne!`] for more details.
///
/// # Examples
/// ```
/// # use float_eq::debug_assert_float_ne;
/// # use std::f32;
/// let a: f32 = 4.0;
/// let b: f32 = 4.1;
///
/// debug_assert_float_ne!(a, b, ulps <= 10);
/// debug_assert_float_ne!(a, b, rmax <= 2.0 * f32::EPSILON);
/// debug_assert_float_ne!(a - b, 0.0, abs <= 0.000_01, "Checking that {} != {}", a, b);
/// ```
///
/// [`assert!`]: https://doc.rust-lang.org/std/macro.assert.html
/// [`debug_assert_ne!`]: https://doc.rust-lang.org/std/macro.debug_assert_ne.html
/// [additional information]: https://jtempest.github.io/float_eq-rs/book/how_to/interpret_assert_failure_messages.html
/// [comparison algorithms]: index.html#comparison-algorithms
/// [from left to right]: index.html#combining-checks
#[macro_export]
macro_rules! debug_assert_float_ne {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_float_ne!($($arg)*); })
}

#[doc(hidden)]
pub struct FloatEqCmp;

#[doc(hidden)]
impl FloatEqCmp {
    #[inline]
    pub fn abs<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::Tol) -> bool
    where
        A: FloatEq<B>,
    {
        a.eq_abs(b, tol)
    }

    #[inline]
    pub fn abs_all<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::AllTol) -> bool
    where
        A: FloatEqAll<B>,
    {
        a.eq_abs_all(b, tol)
    }

    #[inline]
    pub fn rel<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::Tol) -> bool
    where
        A: FloatEq<B>,
    {
        a.eq_rel(b, tol)
    }

    #[inline]
    pub fn rel_all<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::AllTol) -> bool
    where
        A: FloatEqAll<B>,
    {
        a.eq_rel_all(b, tol)
    }

    #[inline]
    pub fn rmax<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::Tol) -> bool
    where
        A: FloatEq<B>,
    {
        a.eq_rmax(b, tol)
    }

    #[inline]
    pub fn rmax_all<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::AllTol) -> bool
    where
        A: FloatEqAll<B>,
    {
        a.eq_rmax_all(b, tol)
    }

    #[inline]
    pub fn rmin<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::Tol) -> bool
    where
        A: FloatEq<B>,
    {
        a.eq_rmin(b, tol)
    }

    #[inline]
    pub fn rmin_all<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::AllTol) -> bool
    where
        A: FloatEqAll<B>,
    {
        a.eq_rmin_all(b, tol)
    }

    #[inline]
    pub fn r1st<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::Tol) -> bool
    where
        A: FloatEq<B>,
    {
        a.eq_r1st(b, tol)
    }

    #[inline]
    pub fn r1st_all<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::AllTol) -> bool
    where
        A: FloatEqAll<B>,
    {
        a.eq_r1st_all(b, tol)
    }

    #[inline]
    pub fn r2nd<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::Tol) -> bool
    where
        A: FloatEq<B>,
    {
        a.eq_r2nd(b, tol)
    }

    #[inline]
    pub fn r2nd_all<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::AllTol) -> bool
    where
        A: FloatEqAll<B>,
    {
        a.eq_r2nd_all(b, tol)
    }

    #[inline]
    pub fn ulps<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &UlpsTol<A::Tol>) -> bool
    where
        A: FloatEq<B>,
    {
        a.eq_ulps(b, tol)
    }

    #[inline]
    pub fn ulps_all<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &UlpsTol<A::AllTol>) -> bool
    where
        A: FloatEqAll<B>,
    {
        a.eq_ulps_all(b, tol)
    }
}

#[doc(hidden)]
pub struct FloatCmpOpTol;

#[doc(hidden)]
impl FloatCmpOpTol {
    #[inline]
    pub fn abs<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::Tol) -> A::DebugTol
    where
        A: FloatEq<B> + AssertFloatEq<B>,
    {
        a.debug_abs_tol(b, tol)
    }

    #[inline]
    pub fn abs_all<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::AllTol) -> A::AllDebugTol
    where
        A: FloatEqAll<B> + AssertFloatEqAll<B>,
    {
        a.debug_abs_all_tol(b, tol)
    }

    #[inline]
    pub fn rel<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::Tol) -> A::DebugTol
    where
        A: FloatEq<B> + AssertFloatEq<B>,
    {
        a.debug_rel_tol(b, tol)
    }

    #[inline]
    pub fn rel_all<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::AllTol) -> A::AllDebugTol
    where
        A: FloatEqAll<B> + AssertFloatEqAll<B>,
    {
        a.debug_rel_all_tol(b, tol)
    }

    #[inline]
    pub fn rmax<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::Tol) -> A::DebugTol
    where
        A: FloatEq<B> + AssertFloatEq<B>,
    {
        a.debug_rmax_tol(b, tol)
    }

    #[inline]
    pub fn rmax_all<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::AllTol) -> A::AllDebugTol
    where
        A: FloatEqAll<B> + AssertFloatEqAll<B>,
    {
        a.debug_rmax_all_tol(b, tol)
    }

    #[inline]
    pub fn rmin<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::Tol) -> A::DebugTol
    where
        A: FloatEq<B> + AssertFloatEq<B>,
    {
        a.debug_rmin_tol(b, tol)
    }

    #[inline]
    pub fn rmin_all<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::AllTol) -> A::AllDebugTol
    where
        A: FloatEqAll<B> + AssertFloatEqAll<B>,
    {
        a.debug_rmin_all_tol(b, tol)
    }

    #[inline]
    pub fn r1st<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::Tol) -> A::DebugTol
    where
        A: FloatEq<B> + AssertFloatEq<B>,
    {
        a.debug_r1st_tol(b, tol)
    }

    #[inline]
    pub fn r1st_all<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::AllTol) -> A::AllDebugTol
    where
        A: FloatEqAll<B> + AssertFloatEqAll<B>,
    {
        a.debug_r1st_all_tol(b, tol)
    }

    #[inline]
    pub fn r2nd<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::Tol) -> A::DebugTol
    where
        A: FloatEq<B> + AssertFloatEq<B>,
    {
        a.debug_r2nd_tol(b, tol)
    }

    #[inline]
    pub fn r2nd_all<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &A::AllTol) -> A::AllDebugTol
    where
        A: FloatEqAll<B> + AssertFloatEqAll<B>,
    {
        a.debug_r2nd_all_tol(b, tol)
    }

    #[inline]
    pub fn ulps<A: ?Sized, B: ?Sized>(a: &A, b: &B, tol: &UlpsTol<A::Tol>) -> UlpsTol<A::DebugTol>
    where
        A: FloatEq<B> + AssertFloatEq<B>,
        UlpsTol<A::DebugTol>: Sized,
    {
        a.debug_ulps_tol(b, tol)
    }

    #[inline]
    pub fn ulps_all<A: ?Sized, B: ?Sized>(
        a: &A,
        b: &B,
        tol: &UlpsTol<A::AllTol>,
    ) -> UlpsTol<A::AllDebugTol>
    where
        A: FloatEqAll<B> + AssertFloatEqAll<B>,
        UlpsTol<A::AllDebugTol>: Sized,
    {
        a.debug_ulps_all_tol(b, tol)
    }
}
