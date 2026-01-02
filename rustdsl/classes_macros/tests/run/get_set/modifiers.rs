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
                       copy_ty: usize               = 0_usize,
                       clas_ty: CRc<Foo>            = Foo::new(0_usize),
            mutable    copy_ty_mutable: usize       = 0_usize,
            mutable    clone_ty_mutable: String     = "0".to_string(),
            mutable    clas_ty_mutable: CRc<Foo>    = Foo::new(0_usize),
            takecell   copy_ty_takecell: usize,
            takecell   clone_ty_takecell: String,
            takecell   clas_ty_takecell: CRc<Foo>,
            raw        copy_ty_raw: usize           = 0_usize,
            raw        clone_ty_raw: String         = "0".to_string(),
            raw        clas_ty_raw: CRc<Foo>        = Foo::new(0_usize),
            final      copy_ty_final: usize         = 0_usize,
            final      clone_ty_final: String       = "0".to_string(),
            final      clas_ty_final: CRc<Foo>      = Foo::new(0_usize),
            late       copy_ty_late: usize,
            late       clas_ty_late: CRc<Foo>,
            late final copy_ty_late_final: usize,
            late final clone_ty_late_final: String,
            late final clas_ty_late_final: CRc<Foo>,
            late       copy_ty_late_init: usize          = self.f(),
            late       clas_ty_late_init: CRc<Foo>       = Foo::new(self.f()),
            late final copy_ty_late_final_init: usize    = self.f(),
            late final clone_ty_late_final_init: String  = self.f().to_string(),
            late final clas_ty_late_final_init: CRc<Foo> = Foo::new(self.f()),
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
    assert_eq!(m.get_copy_ty(), 1_usize);
    assert_eq!(m.replace_copy_ty(2_usize), 1_usize);
    assert_eq!(m.get_copy_ty(), 2_usize);
}

#[test]
fn test_clas_ty() {
    let m = Modifier::new();
    m.set_clas_ty(Foo::new(1_usize));
    assert_eq!(m.get_clas_ty().get_foo(), 1_usize);
    assert_eq!(m.replace_clas_ty(Foo::new(2_usize)).get_foo(), 1_usize);
    assert_eq!(m.get_clas_ty().get_foo(), 2_usize);
}

#[test]
fn test_copy_ty_mutable() {
    let m = Modifier::new();
    m.set_copy_ty_mutable(1_usize);
    assert_eq!(*m.get_copy_ty_mutable(), 1_usize);
    *m.get_mut_copy_ty_mutable() = 2_usize;
    assert_eq!(*m.get_copy_ty_mutable(), 2_usize);
    assert_eq!(m.replace_copy_ty_mutable(3_usize), 2_usize);
    assert_eq!(*m.get_copy_ty_mutable(), 3_usize);
}

#[test]
fn test_clone_ty_mutable() {
    let m = Modifier::new();
    m.set_clone_ty_mutable("1".to_string());
    assert_eq!(*m.get_clone_ty_mutable(), "1");
    m.get_mut_clone_ty_mutable().push('2');
    assert_eq!(*m.get_clone_ty_mutable(), "12");
    assert_eq!(m.replace_clone_ty_mutable("3".to_string()), "12");
    assert_eq!(*m.get_clone_ty_mutable(), "3");
}

#[test]
fn test_clas_ty_mutable() {
    let m = Modifier::new();
    m.set_clas_ty_mutable(Foo::new(1_usize));
    assert_eq!(m.get_clas_ty_mutable().get_foo(), 1_usize);
    *m.get_mut_clas_ty_mutable() = Foo::new(2_usize);
    assert_eq!(m.get_clas_ty_mutable().get_foo(), 2_usize);
    assert_eq!(
        m.replace_clas_ty_mutable(Foo::new(3_usize)).get_foo(),
        2_usize
    );
    assert_eq!(m.get_clas_ty_mutable().get_foo(), 3_usize);
}

