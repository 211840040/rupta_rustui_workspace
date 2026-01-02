use crate::test_utils::{BUF, printlntb};

::classes::classes! {
    class A {
        pub fn new() -> Self { Self {} }
        pub fn f(&self) { printlntb!("A::f"); }
    }
    abstract class I {
        pub fn i(&self);
    }
    #[with(A)]
    mixin M1 implements I {
        pub fn g(&self) { printlntb!("M1::g"); }
    }
    #[with(A/M1)]
    mixin M2 on M1 {
        pub override fn M1::g(&self) { super.g(); printlntb!("M2::g"); }
        pub          fn     h(&self) { printlntb!("M2::h"); }
        pub override fn <M1 as I>::i(&self) { printlntb!("M2::i"); }
    }
    class B extends A with M1, M2 {
        pub fn new() -> Self { Self { super: Super::new(), .. } }
        pub override fn  A::f(&self) { super.f(); printlntb!("B::f"); }
        pub override fn M1::g(&self) { super.g(); printlntb!("B::g"); }
        pub override fn M2::h(&self) { super.h(); printlntb!("B::h"); }
    }
}

static EXPECTED: &[&str] = &[
    // b.f()
    "A::f", "B::f", //
    // b.g()
    "M1::g", "M2::g", "B::g", //
    // b.h()
    "M2::h", "B::h", //
    // b.i()
    "M2::i", //
    // m2.g()
    "M1::g", "M2::g", "B::g", //
    // m2.h()
    "M2::h", "B::h", //
    // m2.i()
    "M2::i", //
    // m1.g()
    "M1::g", "M2::g", "B::g", //
    // i.i()
    "M2::i", //
];

#[test]
fn test_mixin_on_mixin() {
    let b = B::new();
    b.f();
    b.g();
    b.h();
    b.i();
    let m2 = b.cast_mixin::<CRc<M2>>();
    m2.g();
    m2.h();
    m2.i();
    let m1 = m2.cast_mixin::<CRc<M1>>();
    m1.g();
    let i = m1.mixin_to_impl::<CRc<I>>();
    i.i();
    assert_eq!(BUF.take(), EXPECTED);
}
