use float_eq::FloatUlps;

#[derive(Debug, PartialEq, FloatUlps)]
#[float_eq(ulps = "MyComplex32Ulps", cheese = "Hello")]
struct MyComplex32 {
    re: f32,
    im: f32,
}

fn main() {}