#[test]
fn test_copy_ty_takecell() {
    let m = Modifier::new();
    m.set_copy_ty_takecell(Some(1_usize));
    assert_eq!(m.get_copy_ty_takecell().as_deref(), Some(&1_usize));
    *m.get_mut_copy_ty_takecell().unwrap() = 2_usize;
    assert_eq!(m.get_copy_ty_takecell().as_deref(), Some(&2_usize));
    assert_eq!(m.replace_copy_ty_takecell(Some(3_usize)), Some(2_usize));
    assert_eq!(m.get_copy_ty_takecell().as_deref(), Some(&3_usize));
    assert_eq!(m.take_copy_ty_takecell(), Some(3_usize));
    assert_eq!(m.get_copy_ty_takecell().as_deref(), None);
}

#[test]
fn test_clone_ty_takecell() {
    let m = Modifier::new();
    m.set_clone_ty_takecell(Some("1".to_string()));
    assert_eq!(
        m.get_clone_ty_takecell().as_deref().map(String::as_str),
        Some("1")
    );
    m.get_mut_clone_ty_takecell().unwrap().push('2');
    assert_eq!(
        m.get_clone_ty_takecell().as_deref().map(String::as_str),
        Some("12")
    );
    assert_eq!(
        m.replace_clone_ty_takecell(Some("3".to_string()))
            .as_deref(),
        Some("12")
    );
    assert_eq!(
        m.get_clone_ty_takecell().as_deref().map(String::as_str),
        Some("3")
    );
    assert_eq!(m.take_clone_ty_takecell().as_deref(), Some("3"));
    assert_eq!(
        m.get_clone_ty_takecell().as_deref().map(String::as_str),
        None
    );
}

#[test]
fn test_clas_ty_takecell() {
    let m = Modifier::new();
    m.set_clas_ty_takecell(Some(Foo::new(1_usize)));
    assert_eq!(
        m.get_clas_ty_takecell().as_deref().unwrap().get_foo(),
        1_usize
    );
    *m.get_mut_clas_ty_takecell().unwrap() = Foo::new(2_usize);
    assert_eq!(
        m.get_clas_ty_takecell().as_deref().unwrap().get_foo(),
        2_usize
    );
    assert_eq!(
        m.replace_clas_ty_takecell(Some(Foo::new(3_usize)))
            .unwrap()
            .get_foo(),
        2_usize
    );
    assert_eq!(
        m.get_clas_ty_takecell().as_deref().unwrap().get_foo(),
        3_usize
    );
    assert_eq!(m.take_clas_ty_takecell().unwrap().get_foo(), 3_usize);
    assert_eq!(m.get_clas_ty_takecell().as_deref(), None);
}

#[test]
fn test_copy_ty_raw() {
    let m = Modifier::new();
    assert_eq!(*m.get_copy_ty_raw(), 0_usize);
}

#[test]
fn test_clone_ty_raw() {
    let m = Modifier::new();
    assert_eq!(*m.get_clone_ty_raw(), "0");
}

#[test]
fn test_clas_ty_raw() {
    let m = Modifier::new();
    assert_eq!(m.get_clas_ty_raw().get_foo(), 0_usize);
}

#[test]
fn test_copy_ty_final() {
    let m = Modifier::new();
    assert_eq!(*m.get_copy_ty_final(), 0_usize);
}

#[test]
fn test_clone_ty_final() {
    let m = Modifier::new();
    assert_eq!(*m.get_clone_ty_final(), "0");
}

#[test]
fn test_clas_ty_final() {
    let m = Modifier::new();
    assert_eq!(m.get_clas_ty_final().get_foo(), 0_usize);
}

#[test]
fn test_copy_ty_late() {
    let m = Modifier::new();
    m.set_copy_ty_late(1_usize);
    assert_eq!(m.get_copy_ty_late(), 1_usize);
    assert_eq!(m.replace_copy_ty_late(2_usize), Some(1_usize));
    assert_eq!(m.get_copy_ty_late(), 2_usize);
}

