use pretty_assertions::assert_eq;

::classes::classes! {
    abstract class Y {
        pub fn f14(&self);
        pub fn f16(&self);
        pub fn f18(&self);
        pub fn f20(&self);
        pub fn f22(&self);
        pub fn f24(&self);
        pub fn f25(&self);
        pub fn f26(&self);
        pub fn f27(&self);
        pub fn f28(&self);
        pub fn f29(&self);
        pub fn f30(&self);
    }
    abstract class K extends Y {
        pub fn f13(&self);
        pub fn f15(&self);
        pub fn f17(&self);
        pub fn f19(&self);
        pub fn f21(&self);
        pub fn f23(&self);
    }
    abstract class X implements K {}
    abstract class J extends X {
        pub fn f07(&self);
        pub fn f08(&self);
        pub fn f09(&self);
        pub fn f10(&self);
        pub fn f11(&self);
        pub fn f12(&self);
    }
    abstract class I implements J {
        pub fn f05(&self);
    }
    abstract class A implements J {
        pub fn f06(&self);
    }
    #[with(B)]
    mixin M implements J {
        pub fn f01(&self);
        pub fn f02(&self);
        pub fn f04(&self);
    }
    abstract class B extends A implements J {
        pub fn f03(&self);
    }
    class C extends B with M implements I {
        override fn                                   f01(&self) {}
        override fn Super                           ::f02(&self) {}
        override fn B                               ::f03(&self) {}
        override fn M                               ::f04(&self) {}
        override fn I                               ::f05(&self) {}
        override fn A                               ::f06(&self) {}
        override fn <Self as J>                     ::f07(&self) {}
        override fn <Super as J>                    ::f08(&self) {}
        override fn <B as J>                        ::f09(&self) {}
        override fn <M as J>                        ::f10(&self) {}
        override fn <I as J>                        ::f11(&self) {}
        override fn <A as J>                        ::f12(&self) {}
        override fn <Self as <J as X> as K>         ::f13(&self) {}
        override fn <Self as J as <K as Y>>         ::f14(&self) {}
        override fn <Super as <J as X> as K>        ::f15(&self) {}
        override fn <Super as J as <K as Y>>        ::f16(&self) {}
        override fn <B as <J as X> as K>            ::f17(&self) {}
        override fn <B as J as <K as Y>>            ::f18(&self) {}
        override fn <M as <J as X> as K>            ::f19(&self) {}
        override fn <M as J as <K as Y>>            ::f20(&self) {}
        override fn <I as <J as X> as K>            ::f21(&self) {}
        override fn <I as J as <K as Y>>            ::f22(&self) {}
        override fn <A as <J as X> as K>            ::f23(&self) {}
        override fn <A as J as <K as Y>>            ::f24(&self) {}
        override fn <Self as <J as X> as <K as Y>>  ::f25(&self) {}
        override fn <Super as <J as X> as <K as Y>> ::f26(&self) {}
        override fn <B as <J as X> as <K as Y>>     ::f27(&self) {}
        override fn <M as <J as X> as <K as Y>>     ::f28(&self) {}
        override fn <I as <J as X> as <K as Y>>     ::f29(&self) {}
        override fn <A as <J as X> as <K as Y>>     ::f30(&self) {}
    }
}

