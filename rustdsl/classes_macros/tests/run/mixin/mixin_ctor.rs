use crate::test_utils::{BUF, printlntb};

::classes::classes! {
    class A {
        pub fn new(x: i32) -> Self {
            printlntb!("A::new, before, x = {x}");
            let self = Self {};
            printlntb!("A::new, after, x = {x}");
            self
        }
    }
    class B {
        pub fn new(x: i32, y: u32) -> Self {
            printlntb!("B::new, before, x = {x}, y = {y}");
            let self = Self {};
            printlntb!("B::new, after, x = {x}, y = {y}");
            self
        }
    }
    #[with(A, B_M2)]
    mixin M1 {
        struct {
            final x: String = "M1".to_string(),
        }
    }
    #[with(A/M1, B)]
    mixin M2 {
        struct {
            final y: String = "M2".to_string(),
        }
    }
    class C extends A with M1, M2 {
        pub fn new() -> Self {
            printlntb!("C::new, before");
            let self = Self { super: Super::new(1_i32) };
            printlntb!("C::new, after, x = {}, y = {}", self.get_x(), self.get_y());
            self

        }
    }
    class D extends B with M2, M1 {
        pub fn new() -> Self {
            printlntb!("D::new, before");
            let self = Self { super: Super::new(1_i32, 2_u32) };
            printlntb!("D::new, after, x = {}, y = {}", self.get_x(), self.get_y());
            self
        }
    }
}

static EXPECTED: &[&str] = &[
    // C::new()
    "C::new, before",
    "A::new, before, x = 1",
    "A::new, after, x = 1",
    "C::new, after, x = M1, y = M2", //
    // D::new()
    "D::new, before",
    "B::new, before, x = 1, y = 2",
    "B::new, after, x = 1, y = 2",
    "D::new, after, x = M1, y = M2", //
];

#[test]
fn test_mixin_on_mixin() {
    let _ = C::new();
    let _ = D::new();
    assert_eq!(BUF.take(), EXPECTED);
}
