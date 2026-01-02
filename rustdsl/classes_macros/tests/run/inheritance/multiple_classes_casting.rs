use crate::test_utils::{BUF, printlntb};
use ::classes::prelude::CRc;
use classes_macros::classes;

classes! {
    class Alice {
        pub fn new() -> Self {
            Self {}
        }

        pub fn eat(&self) {
            printlntb!("Alice::eat");
        }
    }

    class Bob extends Alice {
        pub fn new() -> Self {
            Self { super: Super::new() }
        }

        pub override fn Alice::eat(&self) {
            printlntb!("Bob::eat");
        }
    }

    class Carol extends Bob {
        pub fn new() -> Self {
            Self { super: Super::new() }
        }

        pub override fn Alice::eat(&self) {
            printlntb!("Carol::eat");
        }
    }
}

static EXPECTED: &[&str] = &["Carol::eat", "Carol::eat", "Carol::eat", "Carol::eat"];

#[test]
fn multiple_classes_casting() {
    let bob = Carol::new().into_superclass::<CRc<Bob>>();
    bob.eat();
    let alice = bob.as_superclass::<CRc<Alice>>();
    alice.eat();
    let bob = alice.as_subclass::<CRc<Bob>>();
    bob.eat();
    let carol = bob.as_subclass::<CRc<Carol>>();
    carol.eat();
    assert_eq!(BUF.take(), EXPECTED);
}
