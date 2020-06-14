use float_eq::FloatUlps;

#[derive(Debug, PartialEq, FloatUlps)]
enum SomeEnum {
    Float(f32),
    Double(f64),
}

fn main() {}
