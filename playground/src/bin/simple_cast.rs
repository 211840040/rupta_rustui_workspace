use classes::prelude::CRc;
use classes_macros::classes;

classes! {
    abstract class Eat {
        pub fn eat(&self);
    }

    class Alice implements Eat {
        pub fn new() -> Self { Self {} }

        pub override fn eat(&self) {
            println!("Alice::eat");
        }
    }

    class Bob extends Alice {
        pub fn new() -> Self {
            Self { super: Super::new() }
        }

        pub override fn <Super as Eat>::eat(&self) {
            println!("Bob::eat");
        }
    }

    class Carol extends Bob {
        pub fn new() -> Self {
            Self { super: Super::new() }
        }

        pub override fn <Alice as Eat>::eat(&self) {
            println!("Carol::eat");
        }
    }
}

fn main() {
    // 创建一个Carol类的实例
    let carol: CRc<Carol> = Carol::new();
    carol.eat(); // 输出：Carol::eat
                 // 使用`as_super`可以将`carol`向上转为其直接父类类型
    let bob: &CRc<Bob> = carol.as_super();
    bob.eat(); // 输出：Carol::eat
               // 使用`as_superclass`可以将`carol`向上转为其父/祖类类型
    let alice: &CRc<Alice> = carol.as_superclass();
    alice.eat(); // 输出：Carol::eat
                 // 使用`upcast`可以将`carol`向上转型为祖类`Alice`的实现的接口`Eat`类型
    let eat: CRc<Eat> = carol.upcast::<CRc<Alice>, _>();
    // 上述语句等价于
    // let eat: CRc<Eat> = alice.to_impl();
    eat.eat(); // 输出：Carol::eat
               // 使用`downcast`可以将`eat`经`Alice`类型向下转型为`Bob`类型
    let bob: CRc<Bob> = eat.downcast::<CRc<Alice>, _>();
}
