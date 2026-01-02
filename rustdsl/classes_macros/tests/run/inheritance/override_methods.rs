use crate::test_utils::{BUF, printlntb};
use classes_macros::classes;

classes! {
    class Shape {
        pub fn new() -> Self {
            printlntb!("Shape::constructor");
            Self {}
        }
        pub fn area(&self) {
            printlntb!("Shape::area");
        }
        pub fn perimeter(&self) {
            printlntb!("Shape::perimeter");
        }
        pub fn description(&self) {
            printlntb!("Shape::description");
        }
    }

    class Circle extends Shape {
        struct {
            radius: f64
        }
        pub fn new(radius: f64) -> Self {
            let self = Self { super: Super::new(), radius };
            printlntb!("Circle::constructor");
            self
        }
        pub override fn area(&self) {
            printlntb!("Circle::area");
        }
        pub override fn perimeter(&self) {
            printlntb!("Circle::perimeter");
        }
    }
}

static EXPECTED: &[&str] = &[
    "Shape::constructor",
    "Circle::constructor",
    "Circle::area",
    "Circle::perimeter",
    "Shape::description",
];

#[test]
fn override_methods() {
    let circle = Circle::new(10.0);
    circle.area();
    circle.perimeter();
    circle.description();
    assert_eq!(BUF.take(), EXPECTED);
}
