// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.
//
// rcpta: Author: Yan Wang, Date: 2026-02-02

//! Class-level points-to set computation on ClassPAG.
//!
//! Store/Load are **constraints**: when obj flows to base, we materialize store (src -> obj.field)
//! and load (obj.field -> dst). PTS and materialized edges are computed together until fixpoint.

use std::collections::{HashMap, HashSet};

use super::ClassPAG;

/// Result of class-level points-to analysis: ptr_id -> set of obj_id.
pub type ClassPTS = HashMap<String, HashSet<String>>;

/// Store edge materialized when obj flows to base: src_ptr -> (obj, field).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MaterializedStoreEdge {
    pub src_ptr_id: String,
    pub obj_id: String,
    pub field: String,
}

/// Load edge materialized when obj flows to base: (obj, field) -> dst_ptr.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MaterializedLoadEdge {
    pub obj_id: String,
    pub field: String,
    pub dst_ptr_id: String,
}

/// Result of solve: PTS plus edges materialized from store/load constraints.
#[derive(Debug, Clone)]
pub struct ClassPTSResult {
    pub pts: ClassPTS,
    pub materialized_stores: Vec<MaterializedStoreEdge>,
    pub materialized_loads: Vec<MaterializedLoadEdge>,
}

/// Runs propagation on the ClassPAG until fixpoint.
/// Store/Load are constraints: for each obj in pts[base], we update content[(obj, field)] and
/// create pointer obj.field; materialized store (src -> obj.field) and load (obj.field -> dst)
/// are recorded after convergence.
pub fn solve_class_pts(pag: &ClassPAG) -> ClassPTSResult {
    let mut pts: ClassPTS = HashMap::new();
    // (obj_id, field) -> set of obj_id that may be stored in this field
    let mut content: HashMap<(String, String), HashSet<String>> = HashMap::new();

    // Initialize: every pointer has an entry; Alloc gives initial points-to.
    for ptr_id in pag.ptr_ids() {
        pts.entry(ptr_id.clone()).or_default();
    }
    for (ptr_id, obj_id) in pag.iter_alloc_edges() {
        pts.get_mut(&ptr_id).unwrap().insert(obj_id);
    }

    // Iterate until fixpoint.
    loop {
        let mut changed = false;

        // Assign: dst may point to whatever src points to
        for (src, dst) in pag.iter_assign_edges() {
            let src_set = pts.get(&src).cloned().unwrap_or_default();
            if !src_set.is_empty() {
                let d = pts.entry(dst.clone()).or_default();
                let prev = d.len();
                d.extend(src_set);
                if d.len() > prev {
                    changed = true;
                }
            }
        }

        // Cast: same as assign
        for (src, dst) in pag.iter_cast_edges() {
            let src_set = pts.get(&src).cloned().unwrap_or_default();
            if !src_set.is_empty() {
                let d = pts.entry(dst.clone()).or_default();
                let prev = d.len();
                d.extend(src_set);
                if d.len() > prev {
                    changed = true;
                }
            }
        }

        // CallArg: actual -> formal
        for e in pag.call_arg_edges() {
            let src_set = pts.get(&e.actual_ptr_id).cloned().unwrap_or_default();
            if !src_set.is_empty() {
                let d = pts.entry(e.formal_ptr_id.clone()).or_default();
                let prev = d.len();
                d.extend(src_set);
                if d.len() > prev {
                    changed = true;
                }
            }
        }

        // CallRet: formal_ret -> actual_ret (actual_ret may not be in pag.ptr_ids() if from caller not yet in PAG)
        for e in pag.call_ret_edges() {
            let src_set = pts.get(&e.formal_ret_ptr_id).cloned().unwrap_or_default();
            if !src_set.is_empty() {
                let d = pts.entry(e.actual_ret_ptr_id.clone()).or_default();
                let prev = d.len();
                d.extend(src_set);
                if d.len() > prev {
                    changed = true;
                }
            }
        }

        // Store constraint: base.field <- src  =>  for each obj in pts[base], content[(obj, field)] += pts[src]
        // (obj.field pointer is created when content is updated; pts[obj.field] synced below)
        for e in pag.iter_store_edges() {
            let base_objs = pts.get(&e.base_ptr_id).cloned().unwrap_or_default();
            let src_objs = pts.get(&e.src_ptr_id).cloned().unwrap_or_default();
            if base_objs.is_empty() || src_objs.is_empty() {
                continue;
            }
            for obj in &base_objs {
                let key = (obj.clone(), e.field.clone());
                let c = content.entry(key).or_default();
                let prev = c.len();
                c.extend(src_objs.clone());
                if c.len() > prev {
                    changed = true;
                }
            }
        }

        // Sync obj.field pointers: pts[obj.field] = content[(obj, field)] for each (obj, field) in content
        for ((obj, field), objs_set) in &content.clone() {
            let obj_field_id = format!("{}.{}", obj, field);
            let d = pts.entry(obj_field_id.clone()).or_default();
            let prev = d.len();
            d.extend(objs_set.iter().cloned());
            if d.len() > prev {
                changed = true;
            }
        }

        // Load constraint: base.field -> dst  =>  for each obj in pts[base], pts[dst] += content[(obj, field)]
        for e in pag.iter_load_edges() {
            let base_objs = pts.get(&e.base_ptr_id).cloned().unwrap_or_default();
            if base_objs.is_empty() {
                continue;
            }
            let mut to_add = HashSet::new();
            for obj in &base_objs {
                let key = (obj.clone(), e.field.clone());
                if let Some(c) = content.get(&key) {
                    to_add.extend(c.iter().cloned());
                }
            }
            if !to_add.is_empty() {
                let d = pts.entry(e.dst_ptr_id.clone()).or_default();
                let prev = d.len();
                d.extend(to_add);
                if d.len() > prev {
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }
    }

    // After fixpoint: materialize store/load edges from constraints using final pts
    let mut materialized_stores = Vec::new();
    for e in pag.iter_store_edges() {
        let base_objs = pts.get(&e.base_ptr_id).cloned().unwrap_or_default();
        for obj in base_objs {
            materialized_stores.push(MaterializedStoreEdge {
                src_ptr_id: e.src_ptr_id.clone(),
                obj_id: obj,
                field: e.field.clone(),
            });
        }
    }
    let mut materialized_loads = Vec::new();
    for e in pag.iter_load_edges() {
        let base_objs = pts.get(&e.base_ptr_id).cloned().unwrap_or_default();
        for obj in base_objs {
            materialized_loads.push(MaterializedLoadEdge {
                obj_id: obj,
                field: e.field.clone(),
                dst_ptr_id: e.dst_ptr_id.clone(),
            });
        }
    }

    ClassPTSResult {
        pts,
        materialized_stores,
        materialized_loads,
    }
}
