//! Entity: abstract base class for the hierarchy.
//! Used by rcpta to test Alloc, into_superclass, try_into_subtype, and interface conversion.

use classes::*;
use super::interfaces::Identifiable;
use super::holder::Holder;
use super::item::Item;

classes! {
    /// Abstract base: has id, implements Identifiable for interface (into()) tests.
    pub abstract class Entity implements Identifiable {
        struct {
            pub entity_id: i32,
        }

        pub fn new(id: i32) -> Self {
            Self { entity_id: id }
        }

        pub fn describe(&self) -> String {
            format!("Entity(id={})", self.get_entity_id())
        }

        pub override fn Identifiable::get_id(&self) -> i32 {
            self.get_entity_id()
        }

        /// Returns the given partner as CRc<Entity>. Used by rcpta to verify call-arg and call-ret edges.
        pub fn with_partner(&self, other: CRc<Entity>) -> CRc<Entity> {
            other
        }

        /// Callee calls another method: other.with_partner(other). Builds call graph edge and call-arg/call-ret inside this callee.
        pub fn chain_with(&self, other: CRc<Entity>) -> CRc<Entity> {
            other.clone().with_partner(other)
        }

        /// Two pointer args, returns a.with_partner(b). Builds more formals and internal call to with_partner.
        pub fn apply_twice(&self, a: CRc<Entity>, b: CRc<Entity>) -> CRc<Entity> {
            a.with_partner(b)
        }

        /// Load inside callee: h.get_item(). Builds Load edge in callee and call-arg (h) / call-ret.
        pub fn process_holder(&self, h: CRc<Holder>) -> CRc<Item> {
            h.get_item()
        }
    }
}