#[test]
fn test_clas_ty_late() {
    let m = Modifier::new();
    m.set_clas_ty_late(Foo::new(1_usize));
    assert_eq!(m.get_clas_ty_late().get_foo(), 1_usize);
    assert_eq!(
        m.replace_clas_ty_late(Foo::new(2_usize)).unwrap().get_foo(),
        1_usize
    );
    assert_eq!(m.get_clas_ty_late().get_foo(), 2_usize);
}

#[test]
fn test_copy_ty_late_final() {
    let m = Modifier::new();
    m.set_clone_ty_late_final("1".to_string());
    assert_eq!(m.get_clone_ty_late_final(), "1");
}

#[test]
fn test_clone_ty_late_final() {
    let m = Modifier::new();
    m.set_clone_ty_late_final("1".to_string());
    assert_eq!(m.get_clone_ty_late_final(), "1");
}

#[test]
fn test_clas_ty_late_final() {
    let m = Modifier::new();
    m.set_clas_ty_late_final(Foo::new(1_usize));
    assert_eq!(m.get_clas_ty_late_final().get_foo(), 1_usize);
}

#[test]
fn test_copy_ty_late_init() {
    let m = Modifier::new();
    assert_eq!(m.get_copy_ty_late_init(), 1_usize);
}

#[test]
fn test_clas_ty_late_init() {
    let m = Modifier::new();
    assert_eq!(m.get_clas_ty_late_init().get_foo(), 1_usize);
}

#[test]
fn test_copy_ty_late_final_init() {
    let m = Modifier::new();
    assert_eq!(m.get_copy_ty_late_final_init(), &1_usize);
}

#[test]
fn test_clone_ty_late_final_init() {
    let m = Modifier::new();
    assert_eq!(m.get_clone_ty_late_final_init(), "1");
}

#[test]
fn test_clas_ty_late_final_init() {
    let m = Modifier::new();
    assert_eq!(m.get_clas_ty_late_final_init().get_foo(), 1_usize);
}

#[test]
#[should_panic(expected = "`late` field not initialized")]
fn copy_ty_late_not_set() {
    let m = Modifier::new();
    m.get_copy_ty_late();
}

#[test]
#[should_panic(expected = "`late` field not initialized")]
fn clas_ty_late_not_set() {
    let m = Modifier::new();
    m.get_clas_ty_late();
}

#[test]
#[should_panic(expected = "`late final` field not initialized")]
fn copy_ty_late_final_not_set() {
    let m = Modifier::new();
    m.get_copy_ty_late_final();
}

#[test]
#[should_panic(expected = "`late final` field not initialized")]
fn clone_ty_late_final_not_set() {
    let m = Modifier::new();
    m.get_clone_ty_late_final();
}

#[test]
#[should_panic(expected = "`late final` field not initialized")]
fn clas_ty_late_final_not_set() {
    let m = Modifier::new();
    m.get_clas_ty_late_final();
}

#[test]
#[should_panic(expected = "`late final` field already initialized")]
fn copy_ty_late_final_double_set() {
    let m = Modifier::new();
    m.set_copy_ty_late_final(1_usize);
    m.set_copy_ty_late_final(2_usize);
}

#[test]
#[should_panic(expected = "`late final` field already initialized")]
fn clone_ty_late_final_double_set() {
    let m = Modifier::new();
    m.set_clone_ty_late_final("1".to_string());
    m.set_clone_ty_late_final("2".to_string());
}

