use crate::{FloatEq, FloatEqAll, FloatEqAllDebug, FloatEqDebug, Ulps};

/// Checks if two floating point expressions are equal to each other.
///
/// Comparisons are applied in order from left to right, shortcutting to return
/// early if a positive result is found.
///
/// - `abs <= max_diff` is an [absolute epsilon comparison].
/// - `rel <= max_diff` is a [relative epsilon comparison].
/// - `ulps <= max_diff` is an [ULPs comparison].
///
/// When comparing [composite types], variants that use a uniform `max_diff`
/// value across all fields are also available:
///
/// - `abs_all <= max_diff` is an [absolute epsilon comparison].
/// - `rel_all <= max_diff` is a [relative epsilon comparison].
/// - `ulps_all <= max_diff` is an [ULPs comparison].
///
/// # Examples
/// ```
/// # use float_eq::float_eq;
/// # use std::f32;
/// let a: f32 = 4.;
/// let b: f32 = 4.000_002_5;
///
/// assert!( float_eq!(a, 3.999_999_8, rel <= f32::EPSILON) );
/// assert!( float_eq!(a - b, 0., abs <= 0.000_01, rel <= f32::EPSILON) );
/// assert!( float_eq!(a - b, 0., abs <= 0.000_01, ulps <= 10) );
///
/// let c = [1.000_000_2f32, -2.0];
/// let d = [1.0f32, -2.000_002];
/// assert!( float_eq!(c, d, abs_all <= 0.000_000_1, ulps <= [2, 8]) );
/// ```
///
/// [`FloatEq`]: trait.FloatEq.html
/// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
/// [relative epsilon comparison]: index.html#relative-epsilon-comparison
/// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
/// [composite types]: index.html#comparing-composite-types
#[macro_export]
macro_rules! float_eq {
    ($a:expr, $b:expr, $($eq:ident <= $max_diff:expr),+) => ({
        match (&$a, &$b) {
            (a_val, b_val) => {
                false $(|| $crate::FloatEqCmp::$eq(a_val, b_val, &$max_diff))+
            }
        }
    });
    ($a:expr, $b:expr, $($eq:ident <= $max_diff:expr),+,) => ({
        $crate::float_eq!($a, $b $(, $eq <= $max_diff)+)
    })
}

/// Checks if two floating point expressions are not equal to each other.
///
/// Comparisons are applied in order from left to right, shortcutting to return
/// early if a positive result is found.
///
/// - `abs <= max_diff` is an [absolute epsilon comparison].
/// - `rel <= max_diff` is a [relative epsilon comparison].
/// - `ulps <= max_diff` is an [ULPs comparison].
///
/// When comparing [composite types], variants that use a uniform `max_diff`
/// value across all fields are also available:
///
/// - `abs_all <= max_diff` is an [absolute epsilon comparison].
/// - `rel_all <= max_diff` is a [relative epsilon comparison].
/// - `ulps_all <= max_diff` is an [ULPs comparison].
///
/// # Examples
/// ```
/// # use float_eq::float_ne;
/// # use std::f32;
/// let a: f32 = 4.;
/// let b: f32 = 4.1;
///
/// assert!( float_ne!(a, 3.999_999, rel <= f32::EPSILON) );
/// assert!( float_ne!(a - b, 0., abs <= 0.000_01, rel <= f32::EPSILON) );
/// assert!( float_ne!(a - b, 0., abs <= 0.000_01, ulps <= 10) );
///
/// let c = [1.000_000_2f32, -2.0];
/// let d = [1.0f32, -2.000_002];
/// assert!( float_ne!(c, d, abs_all <= 0.000_000_1, ulps <= [2, 7]) );
/// ```
///
/// [`FloatEq`]: trait.FloatEq.html
/// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
/// [relative epsilon comparison]: index.html#relative-epsilon-comparison
/// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
/// [composite types]: index.html#comparing-composite-types
#[macro_export]
macro_rules! float_ne {
    ($a:expr, $b:expr, $($eq:ident <= $max_diff:expr),+) => ({
        !$crate::float_eq!($a, $b $(, $eq <= $max_diff)+)
    });
    ($a:expr, $b:expr, $($eq:ident <= $max_diff:expr),+,) => ({
        !$crate::float_eq!($a, $b $(, $eq <= $max_diff)+)
    });
}

