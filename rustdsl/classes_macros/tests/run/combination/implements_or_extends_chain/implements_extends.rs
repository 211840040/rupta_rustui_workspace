use crate::test_utils::{BUF, printlntb};
use classes_macros::classes;

// Test case: Class1 implements Class2 extends Class3
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

        pub override fn print(&self) {
            printlntb!("{}", self.get_value2());
        }

        pub fn double_print(&self) {
            self.print();
            self.print();
        }
    }

    class Class1 implements Class2 {
        struct {
            value1: usize,
        }

        pub fn new(value1: usize) -> Self {
            printlntb!("Class1::constructor");
            Self { value1 }
        }

        pub override fn <Self as <Class2 as Class3>>::print(&self) {
            // FIXME: ERROR(miri) Undefined Behavior: constructing invalid value: encountered a dangling reference (going beyond the bounds of its allocation)
            printlntb!("{}", self.get_value1());
        }

        pub override fn <Self as Class2>::double_print(&self) {
            self.print();
            self.print();
        }
    }
}

static EXPECTED: &[&str] = &["Class1::constructor", "1", "1", "1"];

#[test]
#[cfg_attr(miri, ignore)] // FIXME: undefined behavior in miri
fn implements_extends_chain() {
    let class1 = Class1::new(1);
    class1.print();
    class1.double_print();
    assert_eq!(BUF.take(), EXPECTED);
}
