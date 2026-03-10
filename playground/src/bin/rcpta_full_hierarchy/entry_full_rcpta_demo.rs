//! rcpta_full_hierarchy: hierarchy for verifying rcpta (Load/Store and all supported Rust class syntax).
//!
//! - **Entity** (abstract), **Item** (extends Entity + Tagged), **KeyedItem** (extends Item): inheritance + mixin.
//! - **Holder**: class with `late item: CRc<Item>`; `get_item`/`set_item` give Load/Store semantics.
//! - **Identifiable** interface, **Tagged** mixin: for into() and cast_mixin tests.
//!
//! Entry functions:
//! - `entry_load_store_demo()`: Load/Store only (minimal).
//! - `entry_full_rcpta_demo()`: Alloc, Assign, Clone, upcast, downcast, Option::unwrap, cast_mixin, into(), Load/Store, method calls.
//! - `entry_method_call_demo(choice)`: polymorphic method call; one call site `e.describe()` with pts(e) = { Item, KeyedItem, SpecialItem } → 3 callees.
//! - `entry_method_call_args_ret_demo(choice)`: method call with pointer arg and return (Entity::with_partner) → verifies call-arg and call-ret edges.
//! - `entry_complex_call_chain_demo()`: deep call chain; callees with Load/Cast/internal calls (get_and_wrap, chain_with, apply_twice, process_holder).

mod entity;
mod holder;
mod interfaces;
mod item;
mod keyed_item;
mod mixins;
mod special_item;

pub use entity::Entity;
pub use holder::Holder;
pub use interfaces::Identifiable;
pub use item::Item;
pub use keyed_item::KeyedItem;
pub use mixins::Tagged;
pub use special_item::SpecialItem;

/// Entry function for rcpta: exercises all supported Rust class syntax in one place.
///
/// Covers: Alloc, Assign, Clone, upcast (into_superclass), downcast (try_into_subtype + unwrap),
/// cast_mixin (Tagged), interface conversion (into() -> CRc<Identifiable>), Load/Store, method calls.
pub fn main() {
    use classes::prelude::*;

    // --- Alloc ---
    let item_a = Item::new(10);
    let item_b = Item::new(20);
    let keyed = KeyedItem::new(1, 100);
    let holder = Holder::new();

    // --- Assign (let x = y), Clone ---
    let item_c = item_a.clone();
    let _ = item_c.get_id();

    // --- Upcast: Item -> Entity, KeyedItem -> Item -> Entity ---
    let entity_a: CRc<Entity> = item_a.clone().into_superclass();
    let _ = entity_a.describe();
    let item_view: CRc<Item> = keyed.clone().into_superclass();
    let _ = item_view.get_id();

    // --- Downcast + Option::unwrap ---
    let back_to_item = entity_a.clone().try_into_subtype::<CRc<Item>>();
    let item_again = back_to_item.unwrap();
    let _ = item_again.describe();

    // --- Cast to mixin (Item has Tagged) ---
    let tagged: CRc<Tagged> = item_b.clone().cast_mixin();
    let _ = tagged.describe_tagged();

    // --- Interface conversion (Entity implements Identifiable; into CRc<Identifiable>) ---
    let as_entity: CRc<Entity> = item_b.clone().into_superclass();
    let identifiable: CRc<Identifiable> = as_entity.into();
    let _ = identifiable.get_id();

    // --- Load / Store ---
    holder.set_item(item_b);
    let loaded = holder.get_item();
    let _ = loaded.get_id();

    std::mem::drop(holder);
}
