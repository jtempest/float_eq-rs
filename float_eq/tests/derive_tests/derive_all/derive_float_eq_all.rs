use float_eq::{assert_float_eq, derive_float_eq};

#[derive_float_eq(ulps = "PointUlps", all_epsilon = "f32")]
#[derive(Debug, PartialEq)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z }
    }
}

fn main() {
    let a = Point::new(1.0, -2.0, 4.0);
    let b = Point::new(1.000_000_1, -2.000_000_5, 4.000_0002);
    assert_float_eq!(a, b, abs <= Point::new(0.000_000_2, 0.000_000_6, 0.000_003));
    assert_float_eq!(a, b, abs_all <= 0.000_003);
    assert_float_eq!(a, b, ulps <= PointUlps { x: 1, y: 2, z: 3 });
    assert_float_eq!(a, b, ulps_all <= 3);
}
