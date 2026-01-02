use crate::test_utils::{BUF, printlntb};
use classes_macros::classes;

// Test case: Class1 extends Class2 extends Class3
classes! {
    class Class3 {
        struct {
            value3: usize,
        }

        pub fn new(value3: usize) -> Self {
            printlntb!("Class3::constructor");
            Self { value3 }
        }

        pub fn print(&self) {
            printlntb!("{}", self.get_value3());
        }
    }

    class Class2 extends Class3 {
        struct {
            value2: usize,
        }

        pub fn new(value3: usize, value2: usize) -> Self {
            let self = Self { super: Super::new(value3), value2 };
            printlntb!("Class2::constructor");
            self
        }

        pub fn print(&self) {
            printlntb!("{}", self.get_value2());
        }
    }

    class Class1 extends Class2 {
        struct {
            value1: usize,
        }

        pub fn new(value3: usize, value2: usize, value1: usize) -> Self {
            let self = Self { super: Super::new(value3, value2), value1 };
            printlntb!("Class1::constructor");
            self
        }

        pub fn print(&self) {
            printlntb!("{}", self.get_value1());
        }
    }
}

static EXPECTED: &[&str] = &[
    "Class3::constructor",
    "Class2::constructor",
    "Class1::constructor",
    "3",
];

#[test]
fn extends_extends_chain() {
    let class1 = Class1::new(1, 2, 3);
    class1.print();
    assert_eq!(BUF.take(), EXPECTED);
}
