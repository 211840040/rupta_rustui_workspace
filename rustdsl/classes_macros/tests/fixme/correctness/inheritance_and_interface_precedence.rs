// FIXME: Error messages are not clear

use classes_macros::classes;

classes! {
    abstract class Animal {
        pub fn makeSound(&self) {
            println!("Animal::makeSound");
        }
    }

    class LivingThing {
        pub fn makeSound(&self) {
            println!("LivingThing::makeSound");
        }
    }

    class Dog extends LivingThing implements Animal {}
          //~^ ERROR: could not evaluate static initializer
}

fn main() {}