// Holder: class with a class reference field (CRc<Item>).
// get_item / set_item produce Load/Store semantics for rcpta verification.

use classes::*;
use super::item::Item;
use super::entity::Entity;

classes! {
    /// Holder has a late field of type CRc<Item>; get/set produce Load/Store edges.
    pub class Holder {
        struct {
            /// Class reference field: Store (set_item) and Load (get_item) build PAG edges.
            pub late item: CRc<Item>,
        }

        pub fn new() -> Self {
            Self { .. }
        }

        /// Load + Cast inside callee: get_item then into_superclass. Builds Load and Cast edges in callee body.
        pub fn get_and_wrap(&self) -> CRc<Entity> {
            self.get_item().into_superclass()
        }
    }
}
