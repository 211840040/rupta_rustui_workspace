//! SpecialItem: another subclass of Item for rcpta method-call (polymorphic dispatch) tests.
//! Overrides Entity::describe so that a single call site on CRc<Entity> can resolve to three callees:
//! Item::describe, KeyedItem::describe, SpecialItem::describe.

use classes::*;
use super::item::Item;
use crate::Entity;

classes! {
    /// Concrete Item with a tag (i32); used to exercise polymorphic resolve at one call site.
    pub class SpecialItem extends Item {
        struct {
            pub tag: i32,
        }

        pub fn new(id: i32, tag: i32) -> Self {
            Self {
                super: Super::new(id),
                tag,
            }
        }

        pub override fn Entity::describe(&self) -> String {
            format!("SpecialItem(id={}, tag={})", self.get_id(), self.get_tag())
        }
    }
}
