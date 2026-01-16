//! 包含类对象引用赋值（assign）操作的 DSL 程序

use classes_macros::classes;

classes! {
    class Point {
        struct {
            x: i32,
            y: i32,
        }

        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }
    }
}

fn main() {
    let _p1 = Point::new(10, 20);
    let _p2 = _p1; // Simple assignment
}
