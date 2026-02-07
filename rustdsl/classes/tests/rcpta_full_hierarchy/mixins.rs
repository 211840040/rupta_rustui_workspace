//! Mixin for rcpta cast_mixin tests.

use super::entity::Entity;
use classes::*;

classes! {
    /// Tagged mixin on Entity: adds an optional tag string.
    #[with(Entity)]
    pub mixin Tagged on Entity {
        struct {
            pub mutable tag: Option<String> = None,
        }

        pub fn describe_tagged(&self) -> String {
            format!(
                "Tagged(id={}, tag={:?})",
                self.get_entity_id(),
                self.get_tag()
            )
        }
    }
}
