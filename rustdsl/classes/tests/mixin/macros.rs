use core::cell::RefCell;

thread_local! {
    static BUF: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

use classes::prelude::*;

macro_rules! println {
    ($($args:tt)*) => {
        BUF.with_borrow_mut(|buf| {
            buf.push(format!($($args)*));
        })
    };
}

::classes::classes! {
    abstract class I { pub fn i(&self); }

    abstract class A {
        struct { final x: usize = 0 }
        pub fn new() -> Self { Self { .. } }
        pub fn f(&self) { println!("A::f, x = {}", self.get_x()); }
    }

    abstract class B extends A {
        struct { final y: usize = 1 }
        pub fn new() -> Self { Self { super: Super::new(), .. } }
        pub override fn f(&self) { super.f(); println!("B::f, y = {}", self.get_y()); }
        pub          fn g(&self) {            println!("B::g, y = {}", self.get_y()); }
    }

    #[with(A, B)]
    mixin M on A implements I {
        struct { final z: usize = 2 }
        pub override fn A::f(&self) { super.f(); println!("M::f, z = {}", self.get_z()); }
        pub          fn    h(&self) {            println!("M::h, z = {}", self.get_z()); }
    }

    class C1 extends A with M {
        struct { final w: usize = 3 }
        pub fn new() -> Self { Self { super: Super::new(), .. } }
        pub override fn A       ::f(&self) { super.f(); println!("C1::f, w = {}", self.get_w()); }
        pub override fn M       ::h(&self) { super.h(); println!("C1::h, w = {}", self.get_w()); }
        pub override fn <M as I>::i(&self) {            println!("C1::i, w = {}", self.get_w()); }
        pub          fn           j(&self) {            println!("C1::j, w = {}", self.get_w()); }
    }

    class C2 extends B with M {
        struct { final v: usize = 4 }
        pub fn new() -> Self { Self { super: Super::new(), .. } }
        pub override fn A       ::f(&self) { super.f(); println!("C2::f, v = {}", self.get_v()); }
        pub override fn B       ::g(&self) { super.g(); println!("C2::g, v = {}", self.get_v()); }
        pub override fn M       ::h(&self) { super.h(); println!("C2::h, v = {}", self.get_v()); }
        pub override fn <M as I>::i(&self) {            println!("C2::i, v = {}", self.get_v()); }
        pub          fn           j(&self) {            println!("C2::j, v = {}", self.get_v()); }
    }
}

#[test]
fn mixin() {
    BUF.take();
    let c1 = C1::new();
    let c2 = C2::new();

    c1.f();
    c1.h();
    c1.i();
    c1.j();

    let c1: CRc<M> = c1.to_mixin();
    c1.f();
    c1.h();
    println!("z = {}", c1.get_z());

    c2.f();
    c2.g();
    c2.h();
    c2.i();
    c2.j();

    let c2: CRc<M> = c2.to_mixin();
    c2.f();
    c2.h();
    println!("z = {}", c2.get_z());
    assert_eq!(BUF.take(), super::EXPECTED_OUTPUT);
}
