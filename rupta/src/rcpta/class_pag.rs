// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.
//
// rcpta: Author: Yan Wang, Date: 2026-02-02

//! Class-level Pointer Assignment Graph (ClassPAG).
//!
//! Nodes are class pointers (ClassPtr) and class objects (ClassObj).
//! Edges represent pointer flow: assign, alloc, load, store, and call (arg/ret).

use std::collections::{HashMap, HashSet};

use crate::rcpta::Context;

use super::class_obj::ClassObj;
use super::class_ptr::ClassPtr;

/// Unique identifier for a call site (e.g. `main:bb1[2]`, `Container::get_point:bb0[3]`).
pub type CallSiteId = String;

/// Field name (e.g. `point`, `data`).
pub type FieldId = String;

// ---------- Edge builders: used when constructing the PAG ----------

/// Assign edge: `dst = src` — flow from src pointer to dst pointer (copy/move).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssignEdge {
    pub src_ptr_id: String,
    pub dst_ptr_id: String,
}

/// Cast edge: `dst = src.cast(...)` — same heap obj, different type view (into_superclass, try_into_subtype, cast_mixin).
/// Propagation: pts(dst) += pts(src); obj concrete type is unchanged (ClassObj.class_type set at allocation).
/// Solver may apply type filter when propagating along Cast (e.g. Tai-e: only add objs compatible with dst static type).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CastEdge {
    pub src_ptr_id: String,
    pub dst_ptr_id: String,
}

/// Alloc edge: `ptr = ClassName::new(...)` — ptr points to obj.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AllocEdge {
    pub ptr_id: String,
    pub obj_id: String,
}

/// Load edge: `dst = base.field` (getter) — flow from (base, field) to dst.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoadEdge {
    pub base_ptr_id: String,
    pub field: FieldId,
    pub dst_ptr_id: String,
}

/// Store edge: `base.field = src` (setter) — flow from src to (base, field).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StoreEdge {
    pub base_ptr_id: String,
    pub field: FieldId,
    pub src_ptr_id: String,
}

/// Call argument: actual → formal at a call site.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallArgEdge {
    pub call_site: CallSiteId,
    pub arg_idx: usize,
    pub actual_ptr_id: String,
    pub formal_ptr_id: String,
}

/// Call return: formal_ret → actual_ret at a call site.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallRetEdge {
    pub call_site: CallSiteId,
    pub formal_ret_ptr_id: String,
    pub actual_ret_ptr_id: String,
}

// ---------- ClassPAG ----------

/// Class-level Pointer Assignment Graph.
///
/// - **Nodes**: ClassPtr (by id) and ClassObj (by id).
/// - **Edges**: Assign, Cast, Alloc, Load, Store, CallArg, CallRet.
pub struct ClassPAG {
    /// All class pointers: ptr_id → ClassPtr
    ptrs: HashMap<String, ClassPtr>,
    /// All class objects: obj_id → ClassObj
    objs: HashMap<String, ClassObj>,

    /// Assign: src_ptr_id → set of dst_ptr_id (copy/move)
    assign: HashMap<String, HashSet<String>>,
    /// Cast: src_ptr_id → set of dst_ptr_id (same obj, different type view)
    cast: HashMap<String, HashSet<String>>,
    /// Alloc: ptr_id → set of obj_id
    alloc: HashMap<String, HashSet<String>>,
    /// Load: (base_ptr_id, field) → set of dst_ptr_id
    load: HashMap<(String, FieldId), HashSet<String>>,
    /// Store: (base_ptr_id, field) → set of src_ptr_id
    store: HashMap<(String, FieldId), HashSet<String>>,

    /// Call argument edges (for propagation: actual → formal)
    call_arg: Vec<CallArgEdge>,
    /// Call return edges (formal_ret → actual_ret)
    call_ret: Vec<CallRetEdge>,

    /// Next object id counter (obj_0, obj_1, ...)
    next_obj_counter: usize,
}

impl Default for ClassPAG {
    fn default() -> Self {
        Self::new()
    }
}

impl ClassPAG {
    pub fn new() -> Self {
        Self {
            ptrs: HashMap::new(),
            objs: HashMap::new(),
            assign: HashMap::new(),
            cast: HashMap::new(),
            alloc: HashMap::new(),
            load: HashMap::new(),
            store: HashMap::new(),
            call_arg: Vec::new(),
            call_ret: Vec::new(),
            next_obj_counter: 0,
        }
    }

    // ---------- Node registration ----------

    /// Register or get a class pointer. Returns its id.
    pub fn get_or_create_ptr(&mut self, ptr: ClassPtr) -> String {
        let id = ptr.id.clone();
        let is_new = !self.ptrs.contains_key(&id);
        self.ptrs.entry(id.clone()).or_insert(ptr);
        if is_new && (id.contains("get_and_wrap::local_2") || id.contains("get_and_wrap::local_3")) {
            eprintln!("[rcpta dedup] ClassPAG new ptr id={}", id);
        }
        id
    }

