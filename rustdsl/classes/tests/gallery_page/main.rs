//! This test is translated from a Dart demo of its OOP language features,
//! including classes, interfaces, overriding and downcasting.
//!
//! Dart mixin is not supported yet.
//!
//! See [`gallery_page.dart`](./gallery_page.dart)

mod expanded;
mod macros;

static EXPECTED_OUTPUT: &[&str] = &[
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

#[cfg(not(miri))]
#[test]
fn run_dart() {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };

    if Command::new("dart")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_err()
    {
        println!("`dart` not found, skipping test");
        return;
    }
    let output = Command::new("dart")
        .stderr(Stdio::inherit())
        .arg("tests/gallery_page/gallery_page.dart")
        .output()
        .unwrap();
    if !output.status.success() {
        eprintln!("`dart gallery_page.dart` run failed");
        eprintln!("---- dart stdout ----");
        std::io::stderr().write_all(&output.stdout).unwrap();
        eprintln!("---- dart stderr ----");
        std::io::stderr().write_all(&output.stderr).unwrap();
        assert!(output.status.success());
    }
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.lines().collect::<Vec<_>>(), EXPECTED_OUTPUT);
}
