// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.

//! Class-level Pointer and Object Abstraction System
//! 
//! This module provides a simplified abstraction for class pointers and objects,
//! independent of RUPTA's Path abstraction. It allows us to verify that our analysis
//! correctly models class references as pointers and heap allocations as objects.
//!
//! Key concepts:
//! - ClassPtr: A pointer/reference to a class instance (variables, parameters, return values)
//! - ClassObj: A heap-allocated class instance
//! - ClassPointsTo: Points-to relationships between pointers and objects

use std::collections::{HashMap, HashSet};
use std::fmt;
use log::*;

/// Represents a class-level pointer (variable, parameter, return value, field)
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassPtr {
    /// Pointer identifier (e.g., "main::_p1", "main::_c1.point", "func::param_1")
    pub id: String,
    /// Class type this pointer points to
    pub class_type: String,
    /// Pointer kind
    pub kind: ClassPtrKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClassPtrKind {
    /// Local variable (e.g., "main::_p1")
    Local,
    /// Function parameter (e.g., "func::param_1")
    Param,
    /// Return value (e.g., "func::ret")
    Return,
    /// Field access (e.g., "main::_c1.point")
    Field { base: String, field_name: String },
    /// Temporary value
    Temp,
}

impl ClassPtr {
    pub fn new_local(id: String, class_type: String) -> Self {
        Self {
            id,
            class_type,
            kind: ClassPtrKind::Local,
        }
    }

    pub fn new_param(func_name: String, param_index: usize, class_type: String) -> Self {
        Self {
            id: format!("{}::param_{}", func_name, param_index),
            class_type,
            kind: ClassPtrKind::Param,
        }
    }

    pub fn new_return(func_name: String, class_type: String) -> Self {
        Self {
            id: format!("{}::ret", func_name),
            class_type,
            kind: ClassPtrKind::Return,
        }
    }

    pub fn new_field(base: String, field_name: String, class_type: String) -> Self {
        Self {
            id: format!("{}.{}", base, field_name),
            class_type,
            kind: ClassPtrKind::Field { base, field_name },
        }
    }

    pub fn new_temp(id: String, class_type: String) -> Self {
        Self {
            id,
            class_type,
            kind: ClassPtrKind::Temp,
        }
    }
}

impl fmt::Display for ClassPtr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.id, self.class_type)
    }
}

/// Represents a heap-allocated class instance
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassObj {
    /// Object identifier (e.g., "obj_0", "obj_1")
    pub id: String,
    /// Class type of this object
    pub class_type: String,
    /// Allocation location (function name and location for debugging)
    pub alloc_location: String,
}

impl ClassObj {
    pub fn new(id: String, class_type: String, alloc_location: String) -> Self {
        Self {
            id,
            class_type,
            alloc_location,
        }
    }
}

impl fmt::Display for ClassObj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}@{}", self.id, self.class_type, self.alloc_location)
    }
}

/// Class-level points-to analysis system
#[derive(Debug, Default)]
pub struct ClassPtrSystem {
    /// All class pointers: ptr_id -> ClassPtr
    ptrs: HashMap<String, ClassPtr>,
    
    /// All class objects: obj_id -> ClassObj
    objs: HashMap<String, ClassObj>,
    
    /// Points-to relationships: ptr_id -> set of obj_ids
    points_to: HashMap<String, HashSet<String>>,
    
    /// Object counter for generating unique IDs
    obj_counter: usize,
}

impl ClassPtrSystem {
    pub fn new() -> Self {
        Self::default()
    }

    // ==================== Pointer Management ====================

    /// Register or get a class pointer
    pub fn get_or_create_ptr(&mut self, ptr: ClassPtr) -> String {
        let ptr_id = ptr.id.clone();
        self.ptrs.entry(ptr_id.clone()).or_insert_with(|| {
            debug!("ClassPtrSystem: Created pointer {}", ptr);
            ptr
        });
        ptr_id
    }

    /// Get a pointer by ID
    pub fn get_ptr(&self, ptr_id: &str) -> Option<&ClassPtr> {
        self.ptrs.get(ptr_id)
    }

    // ==================== Object Management ====================

    /// Create a new class object (called during allocation/constructor)
    pub fn create_obj(&mut self, class_type: String, alloc_location: String) -> String {
        let obj_id = format!("obj_{}", self.obj_counter);
        self.obj_counter += 1;
        
        let obj = ClassObj::new(obj_id.clone(), class_type.clone(), alloc_location);
        self.objs.insert(obj_id.clone(), obj.clone());
        
        debug!("ClassPtrSystem: Created object {}", obj);
        obj_id
    }

    /// Get an object by ID
    pub fn get_obj(&self, obj_id: &str) -> Option<&ClassObj> {
        self.objs.get(obj_id)
    }

    // ==================== Points-to Management ====================

    /// Add a points-to relationship: ptr -> obj
    pub fn add_points_to(&mut self, ptr_id: &str, obj_id: &str) {
        self.points_to
            .entry(ptr_id.to_string())
            .or_insert_with(HashSet::new)
            .insert(obj_id.to_string());
        
        debug!("ClassPtrSystem: {} -> {}", ptr_id, obj_id);
    }

