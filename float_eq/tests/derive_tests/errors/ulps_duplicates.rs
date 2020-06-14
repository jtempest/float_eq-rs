use float_eq::FloatUlps;

#[derive(Debug, PartialEq, FloatUlps)]
#[float_eq(ulps = "One", ulps = "Two")]
struct MyComplex32 {
    re: f32,
    im: f32,
}

fn main() {}
