//! Interface (abstract class) used for rcpta interface conversion (into()) tests.

use classes::*;

classes! {
    /// Identifiable interface: entities that expose an id.
    /// Used to test rcpta interface conversion (x.into() -> CRc<Identifiable>).
    pub abstract class Identifiable {
        pub fn get_id(&self) -> i32;
    }
}
