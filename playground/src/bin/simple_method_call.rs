//! 包含普通方法调用的 DSL 程序，用于验证指针信息通过方法调用的传播
//!
//! 这个程序测试以下场景：
//! 1. 类实例的普通方法调用（非 getter/setter）
//! 2. 方法内部访问字段（通过 getter/setter）
//! 3. 方法返回类引用
//! 4. 方法参数传递类引用

use classes_macros::classes;
use classes::prelude::CRc;

classes! {
    class Point {
        struct {
            x: i32,
            y: i32,
        }

        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }
        
        // 普通方法：计算两个坐标的和（不返回新实例，只做计算）
        pub fn sum_coords(&self) -> i32 {
            self.get_x() + self.get_y()
        }
        
        // 普通方法：接受另一个 Point 作为参数，计算距离的平方（不返回新实例）
        pub fn distance_squared(&self, other: &CRc<Point>) -> i32 {
            let dx = self.get_x() - other.get_x();
            let dy = self.get_y() - other.get_y();
            dx * dx + dy * dy
        }
    }
    
    class Container {
        struct {
            late point: CRc<Point>,  // 指向 Point 实例的字段
        }
        
        pub fn new() -> Self {
            Self { .. }
        }
        
        // 普通方法：获取 point 字段并返回（内部调用 getter）
        pub fn get_internal_point(&self) -> CRc<Point> {
            self.get_point()
        }
        
        // 普通方法：对内部的 point 进行操作，返回计算结果
        pub fn get_point_sum(&self) -> i32 {
            let p = self.get_point();
            p.sum_coords()  // 调用 Point 的方法
        }
        
        // 普通方法：接受另一个 Point 作为参数，计算内部 point 和外部 point 的距离
        pub fn distance_to(&self, other: &CRc<Point>) -> i32 {
            let internal = self.get_point();
            internal.distance_squared(other)  // 调用 Point 的方法
        }
    }
}

fn main() {
    // 创建 Point 实例
    let _p1 = Point::new(10, 20);
    let _p2 = Point::new(30, 40);
    
    // 测试 1: 普通方法调用 - sum_coords（不接受参数）
    // 期望：方法调用应该被正确识别和处理
    let _sum1 = _p1.sum_coords();
    
    // 测试 2: 普通方法调用 - distance_squared（接受类引用参数）
    // 期望：方法调用应该被正确识别，参数传递应该正确
    let _dist = _p1.distance_squared(&_p2);
    
    // 创建 Container 实例并设置 point 字段
    let mut _c1 = Container::new();
    let mut _c2 = Container::new();
    
    // 使用 setter 设置 point 字段
    _c1.set_point(_p1);
    
    // 测试 3: 普通方法调用 - get_internal_point（内部调用 getter）
    // 期望：_c1.point 的 points-to 信息应该传播到 _p3
    let _p3 = _c1.get_internal_point();
    
    // 测试 4: 普通方法调用 - get_point_sum（内部调用 getter 和方法）
    // 期望：方法调用链应该被正确识别和处理
    let _sum2 = _c1.get_point_sum();
    
    // 测试 5: 普通方法调用 - distance_to（接受类引用参数，内部调用 getter 和方法）
    // 期望：方法调用链和参数传递应该被正确识别
    // 注意：在设置 _c2 之前使用 _p2，避免 move 问题
    let _dist2 = _c1.distance_to(&_p2);
    
    // 现在设置 _c2
    _c2.set_point(_p2);
    
    // 使用变量，确保它们在分析结果中出现
    let _ = _sum1;
    let _ = _dist;
    std::mem::drop(_p3);
    let _ = _sum2;
    let _ = _dist2;
    std::mem::drop(_c1);
    std::mem::drop(_c2);
    
    // 预期的传播链：
    // 1. _p1.sum_coords(): 普通方法调用，访问 _p1 的字段
    // 2. _p1.distance_squared(&_p2): 普通方法调用，接受类引用参数 _p2
    // 3. _c1.set_point(_p1) -> _c1.point: _p1 的 points-to 传播到 _c1.point
    // 4. _c1.get_internal_point() -> _p3: _c1.point 的 points-to 传播到 _p3
    // 5. _c1.get_point_sum(): 方法内部调用 get_point() 和 sum_coords()
    // 6. _c1.distance_to(&_p2): 方法内部调用 get_point() 和 distance_squared()
}
