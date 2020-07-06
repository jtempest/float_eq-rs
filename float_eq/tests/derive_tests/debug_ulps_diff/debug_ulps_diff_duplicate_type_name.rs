use float_eq::FloatEqDebugUlpsDiff;

#[derive(Debug, Clone, Copy, PartialEq, FloatEqDebugUlpsDiff)]
#[float_eq(debug_ulps_diff = "Name1", debug_ulps_diff = "Name2")]
struct MyComplex32 {
    re: f32,
    im: f32,
}

fn main() {}
