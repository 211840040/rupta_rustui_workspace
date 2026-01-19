use classes_macros::classes;

classes! {
    class Dance {
        pub fn dance(&self) {
            println!("Dance::dance");
        }
    }

    class A {}

    class Shape extends A {
        pub fn draw(&self) {
            println!("Shape::draw");
        }
    }

    class Circle extends Shape {
        pub override fn <A as Dance>::dance(&self) {
            //~^ ERROR[E0599] could not evaluate static initializer
            println!("Circle::dance");
        }
    }
}

fn main() {}
