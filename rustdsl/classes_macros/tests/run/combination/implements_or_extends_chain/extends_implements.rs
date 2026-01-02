use crate::test_utils::{BUF, printlntb};
use classes_macros::classes;

// Test case: Class1 extends Class2 implements Class3
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

    class Class2 implements Class3 {
        struct {
            value2: usize,
        }

        pub fn new(value2: usize) -> Self {
            printlntb!("Class2::constructor");
            Self { value2 }
        }

        pub override fn Class3::print(&self) {
            printlntb!("{}", self.get_value2());
        }

        pub fn double_print(&self) {
            self.print();
            self.print();
        }
    }

    class Class1 extends Class2 {
        struct {
            value1: usize,
        }

        pub fn new(value1: usize, value2: usize) -> Self {
            let self = Self { super: Super::new(value2), value1 };
            printlntb!("Class1::constructor");
            self
        }

        pub override fn <Class2 as Class3>::print(&self) {
            printlntb!("{}", self.get_value1());
        }

        pub override fn Class2::double_print(&self) {
            self.print();
            self.print();
        }
    }
}

static EXPECTED: &[&str] = &["Class2::constructor", "Class1::constructor", "1", "1", "1"];

#[test]
fn extends_implements_schain() {
    let class1 = Class1::new(1, 2);
    class1.print();
    class1.double_print();
    assert_eq!(BUF.take(), EXPECTED);
}
