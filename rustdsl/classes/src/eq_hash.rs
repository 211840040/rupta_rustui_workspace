classes_macros::class! {
    pub abstract /* interface */ class EqHash {
        pub fn new() -> Self { Self {} }
        pub fn eq(&self, other: &CRc<Object>) -> bool {
            super.eq(other)
        }
        pub fn hash(&self, state: &mut (dyn core::hash::Hasher + '_)) {
            super.hash(state);
        }
    }
}
