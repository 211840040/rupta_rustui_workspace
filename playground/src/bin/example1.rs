//! 示例1: 简单的类定义和继承
//!
//! 这是一个用rustdsl DSL语法编写的简单示例程序
//! 用于测试expand和rupta分析

use classes::prelude::CRc;
use classes_macros::classes;

classes! {
    // 基类 Animal
    class Animal {
        struct {
            // 非Copy类型需要使用 final 修饰符
            final name: String,
        }

        pub fn new(name: String) -> Self {
            Self { name }
        }

        pub fn speak(&self) {
            println!("Animal {} makes a sound", self.get_name());
        }
    }

    // 子类 Dog 继承自 Animal
    class Dog extends Animal {
        struct {
            final breed: String,
        }

        pub fn new(name: String, breed: String) -> Self {
            Self {
                super: Super::new(name),
                breed,
            }
        }

        pub override fn speak(&self) {
            println!("Dog {} barks!", self.get_name());
        }

        pub fn fetch(&self) {
            println!("{} the {} is fetching!", self.get_name(), self.get_breed());
        }
    }
}

fn main() {
    // 创建Dog实例
    let dog = Dog::new("Buddy".to_string(), "Golden Retriever".to_string());

    // 调用方法
    dog.speak();
    dog.fetch();

    // 向上转型为Animal
    let animal: CRc<Animal> = dog.into_superclass();
    animal.speak(); // 仍然调用Dog的override方法 (多态)
}