    /// Register or get an instance-field pointer (base.field) so it appears in Pointers and participates in PTS.
    /// Id is `{base_ptr_id}.{field}`. Call this before add_load/add_store when modeling instance field access.
    pub fn get_or_create_field_ptr(
        &mut self,
        base_ptr_id: impl AsRef<str>,
        field: impl AsRef<str>,
        field_class_type: impl Into<String>,
    ) -> String {
        let base = base_ptr_id.as_ref();
        let f = field.as_ref();
        // field ptr and base ptr share the same context. Right?
        let ctx = if let Some(base_ptr) = self.get_ptr(base) {
            base_ptr.context.clone()
        } else {
            Context::new_empty()
        };
        let ptr = ClassPtr::new_instance_field(base, f, field_class_type.into(), ctx);
        self.get_or_create_ptr(ptr)
    }

    /// Register or get a class object. Returns its id.
    pub fn get_or_create_obj(&mut self, obj: ClassObj) -> String {
        let id = obj.id.clone();
        self.objs.entry(id.clone()).or_insert(obj);
        id
    }

    /// Create a new class object with auto-generated id (obj_0, obj_1, ...) and register it.
    pub fn create_obj(
        &mut self,
        class_type: impl Into<String>,
        alloc_site: super::class_obj::AllocSite,
    ) -> String {
        let id = format!("obj_{}", self.next_obj_counter);
        self.next_obj_counter += 1;
        let obj = ClassObj::new(id.clone(), class_type, alloc_site);
        self.get_or_create_obj(obj)
    }

    pub fn create_obj_with_context(
        &mut self,
        class_type: impl Into<String>,
        alloc_site: super::class_obj::AllocSite,
        context: Context,
    ) -> String {
        let id = format!("obj_{}", self.next_obj_counter);
        self.next_obj_counter += 1;
        let obj = ClassObj::new(id.clone(), class_type, alloc_site).with_context(context);
        self.get_or_create_obj(obj)
    }

    /// Get a pointer by id.
    pub fn get_ptr(&self, id: &str) -> Option<&ClassPtr> {
        self.ptrs.get(id)
    }

    /// Get an object by id.
    pub fn get_obj(&self, id: &str) -> Option<&ClassObj> {
        self.objs.get(id)
    }

    /// All pointer ids.
    pub fn ptr_ids(&self) -> impl Iterator<Item = &String> {
        self.ptrs.keys()
    }

    /// All object ids.
    pub fn obj_ids(&self) -> impl Iterator<Item = &String> {
        self.objs.keys()
    }

    /// Number of pointers and objects.
    pub fn num_ptrs(&self) -> usize {
        self.ptrs.len()
    }
    pub fn num_objs(&self) -> usize {
        self.objs.len()
    }

    // ---------- Assign ----------

    /// Add assign edge: dst = src.
    pub fn add_assign(&mut self, src_ptr_id: impl Into<String>, dst_ptr_id: impl Into<String>) {
        let src = src_ptr_id.into();
        let dst = dst_ptr_id.into();
        self.assign.entry(src).or_default().insert(dst);
    }

    /// Assign successors of a pointer (all dst such that dst = src).
    pub fn assign_successors(&self, src_ptr_id: &str) -> impl Iterator<Item = &String> {
        self.assign.get(src_ptr_id).into_iter().flat_map(|set| set.iter())
    }

