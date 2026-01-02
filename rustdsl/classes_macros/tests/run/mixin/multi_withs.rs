use crate::test_utils::{BUF, printlntb};

::classes::classes! {
    class A {
        pub fn new() -> Self { Self {} }
        pub fn f(&self) { printlntb!("A::f"); }
    }
    #[with(A)]
    mixin M1 on A {
        pub override fn A::f(&self) { super.f(); printlntb!("M1::f"); }
    }
    #[with(A, A/M1)]
    mixin M2 on A {
        pub override fn A::f(&self) { super.f(); printlntb!("M2::f"); }
    }
    class B extends A with M1, M2 {
        pub fn new() -> Self { Self { super: Super::new(), .. } }
        pub override fn A::f(&self) { super.f(); printlntb!("B::f"); }
    }
}

static EXPECTED: &[&str] = &["A::f", "M1::f", "M2::f", "B::f"];

#[test]
fn test_multi_mixin() {
    let b = B::new();
    b.f();
    assert_eq!(BUF.take(), EXPECTED);
}
