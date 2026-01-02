use crate::test_utils::{BUF, printlntb};

::classes::classes! {
    class A {
        struct {
            x: u32 = 0_u32,
        }
        pub fn new() -> Self {
            let self = Self { .. };
            self.f();
            self
        }
        pub fn f(&self) {
            printlntb!("A::f, x = {}", self.get_x());
        }
    }

    class B extends A {
        struct {
            y: u32 = 1_u32,
        }
        pub fn new() -> Self {
            let self = Self { super: Super::new(), .. };
            self.f();
            self.g();
            self
        }
        pub override fn f(&self) {
            super.f();
            printlntb!("B::f, x = {}, y = {}", self.get_x(), self.get_y());
        }
        pub fn g(&self) {
            printlntb!("B::g, x = {}, y = {}", self.get_x(), self.get_y());
        }
    }
}

static EXPECTED: &[&str] = &[
    // A::new()
    "A::f, x = 0", // A::f()
    // A::new()
    "A::f, x = 0",        // A::f()
    "B::f, x = 0, y = 1", // B::f()
    // B::new()
    "A::f, x = 0",        // A::f()
    "B::f, x = 0, y = 1", // B::f()
    "B::g, x = 0, y = 1", // B::g()
];

#[test]
fn test_ctor() {
    let _a = A::new();
    let _b = B::new();
    assert_eq!(BUF.take(), EXPECTED);
}
