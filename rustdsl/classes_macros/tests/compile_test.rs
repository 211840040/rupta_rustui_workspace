#[cfg(not(miri))]
#[test]
fn test() {
    let t = trybuild::TestCases::new();
    // `compile`
    t.compile_fail("tests/compile/**/*.rs");
    // `fixme`
    t.compile_fail("tests/fixme/correctness/*.rs");
    t.compile_fail("tests/fixme/grammar/*.rs");
    t.pass("tests/fixme/run/*.rs");
    // `fixed`
    t.compile_fail("tests/fixed/compile/**/*.rs");
}

#[cfg(miri)]
#[test]
fn test() {}
