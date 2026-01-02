// Description: different behavior between implements and extends an interface

use crate::test_utils::{BUF, printlntb};
use classes_macros::classes;

classes! {
    abstract class Shape {
        pub fn draw(&self);
        pub fn resize(&self, scale: f64);

        pub fn new() -> Self {
            Self {}
        }

        pub fn meow(&self) {
            printlntb!("Shape::meow");
        }
    }

    class Circle extends Shape {
        pub fn new() -> Self {
            Self { super: Super::new() }
        }

        pub override fn Shape::draw(&self) {
            printlntb!("Circle::draw");
        }
        pub override fn Shape::resize(&self, scale: f64) {
            printlntb!("Circle::resize");
        }
    }

    class Triangle implements Shape {
        pub fn new() -> Self {
            Self {}
        }

        pub override fn Shape::draw(&self) {
            printlntb!("Triangle::draw");
        }

        pub override fn Shape::resize(&self, scale: f64) {
            printlntb!("Triangle::resize");
        }

        pub override fn Shape::meow(&self) {
            printlntb!("Triangle::meow");
        }
    }

    static EXPECTED: &[&str] = &[
        "Circle::draw",
        "Circle::resize",
        "Shape::meow",
        "Triangle::draw",
        "Triangle::resize",
        "Triangle::meow",
    ];

    #[test]
    fn test() {
        let circle = Circle::new();
        circle.draw();
        circle.resize(1.0);
        circle.meow();
        let triangle = Triangle::new();
        triangle.draw();
        triangle.resize(1.0);
        triangle.meow();
        assert_eq!(BUF.take(), EXPECTED);
    }
}
