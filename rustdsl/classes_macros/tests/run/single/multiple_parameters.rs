use crate::test_utils::{BUF, printlntb};
use classes_macros::classes;

classes! {
    class Animal {
        struct {
            final name: String,
            age: usize,
            gender: bool,
        }

        pub fn new(name: String, age: usize, gender: bool) -> Self {
            printlntb!("Animal::constructor");
            Self { name, age, gender }
        }
    }
}

static EXPECTED: &[&str] = &["Animal::constructor"];

#[test]
fn multiple_parameters() {
    let _animal = Animal::new("Dog".to_string(), 10, true);
    assert_eq!(BUF.take(), EXPECTED);
}
