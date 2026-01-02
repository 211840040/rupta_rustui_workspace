use pretty_assertions::assert_eq;

::classes::classes! {
    abstract class Listenable {
        pub fn new() -> Self { Self {} }
        pub fn add_listener(&self, f: fn());
        pub fn remove_listener(&self, f: fn());
    }

    pub abstract class PipelineManifold implements Listenable {
        pub fn semantics_enabled(&self) -> bool;
        pub fn request_visual_update(&self);
    }

    #[with(Object)]
    mixin ChangeNotifier implements Listenable {
        struct {
            mutable listeners: Vec<fn()> = Vec::new(),
        }
        pub fn has_listeners(&self) -> bool { true }
        pub fn dispose(&self) {}

        pub override fn Listenable::add_listener(&self, f: fn()) {}
        pub override fn Listenable::remove_listener(&self, f: fn()) {}

        pub fn notify_listeners(&self) {}
    }

    class BindingPipelineManifold extends Object with ChangeNotifier implements PipelineManifold {
        pub fn new() -> Self { Self { super: Super::new(), .. } }

        pub override fn PipelineManifold::semantics_enabled(&self) -> bool { true }

        pub override fn PipelineManifold::request_visual_update(&self) { }
    }
}

#[test]
fn test_diamond_inheritance() {
    assert_eq!(
        format!(
            "{:#?}",
            BindingPipelineManifold::vtable().debug_vtable_layout()
        ),
        "(
    [
        MixinVtableHeader {
            instance: vtable_layout::diamond_inheritance::Object_ChangeNotifier,
            mixin_offset: 24,
            super_offset: 16,
            data_offset: 0,
        },
    ],
    BindingPipelineManifold {
        'start: 0,
        super: Object_ChangeNotifier {
            'start: 0,
            super: Object {
                'start: 0,
                header: ObjectHeader {
                    object_ty: vtable_layout::diamond_inheritance::BindingPipelineManifold,
                },
                'end: 16,
            },
            has_listeners: 16,
            dispose: 24,
            notify_listeners: 32,
            Listenable: Listenable {
                'start: 40,
                super: Object {
                    'start: 40,
                    header: ClassHeader {
                        class_ty: vtable_layout::diamond_inheritance::Listenable,
                        offset: -40,
                    },
                    'end: 56,
                },
                add_listener: 56,
                remove_listener: 64,
                'end: 72,
            },
            'end: 72,
        },
        PipelineManifold: PipelineManifold {
            'start: 72,
            super: Object {
                'start: 72,
                header: ClassHeader {
                    class_ty: vtable_layout::diamond_inheritance::PipelineManifold,
                    offset: -72,
                },
                'end: 88,
            },
            semantics_enabled: 88,
            request_visual_update: 96,
            Listenable: Listenable {
                'start: 104,
                super: Object {
                    'start: 104,
                    header: ClassHeader {
                        class_ty: vtable_layout::diamond_inheritance::Listenable,
                        offset: -104,
                    },
                    'end: 120,
                },
                add_listener: 120,
                remove_listener: 128,
                'end: 136,
            },
            'end: 136,
        },
        'end: 136,
    },
)",
    );
}