/// Asserts that two floating point expressions are equal to each other.
///
/// - `abs <= max_diff` is an [absolute epsilon comparison].
/// - `rel <= max_diff` is a [relative epsilon comparison].
/// - `ulps <= max_diff` is an [ULPs comparison].
///
/// When comparing [composite types], variants that use a uniform `max_diff`
/// value across all fields are also available:
///
/// - `abs_all <= max_diff` is an [absolute epsilon comparison].
/// - `rel_all <= max_diff` is a [relative epsilon comparison].
/// - `ulps_all <= max_diff` is an [ULPs comparison].
///
/// On panic, this macro will print the values of the expressions with their debug
/// representations, with additional information from the comparison operations
/// (using [`FloatEqDebug`], [`FloatEqAllDebug`] and [`FloatDiff`]).
///
/// Like [`assert!`], this macro has a second form, where a custom panic message can
/// be provided.
///
/// # Examples
/// ```
/// # use float_eq::assert_float_eq;
/// # use std::f32;
/// let a: f32 = 4.;
/// let b: f32 = 4.000_002_5;
///
/// assert_float_eq!(a, 3.999_999_8, rel <= f32::EPSILON);
/// assert_float_eq!(a - b, 0., abs <= 0.000_01, rel <= f32::EPSILON);
/// assert_float_eq!(a - b, 0., abs <= 0.000_01, ulps <= 10);
///
/// assert_float_eq!(a - b, 0., abs <= 0.000_01, ulps <= 10, "Checking that {} == {}", a, b);
///
/// let c = [1.000_000_2f32, -2.0];
/// let d = [1.0f32, -2.000_002];
/// assert_float_eq!(c, d, abs_all <= 0.000_000_1, ulps <= [2, 8]);
/// ```
///
/// [`assert!`]: https://doc.rust-lang.org/std/macro.assert.html
/// [`float_eq!`]: macro.float_eq.html
/// [`FloatEq`]: trait.FloatEq.html
/// [`FloatEqDebug`]: trait.FloatEqDebug.html
/// [`FloatEqAllDebug`]: trait.FloatEqAllDebug.html
/// [`FloatDiff`]: trait.FloatDiff.html
/// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
/// [relative epsilon comparison]: index.html#relative-epsilon-comparison
/// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
/// [composite types]: index.html#comparing-composite-types
#[macro_export]
macro_rules! assert_float_eq {
    // the order of these rules matters a *lot* for the format string functionality
    // to work, otherwise we end up consuming the general case too early.
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr, $eq2:ident <= $max_diff_2:expr, $eq3:ident <= $max_diff_3:expr) => ({
        match (&$left, &$right, &$max_diff_1, &$max_diff_2, &$max_diff_3) {
            (left_val, right_val, max_diff_1_val, max_diff_2_val, max_diff_3_val) => {
                if !$crate::float_eq!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val,
                    $eq2 <= *max_diff_2_val,
                    $eq3 <= *max_diff_3_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_eq!(left, right, ", stringify!($eq1), " <= ε, ", stringify!($eq2), " <= ε, ", stringify!($eq3), r#" <= ε)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} ε: `{:?}`,
{:>10} ε: `{:?}`,
{:>10} ε: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &*right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpOpEpsilon::$eq2(&*left_val, &*right_val, &*max_diff_2_val),
                        concat!("[", stringify!($eq3), "]"),
                        $crate::FloatCmpOpEpsilon::$eq3(&*left_val, &*right_val, &*max_diff_3_val)
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr, $eq2:ident <= $max_diff_2:expr) => ({
        match (&$left, &$right, &$max_diff_1, &$max_diff_2) {
            (left_val, right_val, max_diff_1_val, max_diff_2_val) => {
                if !$crate::float_eq!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val,
                    $eq2 <= *max_diff_2_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_eq!(left, right, ", stringify!($eq1), " <= ε, ", stringify!($eq2), r#" <= ε)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} ε: `{:?}`,
{:>10} ε: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &*right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpOpEpsilon::$eq2(&*left_val, &*right_val, &*max_diff_2_val),
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr) => ({
        match (&$left, &$right, &$max_diff_1) {
            (left_val, right_val, max_diff_1_val) => {
                if !$crate::float_eq!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_eq!(left, right, ", stringify!($eq1), r#" <= ε)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} ε: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &*right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $($eq:ident <= $max_diff:expr,)+) => ({
        $crate::assert_float_eq!($left, $right $(, $eq <= $max_diff)+)
    });
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr, $eq2:ident <= $max_diff_2:expr, $eq3:ident <= $max_diff_3:expr, $($arg:tt)+) => ({
        match (&$left, &$right, &$max_diff_1, &$max_diff_2, &$max_diff_3) {
            (left_val, right_val, max_diff_1_val, max_diff_2_val, max_diff_3_val) => {
                if !$crate::float_eq!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val,
                    $eq2 <= *max_diff_2_val,
                    $eq3 <= *max_diff_3_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_eq!(left, right, ", stringify!($eq1), " <= ε, ", stringify!($eq2), " <= ε, ", stringify!($eq3), r#" <= ε)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} ε: `{:?}`,
{:>10} ε: `{:?}`,
{:>10} ε: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &*right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpOpEpsilon::$eq2(&*left_val, &*right_val, &*max_diff_2_val),
                        concat!("[", stringify!($eq3), "]"),
                        $crate::FloatCmpOpEpsilon::$eq3(&*left_val, &*right_val, &*max_diff_3_val),
                        format_args!($($arg)+)
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr, $eq2:ident <= $max_diff_2:expr, $($arg:tt)+) => ({
        match (&$left, &$right, &$max_diff_1, &$max_diff_2) {
            (left_val, right_val, max_diff_1_val, max_diff_2_val) => {
                if !$crate::float_eq!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val,
                    $eq2 <= *max_diff_2_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_eq!(left, right, ", stringify!($eq1), " <= ε, ", stringify!($eq2), r#" <= ε)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} ε: `{:?}`,
{:>10} ε: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &*right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpOpEpsilon::$eq2(&*left_val, &*right_val, &*max_diff_2_val),
                        format_args!($($arg)+)
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr, $($arg:tt)+) => ({
        match (&$left, &$right, &$max_diff_1) {
            (left_val, right_val, max_diff_1_val) => {
                if !$crate::float_eq!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_eq!(left, right, ", stringify!($eq1), r#" <= ε)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} ε: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &*right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                        format_args!($($arg)+)
                    )
                }
            }
        }
    });
}

/// Asserts that two floating point expressions are not equal to each other.
///
/// - `abs <= max_diff` is an [absolute epsilon comparison].
/// - `rel <= max_diff` is a [relative epsilon comparison].
/// - `ulps <= max_diff` is an [ULPs comparison].
///
/// When comparing [composite types], variants that use a uniform `max_diff`
/// value across all fields are also available:
///
/// - `abs_all <= max_diff` is an [absolute epsilon comparison].
/// - `rel_all <= max_diff` is a [relative epsilon comparison].
/// - `ulps_all <= max_diff` is an [ULPs comparison].
///
/// On panic, this macro will print the values of the expressions with their debug
/// representations, with additional information from the comparison operations
/// (using [`FloatEqDebug`], [`FloatEqAllDebug`] and [`FloatDiff`]).
///
/// Like [`assert!`], this macro has a second form, where a custom panic message can
/// be provided.
///
/// # Examples
/// ```
/// # use float_eq::assert_float_ne;
/// # use std::f32;
/// let a: f32 = 4.;
/// let b: f32 = 4.1;
///
/// assert_float_ne!(a, 3.9999990, rel <= f32::EPSILON);
/// assert_float_ne!(a - b, 0., abs <= 0.00001, rel <= f32::EPSILON);
/// assert_float_ne!(a - b, 0., abs <= 0.00001, ulps <= 10);
///
/// assert_float_ne!(a - b, 0., abs <= 0.00001, ulps <= 10, "Checking that {} != {}", a, b);
///
/// let c = [1.000_000_2f32, -2.0];
/// let d = [1.0f32, -2.000_002];
/// assert_float_ne!(c, d, abs_all <= 0.000_000_1, ulps <= [2, 7]);
/// ```
///
/// [`assert!`]: https://doc.rust-lang.org/std/macro.assert.html
/// [`float_ne!`]: macro.float_ne.html
/// [`FloatEq`]: trait.FloatEq.html
/// [`FloatEqDebug`]: trait.FloatEqDebug.html
/// [`FloatEqAllDebug`]: trait.FloatEqAllDebug.html
/// [`FloatDiff`]: trait.FloatDiff.html
/// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
/// [relative epsilon comparison]: index.html#relative-epsilon-comparison
/// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
/// [composite types]: index.html#comparing-composite-types
#[macro_export]
macro_rules! assert_float_ne {
    // the order of these rules matters a *lot* for the format string functionality
    // to work, otherwise we end up consuming the general case too early.
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr, $eq2:ident <= $max_diff_2:expr, $eq3:ident <= $max_diff_3:expr) => ({
        match (&$left, &$right, &$max_diff_1, &$max_diff_2, &$max_diff_3) {
            (left_val, right_val, max_diff_1_val, max_diff_2_val, max_diff_3_val) => {
                if !$crate::float_ne!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val,
                    $eq2 <= *max_diff_2_val,
                    $eq3 <= *max_diff_3_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_ne!(left, right, ", stringify!($eq1), " <= ε, ", stringify!($eq2), " <= ε, ", stringify!($eq3), r#" <= ε)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} ε: `{:?}`,
{:>10} ε: `{:?}`,
{:>10} ε: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &*right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpOpEpsilon::$eq2(&*left_val, &*right_val, &*max_diff_2_val),
                        concat!("[", stringify!($eq3), "]"),
                        $crate::FloatCmpOpEpsilon::$eq3(&*left_val, &*right_val, &*max_diff_3_val)
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr, $eq2:ident <= $max_diff_2:expr) => ({
        match (&$left, &$right, &$max_diff_1, &$max_diff_2) {
            (left_val, right_val, max_diff_1_val, max_diff_2_val) => {
                if !$crate::float_ne!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val,
                    $eq2 <= *max_diff_2_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_ne!(left, right, ", stringify!($eq1), " <= ε, ", stringify!($eq2), r#" <= ε)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} ε: `{:?}`,
{:>10} ε: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &*right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpOpEpsilon::$eq2(&*left_val, &*right_val, &*max_diff_2_val),
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr) => ({
        match (&$left, &$right, &$max_diff_1) {
            (left_val, right_val, max_diff_1_val) => {
                if !$crate::float_ne!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_ne!(left, right, ", stringify!($eq1), r#" <= ε)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} ε: `{:?}`"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &*right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $($eq:ident <= $max_diff:expr,)+) => ({
        $crate::assert_float_ne!($left, $right $(, $eq <= $max_diff)+)
    });
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr, $eq2:ident <= $max_diff_2:expr, $eq3:ident <= $max_diff_3:expr, $($arg:tt)+) => ({
        match (&$left, &$right, &$max_diff_1, &$max_diff_2, &$max_diff_3) {
            (left_val, right_val, max_diff_1_val, max_diff_2_val, max_diff_3_val) => {
                if !$crate::float_ne!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val,
                    $eq2 <= *max_diff_2_val,
                    $eq3 <= *max_diff_3_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_ne!(left, right, ", stringify!($eq1), " <= ε, ", stringify!($eq2), " <= ε, ", stringify!($eq3), r#" <= ε)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} ε: `{:?}`,
{:>10} ε: `{:?}`,
{:>10} ε: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &*right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpOpEpsilon::$eq2(&*left_val, &*right_val, &*max_diff_2_val),
                        concat!("[", stringify!($eq3), "]"),
                        $crate::FloatCmpOpEpsilon::$eq3(&*left_val, &*right_val, &*max_diff_3_val),
                        format_args!($($arg)+)
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr, $eq2:ident <= $max_diff_2:expr, $($arg:tt)+) => ({
        match (&$left, &$right, &$max_diff_1, &$max_diff_2) {
            (left_val, right_val, max_diff_1_val, max_diff_2_val) => {
                if !$crate::float_ne!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val,
                    $eq2 <= *max_diff_2_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_ne!(left, right, ", stringify!($eq1), " <= ε, ", stringify!($eq2), r#" <= ε)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} ε: `{:?}`,
{:>10} ε: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &*right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                        concat!("[", stringify!($eq2), "]"),
                        $crate::FloatCmpOpEpsilon::$eq2(&*left_val, &*right_val, &*max_diff_2_val),
                        format_args!($($arg)+)
                    )
                }
            }
        }
    });
    ($left:expr, $right:expr, $eq1:ident <= $max_diff_1:expr, $($arg:tt)+) => ({
        match (&$left, &$right, &$max_diff_1) {
            (left_val, right_val, max_diff_1_val) => {
                if !$crate::float_ne!(
                    *left_val,
                    *right_val,
                    $eq1 <= *max_diff_1_val) {
                    // The reborrows below are intentional. See assert_eq! in the standard library.
                    panic!(concat!(
"assertion failed: `float_ne!(left, right, ", stringify!($eq1), r#" <= ε)`
        left: `{:?}`,
       right: `{:?}`,
    abs_diff: `{:?}`,
   ulps_diff: `{:?}`,
{:>10} ε: `{:?}`: {}"#),
                        &*left_val,
                        &*right_val,
                        $crate::FloatDiff::abs_diff(&*left_val, &*right_val),
                        $crate::FloatDiff::ulps_diff(&*left_val, &*right_val),
                        concat!("[", stringify!($eq1), "]"),
                        $crate::FloatCmpOpEpsilon::$eq1(&*left_val, &*right_val, &*max_diff_1_val),
                        format_args!($($arg)+)
                    )
                }
            }
        }
    });
}

/// Asserts that two floating point expressions are equal to each other.
///
/// - `abs <= max_diff` is an [absolute epsilon comparison].
/// - `rel <= max_diff` is a [relative epsilon comparison].
/// - `ulps <= max_diff` is an [ULPs comparison].
///
/// When comparing [composite types], variants that use a uniform `max_diff`
/// value across all fields are also available:
///
/// - `abs_all <= max_diff` is an [absolute epsilon comparison].
/// - `rel_all <= max_diff` is a [relative epsilon comparison].
/// - `ulps_all <= max_diff` is an [ULPs comparison].
///
/// On panic, this macro will print the values of the expressions with their debug
/// representations, with additional information from the comparison operations
/// (using [`FloatEqDebug`], [`FloatEqAllDebug`] and [`FloatDiff`]).
///
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
/// let a: f32 = 4.;
/// let b: f32 = 4.0000025;
///
/// debug_assert_float_eq!(a, 3.9999998, rel <= f32::EPSILON);
/// debug_assert_float_eq!(a - b, 0., abs <= 0.00001, rel <= f32::EPSILON);
/// debug_assert_float_eq!(a - b, 0., abs <= 0.00001, ulps <= 10);
///
/// debug_assert_float_eq!(a - b, 0., abs <= 0.00001, ulps <= 10, "Checking that {} == {}", a, b);
/// ```
///
/// [`assert_float_eq!`]: macro.assert_float_eq.html
/// [`assert!`]: https://doc.rust-lang.org/std/macro.assert.html
/// [`debug_assert_eq!`]: https://doc.rust-lang.org/std/macro.debug_assert_eq.html
/// [`float_eq!`]: macro.float_eq.html
/// [`FloatEq`]: trait.FloatEq.html
/// [`FloatEqDebug`]: trait.FloatEqDebug.html
/// [`FloatEqAllDebug`]: trait.FloatEqAllDebug.html
/// [`FloatDiff`]: trait.FloatDiff.html
/// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
/// [relative epsilon comparison]: index.html#relative-epsilon-comparison
/// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
/// [composite types]: index.html#comparing-composite-types
#[macro_export]
macro_rules! debug_assert_float_eq {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_float_eq!($($arg)*); })
}

/// Asserts that two floating point expressions are not equal to each other.
///
/// - `abs <= max_diff` is an [absolute epsilon comparison].
/// - `rel <= max_diff` is a [relative epsilon comparison].
/// - `ulps <= max_diff` is an [ULPs comparison].
///
/// When comparing [composite types], variants that use a uniform `max_diff`
/// value across all fields are also available:
///
/// - `abs_all <= max_diff` is an [absolute epsilon comparison].
/// - `rel_all <= max_diff` is a [relative epsilon comparison].
/// - `ulps_all <= max_diff` is an [ULPs comparison].
///
/// On panic, this macro will print the values of the expressions with their debug
/// representations, with additional information from the comparison operations
/// (using [`FloatEqDebug`], [`FloatEqAllDebug`] and [`FloatDiff`]).
///
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
/// let a: f32 = 4.;
/// let b: f32 = 4.1;
///
/// debug_assert_float_ne!(a, 3.9999990, rel <= f32::EPSILON);
/// debug_assert_float_ne!(a - b, 0., abs <= 0.00001, rel <= f32::EPSILON);
/// debug_assert_float_ne!(a - b, 0., abs <= 0.00001, ulps <= 10);
///
/// debug_assert_float_ne!(a - b, 0., abs <= 0.00001, ulps <= 10, "Checking that {} == {}", a, b);
/// ```
///
/// [`assert_float_ne!`]: macro.assert_float_ne.html
/// [`assert!`]: https://doc.rust-lang.org/std/macro.assert.html
/// [`debug_assert_ne!`]: https://doc.rust-lang.org/std/macro.debug_assert_ne.html
/// [`float_ne!`]: macro.float_ne.html
/// [`FloatEq`]: trait.FloatEq.html
/// [`FloatEqDebug`]: trait.FloatEqDebug.html
/// [`FloatEqAllDebug`]: trait.FloatEqAllDebug.html
/// [`FloatDiff`]: trait.FloatDiff.html
/// [absolute epsilon comparison]: index.html#absolute-epsilon-comparison
/// [relative epsilon comparison]: index.html#relative-epsilon-comparison
/// [ULPs comparison]: index.html#units-in-the-last-place-ulps-comparison
/// [composite types]: index.html#comparing-composite-types
#[macro_export]
macro_rules! debug_assert_float_ne {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_float_ne!($($arg)*); })
}

#[doc(hidden)]
pub struct FloatEqCmp;

#[doc(hidden)]
impl FloatEqCmp {
    #[inline]
    pub fn abs<A: ?Sized, B: ?Sized>(a: &A, b: &B, max_diff: &A::Epsilon) -> bool
    where
        A: FloatEq<B>,
    {
        a.eq_abs(b, max_diff)
    }

    #[inline]
    pub fn abs_all<A: ?Sized, B: ?Sized>(a: &A, b: &B, max_diff: &A::Epsilon) -> bool
    where
        A: FloatEqAll<B>,
    {
        a.eq_abs_all(b, max_diff)
    }

    #[inline]
    pub fn rel<A: ?Sized, B: ?Sized>(a: &A, b: &B, max_diff: &A::Epsilon) -> bool
    where
        A: FloatEq<B>,
    {
        a.eq_rel(b, max_diff)
    }

    #[inline]
    pub fn rel_all<A: ?Sized, B: ?Sized>(a: &A, b: &B, max_diff: &A::Epsilon) -> bool
    where
        A: FloatEqAll<B>,
    {
        a.eq_rel_all(b, max_diff)
    }

    #[inline]
    pub fn ulps<A: ?Sized, B: ?Sized>(a: &A, b: &B, max_diff: &Ulps<A::Epsilon>) -> bool
    where
        A: FloatEq<B>,
    {
        a.eq_ulps(b, max_diff)
    }

    #[inline]
    pub fn ulps_all<A: ?Sized, B: ?Sized>(a: &A, b: &B, max_diff: &Ulps<A::Epsilon>) -> bool
    where
        A: FloatEqAll<B>,
    {
        a.eq_ulps_all(b, max_diff)
    }
}

#[doc(hidden)]
pub struct FloatCmpOpEpsilon;

#[doc(hidden)]
impl FloatCmpOpEpsilon {
    #[inline]
    pub fn abs<A: ?Sized, B: ?Sized>(a: &A, b: &B, max_diff: &A::Epsilon) -> A::DebugEpsilon
    where
        A: FloatEq<B> + FloatEqDebug<B>,
    {
        a.debug_abs_epsilon(b, max_diff)
    }

    #[inline]
    pub fn abs_all<A: ?Sized, B: ?Sized>(a: &A, b: &B, max_diff: &A::Epsilon) -> A::DebugEpsilon
    where
        A: FloatEqAll<B> + FloatEqAllDebug<B>,
    {
        a.debug_abs_all_epsilon(b, max_diff)
    }

    #[inline]
    pub fn rel<A: ?Sized, B: ?Sized>(a: &A, b: &B, max_diff: &A::Epsilon) -> A::DebugEpsilon
    where
        A: FloatEq<B> + FloatEqDebug<B>,
    {
        a.debug_rel_epsilon(b, max_diff)
    }

    #[inline]
    pub fn rel_all<A: ?Sized, B: ?Sized>(a: &A, b: &B, max_diff: &A::Epsilon) -> A::DebugEpsilon
    where
        A: FloatEqAll<B> + FloatEqAllDebug<B>,
    {
        a.debug_rel_all_epsilon(b, max_diff)
    }

    #[inline]
    pub fn ulps<A: ?Sized, B: ?Sized>(
        a: &A,
        b: &B,
        max_diff: &Ulps<A::Epsilon>,
    ) -> Ulps<A::DebugEpsilon>
    where
        A: FloatEq<B> + FloatEqDebug<B>,
    {
        a.debug_ulps_epsilon(b, max_diff)
    }

    #[inline]
    pub fn ulps_all<A: ?Sized, B: ?Sized>(
        a: &A,
        b: &B,
        max_diff: &Ulps<A::Epsilon>,
    ) -> Ulps<A::DebugEpsilon>
    where
        A: FloatEqAll<B> + FloatEqAllDebug<B>,
    {
        a.debug_ulps_all_epsilon(b, max_diff)
    }
}
