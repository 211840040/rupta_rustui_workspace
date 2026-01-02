use classes_macros::classes;

classes! {
    class A {
        struct {
            mutable id: usize,
        }

        pub override fn new(id: usize) -> Self {
            //~^ ERROR: unexpected `override` with no `extends` nor `implements`
            // FIXME: The report error is not the real error (Override Constructor)
            Self { id }
        }
    }
}

fn main() {}