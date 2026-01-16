//! 包含多个类对象引用赋值操作的 DSL 程序，用于验证 assign 传播

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

        pub fn dummy(&self) -> i32 {
            self.x + self.y
        }
    }
}

fn main() {
    let _p1 = Point::new(10, 20);
    let _p2 = _p1;
    let _result1 = _p2.dummy(); // Use _p2
    let _p3 = Point::new(30, 40);
    let _p4 = _p3;
    let _result2 = _p4.dummy(); // Use _p4
    std::mem::drop(_result1);
    std::mem::drop(_result2);
}
