use crate::test_utils::{BUF, printlntb};

::classes::classes! {
    abstract class I {
        pub fn i(&self);
    }
    abstract class J {
        pub fn j(&self);
    }
    class A {
        pub fn new() -> Self { Self {} }
        pub fn f(&self) { printlntb!("A::f"); }
    }
    #[with(A)]
    mixin M on A implements I, J {
        pub override fn A::f(&self) { super.f(); printlntb!("M::f"); }
        pub override fn I::i(&self) {            printlntb!("M::i"); }
        pub override fn J::j(&self) {            printlntb!("M::j"); }
    }
    class B extends A with M {
        pub fn new() -> Self { Self { super: Super::new(), .. } }
        pub override fn        A::f(&self) { super.f(); printlntb!("B::f"); }
        pub override fn <M as I>::i(&self) { super.i(); printlntb!("B::i"); }
        pub override fn <M as J>::j(&self) { super.j(); printlntb!("B::j"); }
    }
}

static EXPECTED: &[&str] = &["A::f", "M::f", "B::f", "M::i", "B::i", "M::j", "B::j"];

#[test]
fn test_multi_mixin() {
    let b = B::new();
    b.f();
    b.i();
    b.j();
    assert_eq!(BUF.take(), EXPECTED);
}
