use classes_macros::classes;

classes! {
    abstract class Shape {
        pub fn draw(&self);
    }

    class Circle implements Shape {
        pub fn Shape::draw(&self) {
            //~^ ERROR: expect `override`
            println!("Circle::draw");
        }
    }
}

fn main() {}