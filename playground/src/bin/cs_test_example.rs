use classes::prelude::CRc;
use classes_macros::classes;

classes! {
    class Number {
        pub fn new() -> Self { Self {} }

        pub fn get(&self) -> i32{
            return 0;
        }
    }

    class One extends Number{
        pub fn new() -> Self { Self { super: Super::new() } }

        pub override fn get(&self) -> i32 {
            return 1;
        }
    }

    class Two extends Number{
        pub fn new() -> Self { Self { super: Super::new() } }

        pub override fn get(&self) -> i32 {
            return 2;
        }
    }

    class Cross{
        pub fn new() -> Self { Self {} }

        pub fn id(&self, n: CRc<Number>) -> CRc<Number> {
            return n;
        }
    }
}

fn main() {
    let one = One::new();
    let two = Two::new();
    let n1: CRc<Number> = one.as_super().clone();
    let n2: CRc<Number> = two.as_super().clone();
    let cross = Cross::new();
    let x = cross.id(n1);
    let y = cross.id(n2);
    println!("x{} y{}", x.get(), y.get());
}
