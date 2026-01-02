mod expanded;
mod macros;

static EXPECTED_OUTPUT: &[&str] = &[
    "A::f, x = 0",
    "M::f, z = 2",
    "C1::f, w = 3",
    "M::h, z = 2",
    "C1::h, w = 3",
    "C1::i, w = 3",
    "C1::j, w = 3",
    //
    "A::f, x = 0",
    "M::f, z = 2",
    "C1::f, w = 3",
    "M::h, z = 2",
    "C1::h, w = 3",
    "z = 2",
    //
    "A::f, x = 0",
    "B::f, y = 1",
    "M::f, z = 2",
    "C2::f, v = 4",
    "B::g, y = 1",
    "C2::g, v = 4",
    "M::h, z = 2",
    "C2::h, v = 4",
    "C2::i, v = 4",
    "C2::j, v = 4",
    //
    "A::f, x = 0",
    "B::f, y = 1",
    "M::f, z = 2",
    "C2::f, v = 4",
    "M::h, z = 2",
    "C2::h, v = 4",
    "z = 2",
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
        .arg("tests/mixin/mixin.dart")
        .output()
        .unwrap();
    if !output.status.success() {
        eprintln!("`dart mixin.dart` run failed");
        eprintln!("---- dart stdout ----");
        std::io::stderr().write_all(&output.stdout).unwrap();
        eprintln!("---- dart stderr ----");
        std::io::stderr().write_all(&output.stderr).unwrap();
        assert!(output.status.success());
    }
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.lines().collect::<Vec<_>>(), EXPECTED_OUTPUT);
}
