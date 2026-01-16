//! 包含 load 和 store 行为的 DSL 程序，用于验证指针信息通过 load/store 的传播

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
    }
    
    class Container {
        struct {
            late point: CRc<Point>,  // 指向 Point 实例的字段，使用 late 修饰符允许后续设置
        }
        
        // 创建一个空的 Container，point 字段稍后通过 set_point 设置
        pub fn new() -> Self {
            Self { .. }
        }
    }
}

fn main() {
    // 创建第一个 Point 实例 _p1
    let _p1 = Point::new(10, 20);
    
    // 创建第二个 Point 实例 _p2
    let _p2 = Point::new(30, 40);
    
    // 创建 Container 实例 _c1
    let mut _c1 = Container::new();
    
    // STORE 操作：将 _p1 存储到 _c1 的 point 字段
    // 在 PAG 中会建立 StorePAGEdge: _p1 --Store(point)--> _c1.point
    // 这会触发 set_point() 方法，将 _p1 的 points-to 信息传播到 _c1.point
    _c1.set_point(_p1);
    
    // LOAD 操作：从 _c1 的 point 字段加载值到 _p3
    // 在 PAG 中会建立 LoadPAGEdge: _c1.point --Load(point)--> _p3
    // 这会触发 get_point() 方法，将 _c1.point 的 points-to 信息传播到 _p3
    let _p3 = _c1.get_point();
    
    // 创建第二个 Container 实例 _c2
    let mut _c2 = Container::new();
    
    // STORE 操作：将 _p2 存储到 _c2 的 point 字段
    _c2.set_point(_p2);
    
    // LOAD 操作：从 _c2 的 point 字段加载值到 _p4
    let _p4 = _c2.get_point();
    
    // 使用变量，确保它们在分析结果中出现（不会被优化掉）
    std::mem::drop(_p3);
    std::mem::drop(_p4);
    std::mem::drop(_c1);
    std::mem::drop(_c2);
    
    // 预期的传播链：
    // 1. STORE: _p1 --Store(point)--> _c1.point (通过 set_point)
    // 2. LOAD:  _c1.point --Load(point)--> _p3 (通过 get_point)
    //    因此 _p3 应该指向 _p1 指向的同一个 HeapObj
    // 3. STORE: _p2 --Store(point)--> _c2.point (通过 set_point)
    // 4. LOAD:  _c2.point --Load(point)--> _p4 (通过 get_point)
    //    因此 _p4 应该指向 _p2 指向的同一个 HeapObj
}
