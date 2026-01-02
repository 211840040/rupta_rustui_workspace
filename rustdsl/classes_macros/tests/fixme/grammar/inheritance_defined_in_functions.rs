// FIXME: The error location contains the entire `classes! {}`
// FIXME: Error messages are not clear

use classes_macros::classes;

fn main() {
    classes! {
        //~^ ERROR: cannot find value `MODULE_PATH` in this scope
        class Animal {
            struct {
                id: usize,
            }

            pub fn new(id: usize) -> Self {
                Self { id }
            }
        }

        class Dog extends Animal {
            struct {
                final name: String,
            }

            pub fn new(id: usize, name: String) -> Self {
                Self { super: Super::new(id), name }
            }
        }
    }

    let _dog = Dog::new(1, "dog".to_string());
}
