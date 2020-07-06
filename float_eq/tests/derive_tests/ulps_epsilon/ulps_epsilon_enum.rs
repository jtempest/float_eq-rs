use float_eq::FloatEqUlpsEpsilon;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, FloatEqUlpsEpsilon)]
enum SomeEnum {
    Float(f32),
    Double(f64),
}

fn main() {}
