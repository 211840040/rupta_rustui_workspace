// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.

//! Class-level analysis utilities for DSL programs.
//! 
//! This module provides utilities to identify and analyze class-related operations
//! in programs that use the classes DSL macro.

use std::rc::Rc;
use rustc_middle::ty::{Ty, TyCtxt, TyKind};
use log::*;
use crate::mir::function::FunctionReference;

/// Information about a class constructor call
#[derive(Debug, Clone)]
pub struct ClassConstructor {
    /// The class name (e.g., "Point")
    pub class_name: String,
    /// The full function name
    pub func_name: String,
    /// Whether this is the wrapper constructor or the data constructor
    pub is_wrapper: bool,
}

/// Identifies if a function is a class constructor (new operation)
pub fn identify_class_constructor(func_ref: &Rc<FunctionReference>) -> Option<ClassConstructor> {
    let func_name = func_ref.to_string();
    
    // Pattern 1: _classes::_Point::{impl#0}::new (wrapper constructor)
    // Pattern 2: _classes::_Point::data::{impl#0}::new (data constructor)
    if let Some(class_name) = extract_class_name_from_func(&func_name) {
        if func_name.contains("::new") {
            let is_wrapper = !func_name.contains("::data::");
            return Some(ClassConstructor {
                class_name,
                func_name,
                is_wrapper,
            });
        }
    }
    
    None
}

/// Extracts class name from a function name
/// 
/// Examples:
/// - "simple_new::_classes::_Point::{impl#0}::new" -> Some("Point")
/// - "simple_new::_classes::_Point::data::{impl#0}::new" -> Some("Point")
fn extract_class_name_from_func(func_name: &str) -> Option<String> {
    // Pattern: _classes::_ClassName::...
    if let Some(start) = func_name.find("_classes::_") {
        let after_prefix = &func_name[start + 11..]; // "_classes::_" is 11 chars
        if let Some(end) = after_prefix.find("::") {
            return Some(after_prefix[..end].to_string());
        }
    }
    None
}

/// Checks if a function is class-related (belongs to the classes DSL system)
pub fn is_class_related(func_ref: &Rc<FunctionReference>) -> bool {
    let func_name = func_ref.to_string();
    func_name.contains("_classes::_")
}

/// Information about a getter or setter method
#[derive(Debug, Clone)]
pub struct GetterSetter {
    /// The class name (e.g., "Container")
    pub class_name: String,
    /// The field name (e.g., "point")
    pub field_name: String,
    /// Whether this is a getter (true) or setter (false)
    pub is_getter: bool,
    /// The full function name
    pub func_name: String,
}

/// Identifies if a function is a getter or setter method
/// 
/// Examples:
/// - "simple_load_store::_classes::_Container::{impl#0}::get_point" -> Some(GetterSetter { class_name: "Container", field_name: "point", is_getter: true })
/// - "simple_load_store::_classes::_Container::{impl#0}::set_point" -> Some(GetterSetter { class_name: "Container", field_name: "point", is_getter: false })
pub fn identify_getter_setter(func_ref: &Rc<FunctionReference>) -> Option<GetterSetter> {
    let func_name = func_ref.to_string();
    
    // Pattern: _classes::_ClassName::{impl#N}::get_field_name or set_field_name
    if let Some(class_name) = extract_class_name_from_func(&func_name) {
        // Check for get_ prefix
        if let Some(get_start) = func_name.find("::get_") {
            let after_get = &func_name[get_start + 6..]; // "::get_" is 6 chars
            let field_name_end = after_get.find("::").unwrap_or(after_get.len());
            let field_name = after_get[..field_name_end].to_string();
            return Some(GetterSetter {
                class_name,
                field_name,
                is_getter: true,
                func_name,
            });
        }
        
        // Check for set_ prefix
        if let Some(set_start) = func_name.find("::set_") {
            let after_set = &func_name[set_start + 6..]; // "::set_" is 6 chars
            let field_name_end = after_set.find("::").unwrap_or(after_set.len());
            let field_name = after_set[..field_name_end].to_string();
            return Some(GetterSetter {
                class_name,
                field_name,
                is_getter: false,
                func_name,
            });
        }
    }
    
    None
}

/// Checks if a heap object path represents a class instance
/// This function recursively checks the base path for QualifiedPath and OffsetPath
/// to find the underlying HeapObj
pub fn is_class_instance_heap_obj(
    acx: &crate::mir::analysis_context::AnalysisContext,
    path: &crate::mir::path::PathEnum,
) -> bool {
    use crate::mir::path::{Path, PathEnum};
    
    // Check if this is directly a HeapObj
    if let PathEnum::HeapObj { func_id, location } = path {
        let path_rc = Path::new_heap_obj(*func_id, *location);
        return acx.class_instance_heap_objs.contains(&path_rc);
    }
    
    // For QualifiedPath and OffsetPath, recursively check the base
    let base_path = match path {
        PathEnum::QualifiedPath { base, .. } => Some(base.clone()),
        PathEnum::OffsetPath { base, .. } => Some(base.clone()),
        _ => None,
    };
    
    if let Some(base) = base_path {
        // Recursively check the base path
        return is_class_instance_heap_obj(acx, &base.value);
    }
    
    false
}

