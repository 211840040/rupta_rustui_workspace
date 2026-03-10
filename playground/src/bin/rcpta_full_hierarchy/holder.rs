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

        /// Load + Cast + internal Call inside callee: get_item, into_superclass, then get_id on Entity.
        /// Builds Load, Cast, and Call edges in callee body for rcpta.
        pub fn get_and_wrap(&self) -> CRc<Entity> {
            let item = self.get_item();                    // Load: self.item -> item
            let entity: CRc<Entity> = item.into_superclass(); // Cast: Item -> Entity
            let _ = entity.get_id();             // Call: entity.get_id() (adds call edge in callee)
            entity
        }
    }
}
