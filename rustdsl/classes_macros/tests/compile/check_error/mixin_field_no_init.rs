::classes::classes! {
    #[with(Object)]
    mixin M {
        struct {
            final x: String, //~ ERROR: missing field `x` in initializer of `Self`
        }
        fn new() -> Self { //~ ERROR: constructor in mixin is not supported now
            Self {
                super: Super::new(),
                ..
            }
        }
    }
}
