use float_eq::FloatDiff;

//------------------------------------------------------------------------------
// MyComplex32
//------------------------------------------------------------------------------
#[derive(Debug)]
struct MyComplex32 {
    re: f32,
    im: f32,
}

//------------------------------------------------------------------------------
// FloatDiff with Self
//------------------------------------------------------------------------------
#[derive(Debug)]
struct MyComplex32UlpsDiff {
    re: u32,
    im: u32,
}

impl FloatDiff for MyComplex32 {
    type AbsDiff = Self;
    type UlpsDiff = MyComplex32UlpsDiff;

    fn abs_diff(&self, other: &Self) -> Self::AbsDiff {
        MyComplex32 {
            re: self.re.abs_diff(&other.re),
            im: self.im.abs_diff(&other.im),
        }
    }

    fn ulps_diff(&self, other: &Self) -> Self::UlpsDiff {
        MyComplex32UlpsDiff {
            re: self.re.ulps_diff(&other.re),
            im: self.im.ulps_diff(&other.im),
        }
    }
}

#[test]
fn float_diff_self() {
    let a = MyComplex32 {
        re: 1.0,
        im: 2.0000036,
    };
    let b = MyComplex32 {
        re: 1.0000001,
        im: 2.0,
    };

    let abs_diff = a.abs_diff(&b);
    assert_eq!(abs_diff.re, 0.00000011920929);
    assert_eq!(abs_diff.im, 0.0000035762787);

    let ulps_diff = a.ulps_diff(&b);
    assert_eq!(ulps_diff.re, 1);
    assert_eq!(ulps_diff.im, 15);
}

//------------------------------------------------------------------------------
// FloatDiff with f32
//------------------------------------------------------------------------------
impl FloatDiff<f32> for MyComplex32 {
    type AbsDiff = MyComplex32;
    type UlpsDiff = MyComplex32UlpsDiff;

    fn abs_diff(&self, other: &f32) -> Self::AbsDiff {
        MyComplex32 {
            re: self.re.abs_diff(other),
            im: self.im.abs_diff(&0.0),
        }
    }

    fn ulps_diff(&self, other: &f32) -> Self::UlpsDiff {
        MyComplex32UlpsDiff {
            re: self.re.ulps_diff(other),
            im: self.im.ulps_diff(&0.0),
        }
    }
}

impl FloatDiff<MyComplex32> for f32 {
    type AbsDiff = <MyComplex32 as FloatDiff<f32>>::AbsDiff;
    type UlpsDiff = <MyComplex32 as FloatDiff<f32>>::UlpsDiff;

    fn abs_diff(&self, other: &MyComplex32) -> Self::AbsDiff {
        other.abs_diff(self)
    }

    fn ulps_diff(&self, other: &MyComplex32) -> Self::UlpsDiff {
        other.ulps_diff(self)
    }
}

#[test]
fn float_diff_f32() {
    let a = 1.0000001_f32;
    let b = MyComplex32 {
        re: 1.0,
        im: 2.0000036,
    };

    let abs_diff = a.abs_diff(&b);
    assert_eq!(abs_diff.re, 0.00000011920929);
    assert_eq!(abs_diff.im, 2.0000036);

    let ulps_diff = a.ulps_diff(&b);
    assert_eq!(ulps_diff.re, 1);
    assert_eq!(ulps_diff.im, 1_073_741_839);
}

//TODO: FloatEq with Self
//TODO: FloatEq with f32

//TODO: FloatEqDebug with Self
//TODO: FloatEqDebug with f32

// assert_float_eq!/assert_float_ne! with Self
// assert_float_eq!/assert_float_ne! with f32
