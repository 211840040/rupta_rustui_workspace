macro_rules! printlntb{
    ($($args:tt)*)  => {
        BUF.with_borrow_mut(|buf| {
            buf.push(format!($($args)*));
        })
    };
}

thread_local! {
    pub static BUF: std::cell::RefCell<Vec<String>> = std::cell::RefCell::new(Vec::new());
}

pub(crate) use printlntb;