/// Checks if a type is a DSL class type
/// 
/// DSL class types are structs defined in the `_classes::_` module.
/// Examples:
/// - `Point<ClassMarker, Virtual>` (wrapper type)
/// - `Point<RcDyn<Point>, Virtual>` (instance type)
/// - `Container<ClassMarker, Virtual>`
/// 
/// This function checks if the type's DefId path contains `_classes::_`.
pub fn is_dsl_class_type<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> bool {
    if let TyKind::Adt(adt_def, _args) = ty.kind() {
        // Get the DefId's path string representation
        let def_id = adt_def.did();
        let path = tcx.def_path_str(def_id);
        let is_class = path.contains("_classes::_");
        debug!("is_dsl_class_type: ty={:?}, path={}, result={}", ty, path, is_class);
        is_class
    } else {
        debug!("is_dsl_class_type: ty={:?}, not an Adt, result=false", ty);
        false
    }
}

/// Recursively unwraps wrapper types (ManuallyDrop, Rc, Dyn, etc.) to extract
/// the inner DSL class type, if any.
/// 
/// This function handles the case where DSL class types are wrapped in:
/// - `ManuallyDrop<T>` -> unwraps to T
/// - `Rc<T>` -> unwraps to T
/// - `Dyn<T>` -> unwraps to T
/// - `Cell<T>` -> unwraps to T
/// - `Option<T>` -> unwraps to T
/// 
/// Returns the inner DSL class type if found, or None if no DSL class type
/// is found after unwrapping all wrappers.
pub fn extract_dsl_class_from_wrapper<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
    use rustc_middle::ty::GenericArgKind;
    use crate::util::type_util;
    
    let mut current_ty = ty;
    let mut depth = 0;
    const MAX_DEPTH: usize = 10; // Prevent infinite recursion
    
    while depth < MAX_DEPTH {
        // First, check if current_ty is already a DSL class type
        if is_dsl_class_type(tcx, current_ty) {
            debug!("extract_dsl_class_from_wrapper: found DSL class type {:?} at depth {}", current_ty, depth);
            return Some(current_ty);
        }
        
        // Also check if this is a struct with DSL class type fields
        // For example, Container::data::Container has a point field of type Cell<Option<CRc<Point>>>
        if let TyKind::Adt(adt_def, _args) = current_ty.kind() {
            if adt_def.is_struct() {
                let variant = adt_def.variants().iter().next();
                if let Some(variant) = variant {
                    // Check all fields to see if any contain a DSL class type
                    for (field_idx, _field) in variant.fields.iter().enumerate() {
                        let field_ty = type_util::get_field_type(tcx, current_ty, field_idx);
                        // Recursively check this field type
                        if let Some(class_ty) = extract_dsl_class_from_wrapper(tcx, field_ty) {
                            debug!("extract_dsl_class_from_wrapper: found DSL class type {:?} in field {} of {:?}", 
                                   class_ty, field_idx, current_ty);
                            return Some(class_ty);
                        }
                    }
                }
            }
        }
        
        // Try to unwrap various wrapper types
        match current_ty.kind() {
            // ManuallyDrop<T> -> T
            TyKind::Adt(adt_def, args) => {
                let def_id = adt_def.did();
                let path_str = tcx.def_path_str(def_id);
                
                if path_str == "std::mem::ManuallyDrop" || path_str == "core::mem::ManuallyDrop" {
                    // ManuallyDrop has a single type parameter (index 0)
                    if let Some(GenericArgKind::Type(inner_ty)) = args.get(0).map(|arg| arg.unpack()) {
                        current_ty = inner_ty;
                        depth += 1;
                        continue;
                    }
                }
                // Rc<T> -> T
                else if path_str == "alloc::rc::Rc" || path_str == "std::rc::Rc" {
                    if let Some(GenericArgKind::Type(inner_ty)) = args.get(0).map(|arg| arg.unpack()) {
                        current_ty = inner_ty;
                        depth += 1;
                        continue;
                    }
                }
                // Cell<T> -> T
                else if path_str == "core::cell::Cell" || path_str == "std::cell::Cell" {
                    if let Some(GenericArgKind::Type(inner_ty)) = args.get(0).map(|arg| arg.unpack()) {
                        current_ty = inner_ty;
                        depth += 1;
                        continue;
                    }
                }
                // Option<T> -> T
                else if path_str == "core::option::Option" || path_str == "std::option::Option" {
                    if let Some(GenericArgKind::Type(inner_ty)) = args.get(0).map(|arg| arg.unpack()) {
                        current_ty = inner_ty;
                        depth += 1;
                        continue;
                    }
                }
                // Dyn<T> (from classes::ptr::Dyn)
                else if path_str.contains("classes::ptr::Dyn") {
                    if let Some(GenericArgKind::Type(inner_ty)) = args.get(0).map(|arg| arg.unpack()) {
                        current_ty = inner_ty;
                        depth += 1;
                        continue;
                    }
                }
            }
            // &T -> T
            TyKind::Ref(_, inner_ty, _) => {
                current_ty = *inner_ty;
                depth += 1;
                continue;
            }
            // *const T or *mut T -> T
            TyKind::RawPtr(inner_ty, _) => {
                current_ty = *inner_ty;
                depth += 1;
                continue;
            }
            _ => {
                // No more wrappers to unwrap
                break;
            }
        }
        
        // If we didn't continue, break the loop
        break;
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_class_name() {
        assert_eq!(
            extract_class_name_from_func("simple_new::_classes::_Point::{impl#0}::new"),
            Some("Point".to_string())
        );
        assert_eq!(
            extract_class_name_from_func("simple_new::_classes::_Point::data::{impl#0}::new"),
            Some("Point".to_string())
        );
        assert_eq!(
            extract_class_name_from_func("simple_new::main"),
            None
        );
    }
}
