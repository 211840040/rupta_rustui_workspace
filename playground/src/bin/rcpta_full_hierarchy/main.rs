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

/// Entry function for rcpta: contains load/store semantic operations.
///
/// Expected ClassPAG edges (conceptually):
/// - Store: _a1 → (holder_1, "item"), _b1 → (holder_2, "item")
/// - Load:  (holder_1, "item") → _x, (holder_2, "item") → _y
pub fn entry_load_store_demo() {
    let _a1 = Item::new(1);
    let _b1 = Item::new(2);

    let holder_1 = Holder::new();
    let holder_2 = Holder::new();

    // STORE: value → (base, field)
    holder_1.set_item(_a1);
    holder_2.set_item(_b1);

    // LOAD: (base, field) → result
    let _x = holder_1.get_item();
    let _y = holder_2.get_item();

    std::mem::drop(_x);
    std::mem::drop(_y);
    std::mem::drop(holder_1);
    std::mem::drop(holder_2);
}

/// Entry function for rcpta: exercises all supported Rust class syntax in one place.
///
/// Covers: Alloc, Assign, Clone, upcast (into_superclass), downcast (try_into_subtype + unwrap),
/// cast_mixin (Tagged), interface conversion (into() -> CRc<Identifiable>), Load/Store, method calls.
pub fn entry_full_rcpta_demo() {
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

/// Entry function for rcpta: exercises **method call** (polymorphic dispatch) analysis.
///
/// One variable `e: CRc<Entity>` may point to an Item, KeyedItem, or SpecialItem depending on `choice`.
/// - `e.describe()`: one call site, **three possible callees** (Item::describe, KeyedItem::describe, SpecialItem::describe).
/// - `e.get_id()`: one call site, single callee (Entity's Identifiable::get_id).
/// Optionally uses Holder Load so the receiver of a method call can come from a Load edge.
///
/// Intended for verifying: pts(receiver) → dispatch(concrete_type, method_name) → CallArg/CallRet per callee.
pub fn entry_method_call_demo(choice: u8) {
    use classes::prelude::*;

    let item = Item::new(1);
    let keyed = KeyedItem::new(2, 20);
    let special = SpecialItem::new(3, 300);

    let e: CRc<Entity> = match choice {
        0 => item.into_superclass(),
        1 => keyed.clone().into_superclass(),
        _ => special.clone().into_superclass(),
    };

    let _ = e.describe();
    let _ = e.get_id();

    std::mem::drop(e);
}

/// Entry function for rcpta: method calls with **pointer arguments and return value**.
///
/// Uses Entity::with_partner(self, other: CRc<Entity>) -> CRc<Entity> so that:
/// - Call-arg: receiver and argument (partner) flow to callee params.
/// - Call-ret: callee return flows to result (out).
/// Then calls describe() on the result to also exercise call graph + polymorphic dispatch on the returned pointer.
pub fn entry_method_call_args_ret_demo(choice: u8) {
    use classes::prelude::*;

    let item = Item::new(1);
    let keyed = KeyedItem::new(2, 20);
    let special = SpecialItem::new(3, 300);

    let e: CRc<Entity> = match choice {
        0 => item.into_superclass(),
        1 => keyed.clone().into_superclass(),
        _ => special.clone().into_superclass(),
    };
    let partner: CRc<Entity> = keyed.clone().into_superclass();

    // Method call with pointer arg and pointer return → call-arg edges (e, partner → callee), call-ret (return → out)
    let out: CRc<Entity> = e.with_partner(partner);
    let _ = out.describe();
    let _ = out.get_id();

    std::mem::drop(out);
}

/// Like entry_method_call_demo but receiver can also come from Holder (Load).
/// Tests method call on a value loaded from a class reference field.
pub fn entry_method_call_via_load_demo(choice: u8) {
    use classes::prelude::*;

    let item = Item::new(10);
    let keyed = KeyedItem::new(11, 21);
    let special = SpecialItem::new(12, 120);

    let holder = Holder::new();
    match choice {
        0 => holder.set_item(item),
        1 => holder.set_item(keyed.clone().into_superclass::<CRc<Item>>()),
        _ => holder.set_item(special.clone().into_superclass::<CRc<Item>>()),
    }

    let loaded: CRc<Item> = holder.get_item();
    let as_entity: CRc<Entity> = loaded.clone().into_superclass();
    let _ = as_entity.describe();
    let _ = as_entity.get_id();

    std::mem::drop(holder);
}

/// Entry function for rcpta: **complex call chain** and callees with non-trivial PAG edges.
///
/// From entry we call:
/// - Holder::get_and_wrap (×2): callee does Load + Cast.
/// - Entity::chain_with: callee calls with_partner (call graph + call-arg/call-ret inside callee).
/// - Entity::apply_twice: two pointer params, callee calls with_partner.
/// - Entity::process_holder: callee does Load (h.get_item()).
/// - describe, get_id on results.
/// Intended to verify PAG edge build and call graph edge build across multiple functions with internal edges.
pub fn entry_complex_call_chain_demo() {
    use classes::prelude::*;

    let item_a = Item::new(10);
    let keyed = KeyedItem::new(1, 100);

    let holder_1 = Holder::new();
    let holder_2 = Holder::new();
    holder_1.set_item(item_a);
    holder_2.set_item(keyed.clone().into_superclass::<CRc<Item>>());

    // get_and_wrap: Load + Cast inside Holder
    let e1: CRc<Entity> = holder_1.get_and_wrap();
    let e2: CRc<Entity> = holder_2.get_and_wrap();

    // chain_with: callee calls with_partner
    let chained: CRc<Entity> = e1.chain_with(e2.clone());

    // apply_twice: two args, callee calls with_partner
    let out: CRc<Entity> = chained.apply_twice(e1.clone(), e2.clone());

    // process_holder: Load inside Entity callee
    let loaded_item: CRc<Item> = e1.process_holder(holder_1.clone());
    let _ = loaded_item.get_id();

    let _ = out.describe();
    let _ = out.get_id();

    std::mem::drop(holder_1);
    std::mem::drop(holder_2);
}

fn main() {
    entry_full_rcpta_demo();
}
