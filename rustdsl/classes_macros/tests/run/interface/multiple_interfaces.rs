use crate::test_utils::{BUF, printlntb};
use classes_macros::classes;

classes! {
    abstract class Eat {
        pub fn eat(&self);
    }

    abstract class Drink {
        pub fn drink(&self);
    }

    abstract class Sleep {
        pub fn sleep(&self);
    }

    class Cat implements Eat, Drink, Sleep {
        pub fn new() -> Self {
            Self {}
        }

        pub override fn Eat::eat(&self) {
            printlntb!("Cat::eat");
        }

        pub override fn Drink::drink(&self) {
            printlntb!("Cat::drink");
        }

        pub override fn Sleep::sleep(&self) {
            printlntb!("Cat::sleep");
        }
    }
}

static EXPECTED: &[&str] = &["Cat::eat", "Cat::drink", "Cat::sleep"];

#[test]
fn multiple_interfaces() {
    let cat = Cat::new();
    cat.eat();
    cat.drink();
    cat.sleep();
    assert_eq!(BUF.take(), EXPECTED);
}
