use crate::test_utils::{BUF, printlntb};

::classes::classes! {
    class A {
        struct {
            x: u8 = 0_u8,
        }
        pub fn new() -> Self { Self { .. } }
        pub fn print_offsets(&self) {
            printlntb!("offset!(A, x): {}", unsafe { core::ptr::from_ref(self.raw_get_x()).byte_offset_from(&*self.0) });
        }
    }
    #[with(A)]
    mixin M on A {
        struct {
            y: u16 = 1_u16,
        }
        pub override fn A::print_offsets(&self) {
            super.print_offsets();
            printlntb!("offset!(A/M, x): {}", unsafe { core::ptr::from_ref(self.raw_get_x()).byte_offset_from(&*self.0) });
            printlntb!("offset!(A/M, y): {}", unsafe { core::ptr::from_ref(self.raw_get_y()).byte_offset_from(&*self.0) });
            let this = self.to_mixin();
            printlntb!("offset!(M, x): {}", unsafe { core::ptr::from_ref(this.mixin_to_impl::<CRc<A>>().raw_get_x()).byte_offset_from(&*this.0) });
            printlntb!("offset!(M, y): {}", unsafe { core::ptr::from_ref(this.raw_get_y()).byte_offset_from(&*this.0) });
        }
    }
    class B extends A with M {
        struct {
            z: u32 = 2_u32,
        }
        pub fn new() -> Self { Self { super: Super::new(), .. } }
        pub override fn A::print_offsets(&self) {
            super.print_offsets();
            printlntb!("offset!(B, x): {}", unsafe { core::ptr::from_ref(self.raw_get_x()).byte_offset_from(&*self.0) });
            printlntb!("offset!(B, y): {}", unsafe { core::ptr::from_ref(self.raw_get_y()).byte_offset_from(&*self.0) });
            printlntb!("offset!(B, z): {}", unsafe { core::ptr::from_ref(self.raw_get_z()).byte_offset_from(&*self.0) });
        }
    }
}

static EXPECTED: &[&str] = &[
    // <CRc<B>>::print_offsets
    "offset!(A, x): 0",
    "offset!(A/M, x): 0",
    "offset!(A/M, y): 2",
    "offset!(M, x): 0",
    "offset!(M, y): 2",
    "offset!(B, x): 0",
    "offset!(B, y): 2",
    "offset!(B, z): 4",
    // <CRc<B> as CRc<M>>::print_offsets
    "offset!(A, x): 0",
    "offset!(A/M, x): 0",
    "offset!(A/M, y): 2",
    "offset!(M, x): 0",
    "offset!(M, y): 2",
    "offset!(B, x): 0",
    "offset!(B, y): 2",
    "offset!(B, z): 4",
];

#[test]
fn test_get_set_alignment() {
    let b = B::new();
    b.print_offsets();
    b.cast_mixin::<CRc<M>>().print_offsets();
    assert_eq!(BUF.take(), EXPECTED);
}
