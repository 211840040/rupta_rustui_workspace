//! 稍微复杂一点的DSL程序: 包含多个类的new操作

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
    // 创建多个 Point 实例，使用不同的参数
    let _p1 = Point::new(10, 20);
    let _p2 = Point::new(30, 40);
    let _p3 = Point::new(50, 60);
}


