use crate::test_utils::{BUF, printlntb};
use classes_macros::classes;

classes! {
    class Foo {
        struct {
            foo: usize,
        }

        pub fn new(foo: usize) -> Self {
            Self { foo }
        }
    }

    class Modifier {
        struct {
                       copy_ty: Option<usize>              = Some(0_usize),
                       clas_ty: Option<CRc<Foo>>           = Some(Foo::new(0_usize)),
            mutable    copy_ty_mutable: Option<usize>      = Some(0_usize),
            mutable    clone_ty_mutable: Option<String>    = Some("0".to_string()),
            mutable    clas_ty_mutable: Option<CRc<Foo>>   = Some(Foo::new(0_usize)),
            raw        copy_ty_raw: Option<usize>          = Some(0_usize),
            raw        clone_ty_raw: Option<String>        = Some("0".to_string()),
            raw        clas_ty_raw: Option<CRc<Foo>>       = Some(Foo::new(0_usize)),
            final      copy_ty_final: Option<usize>        = Some(0_usize),
            final      clone_ty_final: Option<String>     = Some("0".to_string()),
            final      clas_ty_final: Option<CRc<Foo>>     = Some(Foo::new(0_usize)),
            late       copy_ty_late: Option<usize>,
            // late       clas_ty_late: Option<CRc<Foo>>,
            late final copy_ty_late_final: Option<usize>,
            late final clone_ty_late_final: Option<String>,
            late final clas_ty_late_final: Option<CRc<Foo>>,
            late final copy_ty_late_final_init: Option<usize> = Some(self.f()),
            late final clone_ty_late_final_init: Option<String> = Some(self.f().to_string()),
            late final clas_ty_late_final_init: Option<CRc<Foo>> = Some(Foo::new(self.f())),
        }

        pub fn new() -> Self {
            printlntb!("Modifier::constructor");
            Self { .. }
        }

        fn f(&self) -> usize {
            1_usize
        }
    }
}

#[test]
fn test_copy_ty() {
    let m = Modifier::new();
    m.set_copy_ty(1_usize);
    assert_eq!(m.get_copy_ty(), Some(1_usize));
    assert_eq!(m.replace_copy_ty(Some(2_usize)), Some(1_usize));
    assert_eq!(m.get_copy_ty(), Some(2_usize));
}

#[test]
fn test_clas_ty() {
    let m = Modifier::new();
    m.set_clas_ty(Foo::new(1_usize));
    assert_eq!(m.get_clas_ty().unwrap().get_foo(), 1_usize);
    assert_eq!(
        m.replace_clas_ty(Foo::new(2_usize)).unwrap().get_foo(),
        1_usize
    );
    assert_eq!(m.get_clas_ty().unwrap().get_foo(), 2_usize);
}

#[test]
fn test_copy_ty_mutable() {
    let m = Modifier::new();
    m.set_copy_ty_mutable(Some(1_usize));
    assert_eq!(*m.get_copy_ty_mutable(), Some(1_usize));
    *m.get_mut_copy_ty_mutable() = Some(2_usize);
    assert_eq!(*m.get_copy_ty_mutable(), Some(2_usize));
    assert_eq!(m.replace_copy_ty_mutable(Some(3_usize)), Some(2_usize));
    assert_eq!(*m.get_copy_ty_mutable(), Some(3_usize));
}

#[test]
fn test_clone_ty_mutable() {
    let m = Modifier::new();
    m.set_clone_ty_mutable(Some("1".to_string()));
    assert_eq!(*m.get_clone_ty_mutable(), Some("1".to_string()));
    m.get_mut_clone_ty_mutable().as_mut().unwrap().push('2');
    assert_eq!(m.get_clone_ty_mutable().as_deref(), Some("12"));
    assert_eq!(
        m.replace_clone_ty_mutable(Some("3".to_string())).as_deref(),
        Some("12")
    );
    assert_eq!(m.get_clone_ty_mutable().as_deref(), Some("3"));
}

#[test]
fn test_clas_ty_mutable() {
    let m = Modifier::new();
    m.set_clas_ty_mutable(Some(Foo::new(1_usize)));
    assert_eq!(m.get_clas_ty_mutable().clone().unwrap().get_foo(), 1_usize);
    *m.get_mut_clas_ty_mutable() = Some(Foo::new(2_usize));
    assert_eq!(m.get_clas_ty_mutable().clone().unwrap().get_foo(), 2_usize);
    assert_eq!(
        m.replace_clas_ty_mutable(Some(Foo::new(3_usize)))
            .unwrap()
            .get_foo(),
        2_usize
    );
    assert_eq!(m.get_clas_ty_mutable().clone().unwrap().get_foo(), 3_usize);
}

#[test]
fn test_copy_ty_raw() {
    let m = Modifier::new();
    assert_eq!(*m.get_copy_ty_raw(), Some(0_usize));
}

#[test]
fn test_clone_ty_raw() {
    let m = Modifier::new();
    assert_eq!(*m.get_clone_ty_raw(), Some("0".to_string()));
}

#[test]
fn test_clas_ty_raw() {
    let m = Modifier::new();
    assert_eq!(m.get_clas_ty_raw().clone().unwrap().get_foo(), 0_usize);
}

#[test]
fn test_copy_ty_final() {
    let m = Modifier::new();
    assert_eq!(*m.get_copy_ty_final(), Some(0_usize));
}

