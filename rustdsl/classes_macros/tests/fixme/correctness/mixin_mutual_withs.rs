// FIXME: The error location contains the entire `classes! {}`
// FIXME: Error messages are not clear

::classes::classes! {
    class A {
        pub fn new() -> Self { Self {} }
        pub fn f(&self) { println!("A::f"); }
    }
    #[with(A, A/M2)]
    mixin M1 on A {
        pub override fn A::f(&self) { super.f(); println!("M1::f"); }
    }
    #[with(A, A/M1)]
    mixin M2 on A {
        pub override fn A::f(&self) { super.f(); println!("M2::f"); }
    }
    class B extends A with M1, M2 {
        pub fn new() -> Self { Self { super: Super::new(), .. } }
        pub override fn A::f(&self) { super.f(); println!("B::f"); }
    }
    class C extends A with M2, M1 {
        pub fn new() -> Self { Self { super: Super::new(), .. } }
        pub override fn A::f(&self) { super.f(); println!("C::f"); }
    }
}

fn main() {}
