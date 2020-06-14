use float_eq::FloatEqAll;

#[derive(FloatEqAll)]
#[float_eq(ulps = "MyTypeUlps")]
struct MyType {
    a: f64,
    b: f64,
}

struct MyTypeUlps {
    a: u64,
    b: u64,
}

fn main() {}
