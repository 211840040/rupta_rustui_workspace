use classes_macros::classes;

classes! {
    class A {
        struct {
            id: usize,
        }

        pub fn new(id: usize) -> Self {
            Self { id }
        }

        const fn x(&self) {}
        //~^ ERROR: `const fn` is not supported yet
        async fn y(&self) {}
        //~^ ERROR: `async fn` is not supported yet
        final fn z(&self) {}
        //~^ ERROR: `final fn` is not supported yet
    }
}

fn main() {}