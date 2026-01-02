use crate::test_utils::{BUF, printlntb};
use ::classes::classes;
use ::classes::prelude::*;

classes! {
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

        pub fn has_listeners(&self) -> bool {
            printlntb!("ChangeNotifier::has_listeners, {} listeners", self.get_listeners().len());
            !self.get_listeners().is_empty()
        }

        pub fn dispose(&self) {
            printlntb!("ChangeNotifier::dispose");
            self.get_mut_listeners().clear();
        }

        pub override fn Listenable::add_listener(&self, f: fn()) {
            printlntb!("ChangeNotifier::add_listener");
            self.get_mut_listeners().push(f);
        }
        pub override fn Listenable::remove_listener(&self, f: fn()) {
            printlntb!("ChangeNotifier::remove_listener");
            self.get_mut_listeners().retain(|&l| !core::ptr::fn_addr_eq(l, f));
        }

        pub fn notify_listeners(&self) {
            printlntb!("ChangeNotifier::notify_listeners");
            if self.has_listeners() {
                for listener in core::mem::take(&mut *self.get_mut_listeners()) {
                    listener();
                }
            }
        }
    }

    class BindingPipelineManifold extends Object with ChangeNotifier implements PipelineManifold {
        pub fn new() -> Self { Self { super: Super::new(), .. } }

        pub override fn PipelineManifold::semantics_enabled(&self) -> bool {
            printlntb!("BindingPipelineManifold::semantics_enabled");
            true
        }

        pub override fn PipelineManifold::request_visual_update(&self) {
            if self.semantics_enabled() {
                printlntb!("BindingPipelineManifold::request_visual_update");
                self.notify_listeners();
            }
        }
    }
}

static EXPECTED: &[&str] = &[
    // manifold.add_listener(listener1);
    "ChangeNotifier::add_listener",
    // manifold.add_listener(listener2);
    "ChangeNotifier::add_listener",
    // manifold.add_listener(listener3);
    "ChangeNotifier::add_listener",
    // manifold.remove_listener(listener2);
    "ChangeNotifier::remove_listener",
    // manifold.request_visual_update();
    "BindingPipelineManifold::semantics_enabled",
    "BindingPipelineManifold::request_visual_update",
    "ChangeNotifier::notify_listeners",
    #[cfg(miri)]
    "ChangeNotifier::has_listeners, 3 listeners",
    #[cfg(not(miri))]
    "ChangeNotifier::has_listeners, 2 listeners",
    "listener1",
    #[cfg(miri)]
    "listener2",
    "listener3",
];

#[test]
fn diamond_inheritance() {
    let manifold: CRc<PipelineManifold> = BindingPipelineManifold::new().into_supertype();
    macro_rules! listener {
        ($name:ident) => {
            fn $name() {
                printlntb!(stringify!($name));
            }
        };
    }
    listener!(listener1);
    listener!(listener2);
    listener!(listener3);
    manifold.to_impl().add_listener(listener1);
    manifold.to_impl().add_listener(listener2);
    manifold.to_impl().add_listener(listener3);
    manifold.to_impl().remove_listener(listener2);
    manifold.request_visual_update();
    assert_eq!(BUF.take(), EXPECTED);
}
