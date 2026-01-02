// 完整的 OOP 示例 - 从 Dart 翻译而来
// 展示了类、接口、继承、重写和向下转型等特性
use classes_macros::classes;
use classes::prelude::*;

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
            Self {
                parent: None,
                widget,
                dirty: true,
            }
        }
        pub fn rebuild(&self, force: bool) {
            if self.get_dirty() || force {
                self.perform_rebuild();
            }
        }
        pub fn perform_rebuild(&self) {
            self.set_dirty(false);
        }
        pub fn mount(&self, parent: Option<CRc<Element>>) {
            self.set_parent(parent);
        }
        pub fn mark_needs_build(&self) {
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
            Self {
                super: Super::new(widget),
                child: None,
            }
        }

        pub fn first_build(&self) {
            super.rebuild(false);
        }

        pub fn build(&self) -> CRc<Widget>;

        pub override fn perform_rebuild(&self) {
            self.build();
            super.perform_rebuild();
        }

        pub override fn mount(&self, parent: Option<CRc<Element>>) {
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
            Self { 
                super: Super::new(Some(widget.into_super())), 
                state 
            }
        }
        pub fn state(&self) -> CRc<State> {
            self.get_state().unwrap()
        }
        pub override fn build(&self) -> CRc<Widget> {
            self.state().build(&self.upcast::<CRc<Element>, _>())
        }
        pub override fn first_build(&self) {
            self.state().init_state();
            super.first_build();
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
        pub fn init_state(&self) {}
        pub fn build(&self, cx: &CRc<BuildContext>) -> CRc<Widget>;
        pub fn set_state(&self, f: &dyn Fn()) {
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

    class GalleryPage extends StatefulWidget {
        struct {}

        pub fn new() -> Self {
            Self { super: Super::new() }
        }

        pub override fn create_state(&self) -> CRc<GalleryPageState> {
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
        }

        pub override fn build(&self, cx: &CRc<BuildContext>) -> CRc<MyWidget> {
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

fn main() {
    let _page = GalleryPage::new();
    let _element = _page.create_element();
}

