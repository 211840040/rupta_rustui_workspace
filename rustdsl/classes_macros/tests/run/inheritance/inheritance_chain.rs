use crate::test_utils::{BUF, printlntb};
use classes_macros::classes;

classes! {
    class Alice {
        struct {
            alice: usize,
        }

        pub fn new(alice: usize) -> Self {
            printlntb!("Alice::constructor");
            Self { alice }
        }
    }

    class Bob extends Alice {
        struct {
            bob: usize,
        }

        pub fn new(alice: usize, bob: usize) -> Self {
            let self = Self { super: Super::new(alice), bob };
            printlntb!("Bob::constructor");
            self
        }
    }

    class Carol extends Bob {
        struct {
            carol: usize,
        }

        pub fn new(alice: usize, bob: usize, carol: usize) -> Self {
            let self = Self { super: Super::new(alice, bob), carol };
            printlntb!("Carol::constructor");
            self
        }
    }

    class Dave extends Carol {
        struct {
            dave: usize,
        }

        pub fn new(alice: usize, bob: usize, carol: usize, dave: usize) -> Self {
            let self = Self { super: Super::new(alice, bob, carol), dave };
            printlntb!("Dave::constructor");
            self
        }
    }

}

static EXPECTED: &[&str] = &[
    "Alice::constructor",
    "Bob::constructor",
    "Carol::constructor",
    "Dave::constructor",
];

#[test]
fn inheritance_chain() {
    let _dave = Dave::new(1, 2, 3, 4);
    assert_eq!(BUF.take(), EXPECTED);
}