#[test]
fn test_override() {
    assert_eq!(
        format!("{:#?}", C::vtable().debug_vtable_layout()),
        "(
    [
        MixinVtableHeader {
            instance: vtable_layout::overrides::B_M,
            mixin_offset: 24,
            super_offset: 480,
            data_offset: 0,
        },
    ],
    C {
        'start: 0,
        super: B_M {
            'start: 0,
            super: B {
                'start: 0,
                super: A {
                    'start: 0,
                    super: Object {
                        'start: 0,
                        header: ObjectHeader {
                            object_ty: vtable_layout::overrides::C,
                        },
                        'end: 16,
                    },
                    f06: 16,
                    J: J {
                        'start: 24,
                        super: X {
                            'start: 24,
                            super: Object {
                                'start: 24,
                                header: ClassHeader {
                                    class_ty: vtable_layout::overrides::J,
                                    offset: -24,
                                },
                                'end: 40,
                            },
                            K: K {
                                'start: 40,
                                super: Y {
                                    'start: 40,
                                    super: Object {
                                        'start: 40,
                                        header: ClassHeader {
                                            class_ty: vtable_layout::overrides::K,
                                            offset: -40,
                                        },
                                        'end: 56,
                                    },
                                    f14: 56,
                                    f16: 64,
                                    f18: 72,
                                    f20: 80,
                                    f22: 88,
                                    f24: 96,
                                    f25: 104,
                                    f26: 112,
                                    f27: 120,
                                    f28: 128,
                                    f29: 136,
                                    f30: 144,
                                    'end: 152,
                                },
                                f13: 152,
                                f15: 160,
                                f17: 168,
                                f19: 176,
                                f21: 184,
                                f23: 192,
                                'end: 200,
                            },
                            'end: 200,
                        },
                        f07: 200,
                        f08: 208,
                        f09: 216,
                        f10: 224,
                        f11: 232,
                        f12: 240,
                        'end: 248,
                    },
                    'end: 248,
                },
                f03: 248,
                J: J {
                    'start: 256,
                    super: X {
                        'start: 256,
                        super: Object {
                            'start: 256,
                            header: ClassHeader {
                                class_ty: vtable_layout::overrides::J,
                                offset: -256,
                            },
                            'end: 272,
                        },
                        K: K {
                            'start: 272,
                            super: Y {
                                'start: 272,
                                super: Object {
                                    'start: 272,
                                    header: ClassHeader {
                                        class_ty: vtable_layout::overrides::K,
                                        offset: -272,
                                    },
                                    'end: 288,
                                },
                                f14: 288,
                                f16: 296,
                                f18: 304,
                                f20: 312,
                                f22: 320,
                                f24: 328,
                                f25: 336,
                                f26: 344,
                                f27: 352,
                                f28: 360,
                                f29: 368,
                                f30: 376,
                                'end: 384,
                            },
                            f13: 384,
                            f15: 392,
                            f17: 400,
                            f19: 408,
                            f21: 416,
                            f23: 424,
                            'end: 432,
                        },
                        'end: 432,
                    },
                    f07: 432,
                    f08: 440,
                    f09: 448,
                    f10: 456,
                    f11: 464,
                    f12: 472,
                    'end: 480,
                },
                'end: 480,
            },
            f01: 480,
            f02: 488,
            f04: 496,
            J: J {
                'start: 504,
                super: X {
                    'start: 504,
                    super: Object {
                        'start: 504,
                        header: ClassHeader {
                            class_ty: vtable_layout::overrides::J,
                            offset: -504,
                        },
                        'end: 520,
                    },
                    K: K {
                        'start: 520,
                        super: Y {
                            'start: 520,
                            super: Object {
                                'start: 520,
                                header: ClassHeader {
                                    class_ty: vtable_layout::overrides::K,
                                    offset: -520,
                                },
                                'end: 536,
                            },
                            f14: 536,
                            f16: 544,
                            f18: 552,
                            f20: 560,
                            f22: 568,
                            f24: 576,
                            f25: 584,
                            f26: 592,
                            f27: 600,
                            f28: 608,
                            f29: 616,
                            f30: 624,
                            'end: 632,
                        },
                        f13: 632,
                        f15: 640,
                        f17: 648,
                        f19: 656,
                        f21: 664,
                        f23: 672,
                        'end: 680,
                    },
                    'end: 680,
                },
                f07: 680,
                f08: 688,
                f09: 696,
                f10: 704,
                f11: 712,
                f12: 720,
                'end: 728,
            },
            'end: 728,
        },
        I: I {
            'start: 728,
            super: Object {
                'start: 728,
                header: ClassHeader {
                    class_ty: vtable_layout::overrides::I,
                    offset: -728,
                },
                'end: 744,
            },
            f05: 744,
            J: J {
                'start: 752,
                super: X {
                    'start: 752,
                    super: Object {
                        'start: 752,
                        header: ClassHeader {
                            class_ty: vtable_layout::overrides::J,
                            offset: -752,
                        },
                        'end: 768,
                    },
                    K: K {
                        'start: 768,
                        super: Y {
                            'start: 768,
                            super: Object {
                                'start: 768,
                                header: ClassHeader {
                                    class_ty: vtable_layout::overrides::K,
                                    offset: -768,
                                },
                                'end: 784,
                            },
                            f14: 784,
                            f16: 792,
                            f18: 800,
                            f20: 808,
                            f22: 816,
                            f24: 824,
                            f25: 832,
                            f26: 840,
                            f27: 848,
                            f28: 856,
                            f29: 864,
                            f30: 872,
                            'end: 880,
                        },
                        f13: 880,
                        f15: 888,
                        f17: 896,
                        f19: 904,
                        f21: 912,
                        f23: 920,
                        'end: 928,
                    },
                    'end: 928,
                },
                f07: 928,
                f08: 936,
                f09: 944,
                f10: 952,
                f11: 960,
                f12: 968,
                'end: 976,
            },
            'end: 976,
        },
        'end: 976,
    },
)"
    );
}
