use classes_macros::classes;

classes! {
    class Dance {
        pub fn dance(&self) {
            println!("Dance::dance");
        }
    }

    class Shape extends Dance {
        pub fn draw(&self) {
            println!("Shape::draw");
        }
    }

    class Circle extends Shape {
        pub override fn dance(&self) {
            //~ ERROR[E0609]: no field `dance` on type `&mut _Shape::vtable::opt::Shape`
            println!("Circle::dance");
        }
    }
}

fn main() {}
