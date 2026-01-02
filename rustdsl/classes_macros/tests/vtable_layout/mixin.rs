use pretty_assertions::assert_eq;

::classes::classes! {
    class A {
        pub fn new() -> Self { Self {} }
        pub fn f(&self) {}
    }
    abstract class I {
        pub fn i(&self);
    }
    #[with(A)]
    mixin M1 implements I {
        pub fn g(&self) {}
    }
    #[with(A/M1)]
    mixin M2 on M1 {
        pub override fn M1::g(&self) {}
        pub          fn     h(&self) {}
        pub override fn <M1 as I>::i(&self) {}
    }
    class B extends A with M1, M2 {
        pub fn new() -> Self { Self { super: Super::new(), .. } }
        pub override fn  A::f(&self) {}
        pub override fn M1::g(&self) {}
        pub override fn M2::h(&self) {}
    }
}

#[test]
fn test_mixin() {
    assert_eq!(
        format!("{:#?}", B::vtable().debug_vtable_layout()),
        "(
    [
        MixinVtableHeader {
            instance: vtable_layout::mixin::A_M1_M2,
            mixin_offset: 48,
            super_offset: 56,
            data_offset: 0,
        },
        MixinVtableHeader {
            instance: vtable_layout::mixin::A_M1,
            mixin_offset: 24,
            super_offset: 24,
            data_offset: 0,
        },
    ],
    B {
        'start: 0,
        super: A_M1_M2 {
            'start: 0,
            super: A_M1 {
                'start: 0,
                super: A {
                    'start: 0,
                    super: Object {
                        'start: 0,
                        header: ObjectHeader {
                            object_ty: vtable_layout::mixin::B,
                        },
                        'end: 16,
                    },
                    f: 16,
                    'end: 24,
                },
                g: 24,
                I: I {
                    'start: 32,
                    super: Object {
                        'start: 32,
                        header: ClassHeader {
                            class_ty: vtable_layout::mixin::I,
                            offset: -32,
                        },
                        'end: 48,
                    },
                    i: 48,
                    'end: 56,
                },
                'end: 56,
            },
            h: 56,
            'end: 64,
        },
        'end: 64,
    },
)",
    );

    let b = B::new();
    assert_eq!(
        format!(
            "{:#?}",
            ::classes::class::ClassRcWeak::vtable(&b).debug_vtable_layout(0)
        ),
        "B {
    'start: 0,
    super: A_M1_M2 {
        'start: 0,
        super: A_M1 {
            'start: 0,
            super: A {
                'start: 0,
                super: Object {
                    'start: 0,
                    header: ObjectHeader {
                        object_ty: vtable_layout::mixin::B,
                    },
                    'end: 16,
                },
                f: 16,
                'end: 24,
            },
            g: 24,
            I: I {
                'start: 32,
                super: Object {
                    'start: 32,
                    header: ClassHeader {
                        class_ty: vtable_layout::mixin::I,
                        offset: -32,
                    },
                    'end: 48,
                },
                i: 48,
                'end: 56,
            },
            'end: 56,
        },
        h: 56,
        'end: 64,
    },
    'end: 64,
}",
    );

    let m2: CRc<M2> = b.into_supertype();
    assert_eq!(
        format!(
            "{:#?}",
            ::classes::class::ClassRcWeak::vtable(&m2).debug_vtable_layout()
        ),
        "(
    [
        MixinVtableHeader {
            instance: vtable_layout::mixin::A_M1_M2,
            mixin_offset: 48,
            super_offset: 56,
            data_offset: 0,
        },
        ..
    ],
    M2 {
        'start: 56,
        h: 56,
        'end: 64,
    },
)"
    );

    let m1: CRc<M1> = m2.into_supertype();
    assert_eq!(
        format!(
            "{:#?}",
            ::classes::class::ClassRcWeak::vtable(&m1).debug_vtable_layout()
        ),
        "(
    [
        MixinVtableHeader {
            instance: vtable_layout::mixin::A_M1,
            mixin_offset: 24,
            super_offset: 24,
            data_offset: 0,
        },
        ..
    ],
    M1 {
        'start: 24,
        g: 24,
        I: I {
            'start: 32,
            super: Object {
                'start: 32,
                header: ClassHeader {
                    class_ty: vtable_layout::mixin::I,
                    offset: -32,
                },
                'end: 48,
            },
            i: 48,
            'end: 56,
        },
        'end: 56,
    },
)"
    );
}
