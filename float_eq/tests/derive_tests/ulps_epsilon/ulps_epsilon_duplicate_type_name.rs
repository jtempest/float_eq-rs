use float_eq::FloatEqUlpsEpsilon;

#[derive(Debug, Clone, Copy, PartialEq, FloatEqUlpsEpsilon)]
#[float_eq(ulps_epsilon = "Name1", ulps_epsilon = "Name2")]
struct MyComplex32 {
    re: f32,
    im: f32,
}

fn main() {}
