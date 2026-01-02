use crate::test_utils::{BUF, printlntb};
use ::classes::prelude::CRc;
use classes_macros::classes;

classes! {
    abstract class Eat {
        pub fn eat(&self);
    }

    abstract class Drink {
        pub fn drink(&self);
    }

    class Cat implements Eat, Drink {
        pub fn new() -> Self {
            Self {}
        }

        pub override fn Eat::eat(&self) {
            printlntb!("Cat::eat");
        }

        pub override fn Drink::drink(&self) {
            printlntb!("Cat::drink");
        }

        pub fn meow(&self) {
            printlntb!("Cat::meow");
        }
    }
}

static EXPECTED: &[&str] = &["Cat::eat", "Cat::drink", "Cat::meow", "Cat::meow"];

#[test]
fn multiple_interfaces_casting() {
    let cat = Cat::new();
    let eater = cat.upcast::<CRc<Cat>, CRc<Eat>>();
    eater.eat();
    let drinker = cat.upcast::<CRc<Cat>, CRc<Drink>>();
    drinker.drink();
    let cat1 = drinker.downcast::<CRc<Cat>, CRc<Cat>>();
    cat1.meow();
    let cat2 = eater.downcast::<CRc<Cat>, CRc<Cat>>();
    cat2.meow();
    assert_eq!(BUF.take(), EXPECTED);
    assert_eq!(cat1, cat2);
}