    /// Propagate points-to from source pointer to destination pointer
    /// (called during assign operations)
    pub fn propagate_points_to(&mut self, src_ptr_id: &str, dst_ptr_id: &str) {
        // Clone the source objects to avoid borrowing issues
        let src_objs: Vec<String> = self.points_to.get(src_ptr_id)
            .map(|objs| objs.iter().cloned().collect())
            .unwrap_or_default();
        
        if !src_objs.is_empty() {
            let dst_objs = self.points_to
                .entry(dst_ptr_id.to_string())
                .or_insert_with(HashSet::new);
            
            for obj_id in &src_objs {
                dst_objs.insert(obj_id.clone());
            }
            
            debug!("ClassPtrSystem: Propagated {} -> {} ({} objects)", 
                   src_ptr_id, dst_ptr_id, src_objs.len());
        }
    }

    /// Get all objects a pointer points to
    pub fn get_points_to(&self, ptr_id: &str) -> Vec<&ClassObj> {
        self.points_to.get(ptr_id)
            .map(|obj_ids| {
                obj_ids.iter()
                    .filter_map(|id| self.objs.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    // ==================== Query Interface ====================

    /// Get all pointers
    pub fn get_all_ptrs(&self) -> Vec<&ClassPtr> {
        let mut ptrs: Vec<_> = self.ptrs.values().collect();
        ptrs.sort();
        ptrs
    }

    /// Get all objects
    pub fn get_all_objs(&self) -> Vec<&ClassObj> {
        let mut objs: Vec<_> = self.objs.values().collect();
        objs.sort();
        objs
    }

    /// Get all points-to relationships
    pub fn get_all_points_to(&self) -> Vec<(&ClassPtr, Vec<&ClassObj>)> {
        let mut result = Vec::new();
        for ptr in self.get_all_ptrs() {
            let objs = self.get_points_to(&ptr.id);
            if !objs.is_empty() {
                result.push((ptr, objs));
            }
        }
        result
    }

    // ==================== Statistics ====================

    pub fn stats(&self) -> ClassPtrSystemStats {
        ClassPtrSystemStats {
            num_ptrs: self.ptrs.len(),
            num_objs: self.objs.len(),
            num_points_to: self.points_to.values().map(|s| s.len()).sum(),
            num_ptrs_with_targets: self.points_to.len(),
        }
    }
}

#[derive(Debug)]
pub struct ClassPtrSystemStats {
    pub num_ptrs: usize,
    pub num_objs: usize,
    pub num_points_to: usize,
    pub num_ptrs_with_targets: usize,
}

impl fmt::Display for ClassPtrSystemStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ClassPtrSystem Stats: {} pointers, {} objects, {} points-to relationships ({} pointers have targets)",
               self.num_ptrs, self.num_objs, self.num_points_to, self.num_ptrs_with_targets)
    }
}

// ==================== Helper Functions ====================

/// Generate a simplified string representation of a Path for ClassPtrSystem
/// This creates a human-readable identifier independent of RUPTA's Path abstraction.
/// For Option::Some.0 (path = base + [Downcast(1), Field(0)]), returns the base's id so the
/// Option-holder local (e.g. downcast_to_eagle) is used consistently instead of base.as_variant#1.0.
///
/// When `param_slots` is `Some(s)` (e.g. 1 + num_params from MIR), LocalVariable(ordinal) with
/// ordinal < s is treated as return/params so CallArg formals (param_1, ...) match callee body ptr_ids.
pub fn path_to_class_ptr_id(
    path: &crate::mir::path::Path,
    func_name: Option<&str>,
    param_slots: Option<usize>,
) -> String {
    use crate::mir::path::{PathEnum, PathSelector};
    
    match &path.value {
        PathEnum::LocalVariable { func_id: _, ordinal } => {
            // In callee body, use param_N/ret for parameter slots so they match CallArg formals.
            if let (Some(fn_name), Some(s)) = (func_name, param_slots) {
                if *ordinal < s {
                    if *ordinal == 0 {
                        return format!("{}::ret", fn_name);
                    }
                    return format!("{}::param_{}", fn_name, ordinal);
                }
            }
            if let Some(fn_name) = func_name {
                format!("{}::local_{}", fn_name, ordinal)
            } else {
                format!("local_{}", ordinal)
            }
        }
        PathEnum::Parameter { func_id: _, ordinal } => {
            if let Some(fn_name) = func_name {
                format!("{}::param_{}", fn_name, ordinal)
            } else {
                format!("param_{}", ordinal)
            }
        }
        PathEnum::ReturnValue { func_id: _ } => {
            if let Some(fn_name) = func_name {
                format!("{}::ret", fn_name)
            } else {
                "ret".to_string()
            }
        }
        PathEnum::HeapObj { func_id: _, location } => {
            format!("heap_{:?}", location)
        }
        PathEnum::QualifiedPath { base, projection } => {
            // Option<CRc<T>>.Some.0: use the Option-holder local as the pointer id for consistency.
            if projection.len() == 2 {
                if let PathSelector::Downcast(1) = projection[0] {
                    if let PathSelector::Field(0) = projection[1] {
                        return path_to_class_ptr_id(base, func_name, param_slots);
                    }
                }
            }
            let base_id = path_to_class_ptr_id(base, func_name, param_slots);
            // Simplify projection to field name if possible
            let proj_str = projection.iter()
                .map(|sel| format!("{:?}", sel))
                .collect::<Vec<_>>()
                .join(".");
            format!("{}.{}", base_id, proj_str)
        }
        PathEnum::OffsetPath { base, offset } => {
            let base_id = path_to_class_ptr_id(base, func_name, param_slots);
            format!("{}.ofs({})", base_id, offset)
        }
        _ => {
            format!("{:?}", path)
        }
    }
}
