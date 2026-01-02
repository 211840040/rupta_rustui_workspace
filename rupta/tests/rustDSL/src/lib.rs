#![feature(panic_internals)]
#![feature(derive_clone_copy)]
#![feature(const_trait_impl)]
#![feature(const_fn_trait_bound)]
#![feature(builtin_syntax)]

pub static EXPECTED_OUTPUT: &[&str] = &[
    "GalleryPage::create_state",
    "Element",
    "ComponentElement",
    "StatefulElement",
    "ComponentElement::mount",
    "Element::mount",
    "StatefulElement::first_build",
    "State::init_state",
    "GalleryPageState::init_state",
    "ComponentElement::first_build",
    "Element::rebuild",
    "StatefulElement::perform_rebuild",
    "ComponentElement::perform_rebuild",
    "GalleryPageState::build",
    "GalleryPage::on_create",
    "Element::perform_rebuild",
];

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/expanded.rs"));

pub fn test_entry() {
    use _classes::*;
    use classes::prelude::*;

    BUF.with_borrow_mut(|buf| buf.clear());

    let page = GalleryPage::new();
    let element = page.create_element();
    element.mount(None);
    element.mark_needs_build();
    element.rebuild(true);

    let gallery_state = GalleryPageState::new();
    gallery_state.init_state();
    let build_context = element.to_supertype::<CRc<BuildContext>>();
    let built_widget = gallery_state.build(&build_context);
    let child_element = built_widget.create_element();
    child_element.mount(Some(element.clone()));
    child_element.mark_needs_build();

    let my_widget = MyWidget::new();
    let my_element = my_widget.create_element();
    my_element.mount(Some(child_element.clone()));
    my_element.rebuild(false);
}