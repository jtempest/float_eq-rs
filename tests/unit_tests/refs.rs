// The commented variants in these tests require specialisation or the traits to
// be implemented differently in other ways.

use float_eq::assert_float_eq;

#[test]
fn refs() {
    //assert_float_eq!(1.0, &2.0, abs <= 1.0);
    //assert_float_eq!(1.0, &2.0, abs_all <= 1.0);
    //assert_float_eq!(&1.0, 2.0, abs <= 1.0);
    //assert_float_eq!(&1.0, 2.0, abs_all <= 1.0);
    assert_float_eq!(&1.0, &2.0, abs <= 1.0);
    assert_float_eq!(&1.0, &2.0, abs_all <= 1.0);
    //assert_float_eq!(&1.0, &2.0, abs <= &1.0);
    //assert_float_eq!(&1.0, &2.0, abs_all <= &1.0);

    let a = 1.0;
    //assert_float_eq!(a, &1.0, abs <= 1.0);
    //assert_float_eq!(a, &1.0, abs_all <= 1.0);
    //assert_float_eq!(&a, 1.0, abs <= 1.0);
    //assert_float_eq!(&a, 1.0, abs_all <= 1.0);
    assert_float_eq!(&a, &1.0, abs <= 1.0);
    assert_float_eq!(&a, &1.0, abs_all <= 1.0);
    //assert_float_eq!(&a, &1.0, abs <= &1.0);
    //assert_float_eq!(&a, &1.0, abs_all <= &1.0);

    let b = 2.0;
    //assert_float_eq!(a, &b, abs <= 1.0);
    //assert_float_eq!(a, &b, abs_all <= 1.0);
    //assert_float_eq!(&a, b, abs <= 1.0);
    //assert_float_eq!(&a, b, abs_all <= 1.0);
    assert_float_eq!(&a, &b, abs <= 1.0);
    assert_float_eq!(&a, &b, abs_all <= 1.0);
    //assert_float_eq!(&a, &b, abs <= &1.0);
    //assert_float_eq!(&a, &b, abs_all <= &1.0);
}

#[test]
#[allow(unused_mut)]
fn mutable() {
    let mut ma = 1.0;
    let mut mb = 2.0;
    assert_float_eq!(ma, mb, abs <= 1.0);
    assert_float_eq!(ma, mb, abs_all <= 1.0);
    //assert_float_eq!(&ma, mb, abs <= 1.0);
    //assert_float_eq!(&ma, mb, abs_all <= 1.0);
    //assert_float_eq!(ma, &mb, abs <= 1.0);
    //assert_float_eq!(ma, &mb, abs_all <= 1.0);
    assert_float_eq!(&ma, &mb, abs <= 1.0);
    assert_float_eq!(&ma, &mb, abs_all <= 1.0);

    let a = 1.0;
    assert_float_eq!(a, mb, abs <= 1.0);
    assert_float_eq!(a, mb, abs_all <= 1.0);
    //assert_float_eq!(&a, mb, abs <= 1.0);
    //assert_float_eq!(&a, mb, abs_all <= 1.0);
    //assert_float_eq!(a, &mb, abs <= 1.0);
    //assert_float_eq!(a, &mb, abs_all <= 1.0);
    assert_float_eq!(&a, &mb, abs <= 1.0);
    assert_float_eq!(&a, &mb, abs_all <= 1.0);

    let b = 2.0;
    assert_float_eq!(ma, b, abs <= 1.0);
    assert_float_eq!(ma, b, abs_all <= 1.0);
    //assert_float_eq!(&ma, mb, abs <= 1.0);
    //assert_float_eq!(&ma, mb, abs_all <= 1.0);
    //assert_float_eq!(ma, &mb, abs <= 1.0);
    //assert_float_eq!(ma, &mb, abs_all <= 1.0);
    assert_float_eq!(&ma, &b, abs <= 1.0);
    assert_float_eq!(&ma, &b, abs_all <= 1.0);
}

#[test]
#[allow(unused_mut)]
fn mutable_refs() {
    let mut a = 1.0;
    let mut b = 2.0;
    assert_float_eq!(&mut a, &b, abs <= 1.0);
    assert_float_eq!(&mut a, &b, abs_all <= 1.0);
    assert_float_eq!(&a, &mut b, abs <= 1.0);
    assert_float_eq!(&a, &mut b, abs_all <= 1.0);
    assert_float_eq!(&mut a, &mut b, abs <= 1.0);
    assert_float_eq!(&mut a, &mut b, abs_all <= 1.0);
}
