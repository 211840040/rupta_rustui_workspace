classes_macros::class! {
    pub abstract /* interface */ class Format {
        pub fn new() -> Self {
            Self {}
        }

        pub fn fmt_debug(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            core::fmt::Display::fmt(&CRc::<Self>::as_ptr(self), f)
        }
    }
}
