//! Item: concrete class extending Entity with Tagged mixin.
//! Used as the type of class reference fields (Holder) and for rcpta Load/Store, cast_mixin, and inheritance tests.

use classes::*;
use super::entity::Entity;
use super::mixins::Tagged;

classes! {
    /// Concrete entity with an id; held by Holder via CRc<Item>.
    /// Extends Entity, mixes in Tagged for cast_mixin tests.
    pub class Item extends Entity with Tagged {
        struct {}

        pub fn new(id: i32) -> Self {
            Self {
                super: Super::new(id),
                ..
            }
        }

        pub override fn Entity::describe(&self) -> String {
            format!("Item(id={})", self.get_id())
        }
    }
}