#[test]
#[should_panic(expected = "`late final` field already initialized")]
fn clas_ty_late_final_double_set() {
    let m = Modifier::new();
    m.set_clas_ty_late_final(Foo::new(1_usize));
    m.set_clas_ty_late_final(Foo::new(2_usize));
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

#[test]
#[cfg_attr(debug_assertions, should_panic(expected = "TakeCell already borrowed"))]
fn copy_ty_takecell_get_and_get_mut() {
    let m = Modifier::new();
    m.set_copy_ty_takecell(Some(1_usize));
    let _ref = m.get_copy_ty_takecell();
    let ref_mut = m.get_mut_copy_ty_takecell();
    assert!(ref_mut.is_none());
}

#[test]
#[cfg_attr(debug_assertions, should_panic(expected = "TakeCell already borrowed"))]
fn clone_ty_takecell_get_and_get_mut() {
    let m = Modifier::new();
    m.set_clone_ty_takecell(Some("1".to_string()));
    let _ref = m.get_clone_ty_takecell();
    let ref_mut = m.get_mut_clone_ty_takecell();
    assert!(ref_mut.is_none());
}

#[test]
#[cfg_attr(debug_assertions, should_panic(expected = "TakeCell already borrowed"))]
fn clas_ty_takecell_get_and_get_mut() {
    let m = Modifier::new();
    m.set_clas_ty_takecell(Some(Foo::new(1_usize)));
    let _ref = m.get_clas_ty_takecell();
    let ref_mut = m.get_mut_clas_ty_takecell();
    assert!(ref_mut.is_none());
}

#[test]
#[cfg_attr(debug_assertions, should_panic(expected = "TakeCell already borrowed"))]
fn copy_ty_takecell_double_get_mut() {
    let m = Modifier::new();
    m.set_copy_ty_takecell(Some(1_usize));
    let _ref_mut1 = m.get_mut_copy_ty_takecell();
    let ref_mut2 = m.get_mut_copy_ty_takecell();
    assert!(ref_mut2.is_none());
}

#[test]
#[cfg_attr(debug_assertions, should_panic(expected = "TakeCell already borrowed"))]
fn clone_ty_takecell_double_get_mut() {
    let m = Modifier::new();
    m.set_clone_ty_takecell(Some("1".to_string()));
    let _ref_mut1 = m.get_mut_clone_ty_takecell();
    let ref_mut2 = m.get_mut_clone_ty_takecell();
    assert!(ref_mut2.is_none());
}

#[test]
#[cfg_attr(debug_assertions, should_panic(expected = "TakeCell already borrowed"))]
fn clas_ty_takecell_double_get_mut() {
    let m = Modifier::new();
    m.set_clas_ty_takecell(Some(Foo::new(1_usize)));
    let _ref_mut1 = m.get_mut_clas_ty_takecell();
    let ref_mut2 = m.get_mut_clas_ty_takecell();
    assert!(ref_mut2.is_none());
}

#[test]
#[cfg_attr(debug_assertions, should_panic(expected = "TakeCell already borrowed"))]
fn copy_ty_takecell_double_get() {
    let m = Modifier::new();
    m.set_copy_ty_takecell(Some(1_usize));
    let _ref1 = m.get_copy_ty_takecell();
    let ref2 = m.get_copy_ty_takecell();
    assert!(ref2.is_none());
}

#[test]
#[cfg_attr(debug_assertions, should_panic(expected = "TakeCell already borrowed"))]
fn clone_ty_takecell_double_get() {
    let m = Modifier::new();
    m.set_clone_ty_takecell(Some("1".to_string()));
    let _ref1 = m.get_clone_ty_takecell();
    let ref2 = m.get_clone_ty_takecell();
    assert!(ref2.is_none());
}

#[test]
#[cfg_attr(debug_assertions, should_panic(expected = "TakeCell already borrowed"))]
fn clas_ty_takecell_double_get() {
    let m = Modifier::new();
    m.set_clas_ty_takecell(Some(Foo::new(1_usize)));
    let _ref1 = m.get_clas_ty_takecell();
    let ref2 = m.get_clas_ty_takecell();
    assert!(ref2.is_none());
}