#[test]
fn test_clone_ty_final() {
    let m = Modifier::new();
    assert_eq!(*m.get_clone_ty_final(), Some("0".to_string()));
}

#[test]
fn test_clas_ty_final() {
    let m = Modifier::new();
    assert_eq!(m.get_clas_ty_final().clone().unwrap().get_foo(), 0_usize);
}

#[test]
fn test_copy_ty_late() {
    let m = Modifier::new();
    m.set_copy_ty_late(Some(1_usize));
    assert_eq!(m.get_copy_ty_late(), Some(1_usize));
    let _ = m.replace_copy_ty_late(Some(2_usize));
    assert_eq!(m.get_copy_ty_late(), Some(2_usize));
}

// #[test]
// fn test_clas_ty_late() {
//     let m = Modifier::new();
//     m.set_clas_ty_late(Foo::new(1_usize));
//     assert_eq!(m.get_clas_ty_late().clone().unwrap().get_foo(), 1_usize);
//     let _ = m.replace_clas_ty_late(Foo::new(2_usize));
//     assert_eq!(m.get_clas_ty_late().clone().unwrap().get_foo(), 2_usize);
// }

#[test]
fn test_copy_ty_late_final() {
    let m = Modifier::new();
    m.set_copy_ty_late_final(Some(1_usize));
    assert_eq!(m.get_copy_ty_late_final(), &Some(1_usize));
}

#[test]
fn test_clone_ty_late_final() {
    let m = Modifier::new();
    m.set_clone_ty_late_final(Some("1".to_string()));
    assert_eq!(m.get_clone_ty_late_final().as_deref(), Some("1"));
}

#[test]
fn test_clas_ty_late_final() {
    let m = Modifier::new();
    m.set_clas_ty_late_final(Some(Foo::new(1_usize)));
    assert_eq!(
        m.get_clas_ty_late_final().clone().unwrap().get_foo(),
        1_usize
    );
}

#[test]
#[should_panic(expected = "`late` field not initialized")]
fn copy_ty_late_not_set() {
    let m = Modifier::new();
    m.get_copy_ty_late();
}

// #[test]
// #[should_panic(expected = "`late` field not initialized")]
// fn clas_ty_late_not_set() {
//     let m = Modifier::new();
//     m.get_clas_ty_late();
// }

#[test]
#[should_panic(expected = "`late final` field not initialized")]
fn copy_ty_late_final_not_set() {
    let m = Modifier::new();
    m.get_copy_ty_late_final();
}

#[test]
fn clas_ty_late_final_not_set() {
    let m = Modifier::new();
    assert!(m.get_clas_ty_late_final().is_none());
}

#[test]
#[should_panic(expected = "`late final` field already initialized")]
fn copy_ty_late_final_double_set() {
    let m = Modifier::new();
    m.set_copy_ty_late_final(Some(1_usize));
    m.set_copy_ty_late_final(Some(2_usize));
}

#[test]
#[should_panic(expected = "`late final` field already initialized")]
fn clone_ty_late_final_double_set() {
    let m = Modifier::new();
    m.set_clone_ty_late_final(Some("1".to_string()));
    m.set_clone_ty_late_final(Some("2".to_string()));
}

#[test]
#[should_panic(expected = "`late final` field already initialized")]
fn clas_ty_late_final_double_set() {
    let m = Modifier::new();
    m.set_clas_ty_late_final(Some(Foo::new(1_usize)));
    m.set_clas_ty_late_final(Some(Foo::new(2_usize)));
}

#[test]
#[should_panic(expected = "already borrowed: BorrowMutError")]
fn copy_ty_mutable_get_and_get_mut() {
    let m = Modifier::new();
    let _ref = m.get_copy_ty_mutable();
    let _ref_mut = m.get_mut_copy_ty_mutable();
}

#[test]
#[should_panic(expected = "already borrowed: BorrowMutError")]
fn clone_ty_mutable_get_and_get_mut() {
    let m = Modifier::new();
    let _ref = m.get_clone_ty_mutable();
    let _ref_mut = m.get_mut_clone_ty_mutable();
}

#[test]
#[should_panic(expected = "already borrowed: BorrowMutError")]
fn clas_ty_mutable_get_and_get_mut() {
    let m = Modifier::new();
    let _ref = m.get_clas_ty_mutable();
    let _ref_mut = m.get_mut_clas_ty_mutable();
}

#[test]
#[should_panic(expected = "already borrowed: BorrowMutError")]
fn copy_ty_mutable_double_get_mut() {
    let m = Modifier::new();
    let _ref_mut1 = m.get_mut_copy_ty_mutable();
    let _ref_mut2 = m.get_mut_copy_ty_mutable();
}

#[test]
#[should_panic(expected = "already borrowed: BorrowMutError")]
fn clone_ty_mutable_double_get_mut() {
    let m = Modifier::new();
    let _ref_mut1 = m.get_mut_clone_ty_mutable();
    let _ref_mut2 = m.get_mut_clone_ty_mutable();
}

#[test]
#[should_panic(expected = "already borrowed: BorrowMutError")]
fn clas_ty_mutable_double_get_mut() {
    let m = Modifier::new();
    let _ref_mut1 = m.get_mut_clas_ty_mutable();
    let _ref_mut2 = m.get_mut_clas_ty_mutable();
}
