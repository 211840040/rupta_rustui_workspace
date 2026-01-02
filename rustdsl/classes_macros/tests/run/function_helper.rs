use classes::prelude::*;
classes::classes! {
    class TestSuper {
        struct {
            z: i32,
            w: i32,
        }
        fn new(z: i32, w: i32 = 5) -> Self {
            Self {
                z,
                w,
            }
        }

        fn two_w(z: i32 = 1) -> Self {
            Self {
                z,
                w: 2,
            }
        }

        #[builder = "free_w"]
        fn two_z(w: i32 = 1) -> Self {
            Self {
                z: 2,
                w,
            }
        }
    }
    class Test extends TestSuper{
        struct {
            x: i32,
            y: i32,
        }
        fn new(x: i32, y: i32 = 5, z: i32 = 3, w: i32 = 3) -> Self {
            Self {
                super: Super::builder(z).w(w).build(),
                x,
                y
            }
        }

        fn one_y(x: i32 = 5) -> Self {
            Self {
                super: Super::two_w_builder().build(),
                x,
                y: 1
            }
        }

        #[builder = "free_y"]
        fn one_x(y: i32 = 5) -> Self {
            Self {
                super: Super::free_w().build(),
                x: 1,
                y
            }
        }

        fn x(&self) -> i32 {
            self.get_x()
        }

        fn y(&self) -> i32 {
            self.get_y()
        }

        fn z(&self) -> i32 {
            self.get_z()
        }

        fn w(&self) -> i32 {
            self.get_w()
        }

        #[helper = "x_rec_helper"]
        fn cal_x_rectangle(&self, y: i32 = 5) -> i32 {
            self.x() * y
        }

        #[helper = "x_ref_helper"]
        fn test_reference(&self, x: &i32, y: Option<&i32> = None) -> i32 {
            *y.unwrap_or(x)
        }
    }
}

#[test]
fn test_builder() {
    let sample = Test::builder(1).y(2).z(4).build();
    assert_eq!(sample.x(), 1);
    assert_eq!(sample.y(), 2);
    assert_eq!(sample.z(), 4);
    assert_eq!(sample.w(), 3);

    let sample = Test::builder(1).y(2).z(4).w(5).build();
    assert_eq!(sample.x(), 1);
    assert_eq!(sample.y(), 2);
    assert_eq!(sample.z(), 4);
    assert_eq!(sample.w(), 5);

    let sample = Test::builder(1).y(2).build();
    assert_eq!(sample.x(), 1);
    assert_eq!(sample.y(), 2);
    assert_eq!(sample.z(), 3);
    assert_eq!(sample.w(), 3);

    let sample = Test::builder(1).build();
    assert_eq!(sample.x(), 1);
    assert_eq!(sample.y(), 5);
    assert_eq!(sample.z(), 3);
    assert_eq!(sample.w(), 3);

    let sample = Test::builder(1).x(3).y(4).build();
    assert_eq!(sample.x(), 3);
    assert_eq!(sample.y(), 4);
    assert_eq!(sample.z(), 3);
    assert_eq!(sample.w(), 3);

    let sample = Test::one_y_builder().x(7).build();
    assert_eq!(sample.x(), 7);
    assert_eq!(sample.y(), 1);
    assert_eq!(sample.z(), 1);
    assert_eq!(sample.w(), 2);

    let sample = Test::free_y().y(4).build();
    assert_eq!(sample.x(), 1);
    assert_eq!(sample.y(), 4);
    assert_eq!(sample.z(), 2);
    assert_eq!(sample.w(), 1);
}

#[test]
fn test_helper() {
    let sample = Test::builder(3).build();

    let result = sample.x_rec_helper().call();
    assert_eq!(result, 15);

    let result = sample.x_rec_helper().y(7).call();
    assert_eq!(result, 21);

    let result = sample.x_rec_helper().y(9).call();
    assert_eq!(result, 27);

    let result = sample.x_ref_helper(&10).call();
    assert_eq!(result, 10);

    let result = sample.x_ref_helper(&10).y(Some(&20)).call();
    assert_eq!(result, 20);
}
