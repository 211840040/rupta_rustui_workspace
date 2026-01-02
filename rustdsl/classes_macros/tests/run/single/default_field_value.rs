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
        pub fn new(gender: bool) -> Self {
            printlntb!("Animal::constructor");
            Self { gender, .. }
        }
    }
}

static EXPECTED: &[&str] = &["Animal::constructor"];

#[test]
fn default_field_value() {
    let animal = Animal::new(true);
    assert_eq!(BUF.take(), EXPECTED);
    assert_eq!(animal.get_name(), "");
    assert_eq!(animal.get_age(), 0);
    assert_eq!(animal.get_gender(), true);
}
