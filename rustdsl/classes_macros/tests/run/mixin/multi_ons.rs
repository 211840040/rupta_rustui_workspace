use crate::test_utils::{BUF, printlntb};

::classes::classes! {
    class A {
        pub fn new() -> Self { Self {} }
        pub fn f(&self) { printlntb!("A::f"); }
    }
    class B extends A {
        pub fn new() -> Self { Self { super: Super::new(), .. } }
        pub override fn f(&self) { super.f(); printlntb!("B::f"); }
        pub          fn g(&self) {            printlntb!("B::g"); }
    }
    #[with(B)]
    mixin M on A, B {
        pub override fn A::f(&self) { super.f(); printlntb!("M::f"); }
        pub override fn B::g(&self) { super.g(); printlntb!("M::g"); }
    }
    class C extends B with M {
        pub fn new() -> Self { Self { super: Super::new(), .. } }
        pub override fn A::f(&self) { super.f(); printlntb!("C::f"); }
        pub override fn B::g(&self) { super.g(); printlntb!("C::g"); }
    }
}

static EXPECTED: &[&str] = &["A::f", "B::f", "M::f", "C::f", "B::g", "M::g", "C::g"];

#[test]
fn test_multi_mixin() {
    let c = C::new();
    c.f();
    c.g();
    assert_eq!(BUF.take(), EXPECTED);
}
