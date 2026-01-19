use classes_macros::classes;

mod lack_interface_0 {
    use super::*;
    classes! {
        class Dance {
            pub fn dance(&self) {
                println!("Dance::dance");
            }
        }

        class Shape {
            pub fn draw(&self) {
                println!("Shape::draw");
            }
        }

        class Circle extends Shape {
            //~^ ERROR[E0599] could not evaluate static initializer
            pub override fn Dance::dance(&self) {
                println!("Circle::dance");
            }
        }
    }
}

mod lack_interface_1 {
    use super::*;
    classes! {
        class Dance {
            pub fn dance(&self) {
                println!("Dance::dance");
            }
        }

        class Shape {
            pub fn draw(&self) {
                println!("Shape::draw");
            }
        }

        class Circle extends Shape {
            //~^ ERROR[E0599] could not evaluate static initializer
            pub override fn <Self as Dance>::dance(&self) {
                println!("Circle::dance");
            }
        }
    }
}

fn main() {}
