use classes_macros::classes;

classes! {
    class A {
        struct {
            id: usize,
        }

        pub fn new(id: usize) -> Self {
            Self { id }
        }
    }
}

#[test]
fn class_name_is_captal_a() {
    let a = A::new(1);
    assert_eq!(a.get_id(), 1);
}
