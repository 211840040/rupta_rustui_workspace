use classes_macros::classes;
use core::cell::RefCell;

thread_local! {
    static BUF: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

macro_rules! println {
    ($($args:tt)*) => {
        BUF.with_borrow_mut(|buf| {
            buf.push(format!($($args)*));
        })
    };
}

classes! {
    abstract class BuildContext {
        pub fn widget(&self) -> CRc<Widget>;
    }

    abstract class Element implements BuildContext {
        struct {
            parent: Option<CWeak<Element>>,
            widget: Option<CWeak<Widget>>,
            dirty: bool,
        }
        pub fn new(widget: Option<CRc<Widget>>) -> Self {
            println!("Element");
            Self {
                parent: None,
                widget,
                dirty: true,
            }
        }
        pub fn rebuild(&self, force: bool) {
            println!("Element::rebuild");
            if self.get_dirty() || force {
                self.perform_rebuild();
            }
        }
        pub fn perform_rebuild(&self) {
            println!("Element::perform_rebuild");
            self.set_dirty(false);
        }
        pub fn mount(&self, parent: Option<CRc<Element>>) {
            println!("Element::mount");
            self.set_parent(parent);
        }
        pub fn mark_needs_build(&self) {
            println!("Element::mark_needs_build");
            self.set_dirty(true);
            self.rebuild(false);
        }
        pub override fn <Self as BuildContext>::widget(&self) -> CRc<Widget> {
            self.get_widget().unwrap()
        }
    }

    abstract class ComponentElement extends Element {
        struct {
            child: Option<CRc<Element>>,
        }
        pub fn new(widget: Option<CRc<Widget>>) -> Self {
            let self = Self {
                super: Super::new(widget),
                child: None,
            };
            println!("ComponentElement");
            self
        }

        pub fn first_build(&self) {
            println!("ComponentElement::first_build");
            super.rebuild(false);
        }

        pub fn build(&self) -> CRc<Widget>;

        pub override fn perform_rebuild(&self) {
            println!("ComponentElement::perform_rebuild");
            self.build();
            super.perform_rebuild();
        }

        pub override fn mount(&self, parent: Option<CRc<Element>>) {
            println!("ComponentElement::mount");
            super.mount(parent);
            self.first_build();
        }
    }

    class StatefulElement extends ComponentElement {
        struct {
            state: Option<CRc<State>>,
        }
        pub fn new(widget: CRc<StatefulWidget>) -> Self {
            let state = Some(widget.create_state());
            let self = Self { super: Super::new(Some(widget.into_super())), state };
            println!("StatefulElement");
            self
        }
        pub fn state(&self) -> CRc<State> {
            self.get_state().unwrap()
        }
        pub override fn build(&self) -> CRc<Widget> {
            self.state().build(&self.upcast::<CRc<Element>, _>())
        }
        pub override fn first_build(&self) {
            println!("StatefulElement::first_build");
            self.state().init_state();
            super.first_build();
        }
        pub override fn Element::perform_rebuild(&self) {
            println!("StatefulElement::perform_rebuild");
            super.perform_rebuild();
        }
    }

    abstract class State {
        struct {
            widget: Option<CRc<Widget>> = None,
            element: Option<CRc<Element>> = None,
        }
        pub fn new() -> Self {
            Self { .. }
        }
        pub fn init_state(&self) {
            println!("State::init_state");
        }
        pub fn build(&self, cx: &CRc<BuildContext>) -> CRc<Widget>;
        pub fn set_state(&self, f: &dyn Fn()) {
            println!("State::set_state");
            f();
            self.get_element().unwrap().mark_needs_build();
        }
    }

    abstract class Widget {
        struct {}
        pub fn new() -> Self {
            Self { }
       }
        pub fn create_element(&self) -> CRc<Element>;
    }

    abstract class StatefulWidget extends Widget {
        struct {}
        pub fn new() -> Self {
            Self { super: Super::new() }
        }
        pub override fn create_element(&self) -> CRc<Element> {
            StatefulElement::new(self.clone()).into_superclass()
        }

        pub fn create_state(&self) -> CRc<State>;
    }
}

#[test]
fn gallery_page() {
    classes! {
        class GalleryPage extends StatefulWidget {
            struct {}

            pub fn new() -> Self {
                Self { super: Super::new() }
            }

            pub override fn create_state(&self) -> CRc<GalleryPageState> {
                println!("GalleryPage::create_state");
                GalleryPageState::new()
            }

            pub fn on_create(&self) {
                println!("GalleryPage::on_create");
            }
        }

        class GalleryPageState extends State {
            struct {}

            pub fn new() -> Self {
                Self { super: Super::new() }
            }

            pub override fn init_state(&self) {
                super.init_state();
                println!("GalleryPageState::init_state");
            }

            pub override fn build(&self, cx: &CRc<BuildContext>) -> CRc<MyWidget> {
                println!("GalleryPageState::build");
                cx.widget().as_subclass::<CRc<GalleryPage>>().on_create();
                MyWidget::new()
            }
        }

        class MyWidget extends Widget {
            struct {}

            pub fn new() -> Self {
                Self { super: Super::new() }
            }

            pub override fn create_element(&self) -> CRc<MyElement> {
                MyElement::new(self.as_super().clone())
            }
        }

        class MyElement extends Element {
            struct {}
            pub fn new(widget: CRc<Widget>) -> Self {
                Self { super: Super::new(Some(widget)) }
            }
        }
    }

    BUF.take();
    GalleryPage::new().create_element().mount(None);
    assert_eq!(BUF.take(), super::EXPECTED_OUTPUT);
}
