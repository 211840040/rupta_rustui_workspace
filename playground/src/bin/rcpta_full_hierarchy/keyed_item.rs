//! KeyedItem: subclass of Item for rcpta downcast/upcast tests.

use classes::*;
use super::item::Item;
use crate::Entity;

classes! {
    /// Item with a numeric key; subclass of Item for try_into_subtype / into_superclass.
    pub class KeyedItem extends Item {
        struct {
            pub key: i32,
        }

        pub fn new(id: i32, key: i32) -> Self {
            Self {
                super: Super::new(id),
                key,
            }
        }

        pub override fn Entity::describe(&self) -> String {
            format!("KeyedItem(id={}, key={})", self.get_id(), self.get_key())
        }
    }
}
