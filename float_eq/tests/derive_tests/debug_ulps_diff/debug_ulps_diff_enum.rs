use float_eq::FloatEqDebugUlpsDiff;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, FloatEqDebugUlpsDiff)]
enum SomeEnum {
    Float(f32),
    Double(f64),
}

fn main() {}
