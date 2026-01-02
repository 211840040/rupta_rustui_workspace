use crate::test_utils::{BUF, printlntb};
use classes_macros::classes;

classes! {
    #[derive(Default)]
    class Animal {
        struct {
            final name: String = "".to_string(),
            age: usize = 0_usize,
            gender: bool,
        }

        pub fn new(gender: bool, name: String) -> Self {
            printlntb!("Animal::constructor");
            Self { gender, name, .. }
        }
    }
}

static EXPECTED: &[&str] = &["Animal::constructor"];

#[test]
fn override_default_field_value() {
    let animal = Animal::new(true, "Dog".to_string());
    assert_eq!(BUF.take(), EXPECTED);
    assert_eq!(animal.get_name(), "Dog");
    assert_eq!(animal.get_age(), 0);
    assert_eq!(animal.get_gender(), true);
}
