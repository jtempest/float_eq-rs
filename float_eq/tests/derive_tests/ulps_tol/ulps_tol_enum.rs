use float_eq::FloatEqUlpsTol;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, FloatEqUlpsTol)]
enum SomeEnum {
    Float(f32),
    Double(f64),
}

fn main() {}
