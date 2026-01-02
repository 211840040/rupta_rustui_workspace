//! 最简单的DSL程序: 只包含类的new操作

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
    let _p = Point::new(10, 20);
}