    /// Iterate all assign edges (src_id, dst_id).
    pub fn iter_assign_edges(&self) -> impl Iterator<Item = (String, String)> + '_ {
        self.assign
            .iter()
            .flat_map(|(src, dsts)| dsts.iter().map(move |dst| (src.clone(), dst.clone())))
    }

    // ---------- Cast ----------

    /// Add cast edge: dst = src.cast(...) — same obj, different type view. Edge is marked as cast (not assign).
    pub fn add_cast(&mut self, src_ptr_id: impl Into<String>, dst_ptr_id: impl Into<String>) {
        let src = src_ptr_id.into();
        let dst = dst_ptr_id.into();
        self.cast.entry(src).or_default().insert(dst);
    }

    /// Cast successors of a pointer (all dst such that dst = src.cast(...)).
    pub fn cast_successors(&self, src_ptr_id: &str) -> impl Iterator<Item = &String> {
        self.cast.get(src_ptr_id).into_iter().flat_map(|set| set.iter())
    }

    /// Iterate all cast edges (src_id, dst_id).
    pub fn iter_cast_edges(&self) -> impl Iterator<Item = (String, String)> + '_ {
        self.cast
            .iter()
            .flat_map(|(src, dsts)| dsts.iter().map(move |dst| (src.clone(), dst.clone())))
    }

    // ---------- Alloc ----------

    /// Add alloc edge: ptr points to obj.
    pub fn add_alloc(&mut self, ptr_id: impl Into<String>, obj_id: impl Into<String>) {
        let ptr = ptr_id.into();
        let obj = obj_id.into();
        self.alloc.entry(ptr).or_default().insert(obj);
    }

    /// Objects that this pointer is allocated to (initial points-to).
    pub fn alloc_targets(&self, ptr_id: &str) -> impl Iterator<Item = &String> {
        self.alloc.get(ptr_id).into_iter().flat_map(|set| set.iter())
    }

    /// Iterate all alloc edges (ptr_id, obj_id).
    pub fn iter_alloc_edges(&self) -> impl Iterator<Item = (String, String)> + '_ {
        self.alloc
            .iter()
            .flat_map(|(ptr, objs)| objs.iter().map(move |obj| (ptr.clone(), obj.clone())))
    }

    // ---------- Load ----------

    /// Add load edge: dst = base.field.
    pub fn add_load(
        &mut self,
        base_ptr_id: impl Into<String>,
        field: impl Into<FieldId>,
        dst_ptr_id: impl Into<String>,
    ) {
        let base = base_ptr_id.into();
        let field = field.into();
        let dst = dst_ptr_id.into();
        self.load.entry((base, field)).or_default().insert(dst);
    }

    /// Load successors: (base, field) → set of dst_ptr_id.
    pub fn load_targets(&self, base_ptr_id: &str, field: &str) -> impl Iterator<Item = &String> {
        self.load
            .get(&(base_ptr_id.to_string(), field.to_string()))
            .into_iter()
            .flat_map(|set| set.iter())
    }

    /// Iterate all load edges.
    pub fn iter_load_edges(&self) -> impl Iterator<Item = LoadEdge> + '_ {
        self.load.iter().flat_map(|((base, field), dsts)| {
            dsts.iter().map(move |dst| LoadEdge {
                base_ptr_id: base.clone(),
                field: field.clone(),
                dst_ptr_id: dst.clone(),
            })
        })
    }

    // ---------- Store ----------

    /// Add store edge: base.field = src.
    pub fn add_store(
        &mut self,
        base_ptr_id: impl Into<String>,
        field: impl Into<FieldId>,
        src_ptr_id: impl Into<String>,
    ) {
        let base = base_ptr_id.into();
        let field = field.into();
        let src = src_ptr_id.into();
        self.store.entry((base, field)).or_default().insert(src);
    }

    /// Store sources: (base, field) ← set of src_ptr_id.
    pub fn store_sources(&self, base_ptr_id: &str, field: &str) -> impl Iterator<Item = &String> {
        self.store
            .get(&(base_ptr_id.to_string(), field.to_string()))
            .into_iter()
            .flat_map(|set| set.iter())
    }

    /// Iterate all store edges.
    pub fn iter_store_edges(&self) -> impl Iterator<Item = StoreEdge> + '_ {
        self.store.iter().flat_map(|((base, field), srcs)| {
            srcs.iter().map(move |src| StoreEdge {
                base_ptr_id: base.clone(),
                field: field.clone(),
                src_ptr_id: src.clone(),
            })
        })
    }

    // ---------- Call ----------

    /// Add call argument edge: actual → formal at call_site, arg_idx.
    pub fn add_call_arg(
        &mut self,
        call_site: impl Into<CallSiteId>,
        arg_idx: usize,
        actual_ptr_id: impl Into<String>,
        formal_ptr_id: impl Into<String>,
    ) {
        self.call_arg.push(CallArgEdge {
            call_site: call_site.into(),
            arg_idx,
            actual_ptr_id: actual_ptr_id.into(),
            formal_ptr_id: formal_ptr_id.into(),
        });
    }

    /// Add call return edge: formal_ret → actual_ret at call_site.
    pub fn add_call_ret(
        &mut self,
        call_site: impl Into<CallSiteId>,
        formal_ret_ptr_id: impl Into<String>,
        actual_ret_ptr_id: impl Into<String>,
    ) {
        self.call_ret.push(CallRetEdge {
            call_site: call_site.into(),
            formal_ret_ptr_id: formal_ret_ptr_id.into(),
            actual_ret_ptr_id: actual_ret_ptr_id.into(),
        });
    }

    /// All call argument edges.
    pub fn call_arg_edges(&self) -> &[CallArgEdge] {
        &self.call_arg
    }

    /// All call return edges.
    pub fn call_ret_edges(&self) -> &[CallRetEdge] {
        &self.call_ret
    }
}
