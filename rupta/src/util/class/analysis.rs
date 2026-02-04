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
pub fn extract_class_name_from_func(func_name: &str) -> Option<String> {
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

/// Whether the **caller** is a "source-level allocation context" for rcpta.
/// Only when this is true should we create a ClassObj in ClassPAG for a class constructor Call.
/// Author: Yan Wang, Date: 2026-02-02
///
/// Returns false (do not create rcpta ClassObj) when the call is from:
/// - A class constructor body (caller name contains `_classes::_` and `::new`) — internal Call
/// - DSL internal helpers (e.g. `classes::ptr::` / `into_raw`) — not user-visible allocation
pub fn is_source_level_allocation_caller(caller_func_name: &str) -> bool {
    if caller_func_name.contains("_classes::_") && caller_func_name.contains("::new") {
        return false; // inside a class constructor
    }
    if caller_func_name.contains("classes::ptr::") || caller_func_name.contains("::into_raw") {
        return false; // DSL internal (into_raw etc.)
    }
    true
}

/// Whether the **caller** is a "source-level cast context" for rcpta.
/// Only when this is true should we add Cast edge and ClassPtr in ClassPAG for a class cast Call.
/// Author: Yan Wang, Date: 2026-02-02
///
/// Returns false (do not add rcpta Cast/ClassPtr) when the call is from:
/// - A cast method implementation (caller name contains into_superclass, try_into_subtype, cast_mixin)
/// - DSL internal (e.g. classes::ptr::, _delegate_ctor) — not user-visible cast
pub fn is_source_level_cast_caller(caller_func_name: &str) -> bool {
    if caller_func_name.contains("into_superclass")
        || caller_func_name.contains("try_into_subtype")
        || caller_func_name.contains("cast_mixin")
    {
        return false; // inside a cast method implementation
    }
    if caller_func_name.contains("classes::ptr::") || caller_func_name.contains("_delegate_ctor") {
        return false; // DSL internal
    }
    true
}

/// Whether the callee is a source-level class cast method (same object, different type view).
/// rcpta: add Assign edge receiver → destination. Author: Yan Wang, Date: 2026-02-02
///
/// Recognized: into_superclass, try_into_subtype, cast_mixin (classes crate / _classes).
pub fn identify_class_cast_method(func_ref: &Rc<FunctionReference>) -> bool {
    let name = func_ref.to_string();
    let is_cast_name = name.contains("into_superclass")
        || name.contains("try_into_subtype")
        || name.contains("cast_mixin");
    let is_classes = name.contains("classes::ptr") || name.contains("_classes::_");
    is_cast_name && is_classes
}

/// Class name string for a DSL class type (e.g. "Dog", "Animal").
/// Returns None if `ty` is not a DSL class type (path contains `_classes::_`).
pub fn class_name_of_dsl_type<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<String> {
    if let TyKind::Adt(adt_def, _) = ty.kind() {
        let path = tcx.def_path_str(adt_def.did());
        if let Some(start) = path.find("_classes::_") {
            let after = &path[start + 11..];
            let end = after.find("::").unwrap_or(after.len());
            return Some(after[..end].to_string());
        }
    }
    None
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
/// A getter/setter method must have the exact pattern: get_<field_name> or set_<field_name>
/// where field_name contains no underscores (single word). Methods like get_internal_point
/// or get_point_sum are NOT getters, they are regular class methods.
/// 
/// Examples:
/// - "simple_load_store::_classes::_Container::{impl#0}::get_point" -> Some(GetterSetter { class_name: "Container", field_name: "point", is_getter: true })
/// - "simple_load_store::_classes::_Container::{impl#0}::set_point" -> Some(GetterSetter { class_name: "Container", field_name: "point", is_getter: false })
/// - "simple_method_call::_classes::_Container::{impl#0}::get_internal_point" -> None (not a getter, has underscore in field name)
/// - "simple_method_call::_classes::_Container::{impl#0}::get_point_sum" -> None (not a getter, has underscore in field name)
pub fn identify_getter_setter(func_ref: &Rc<FunctionReference>) -> Option<GetterSetter> {
    let func_name = func_ref.to_string();
    
    // Pattern: _classes::_ClassName::{impl#N}::get_field_name or set_field_name
    if let Some(class_name) = extract_class_name_from_func(&func_name) {
        // Check for get_ prefix
        if let Some(get_start) = func_name.find("::get_") {
            let after_get = &func_name[get_start + 6..]; // "::get_" is 6 chars
            let field_name_end = after_get.find("::").unwrap_or(after_get.len());
            let field_name = after_get[..field_name_end].to_string();
            
            // Only recognize as getter if field_name contains no underscores
            // (i.e., it's a single-word field name like "point", not "internal_point")
            if !field_name.contains('_') {
                return Some(GetterSetter {
                    class_name,
                    field_name,
                    is_getter: true,
                    func_name,
                });
            }
        }
        
        // Check for set_ prefix
        if let Some(set_start) = func_name.find("::set_") {
            let after_set = &func_name[set_start + 6..]; // "::set_" is 6 chars
            let field_name_end = after_set.find("::").unwrap_or(after_set.len());
            let field_name = after_set[..field_name_end].to_string();
            
            // Only recognize as setter if field_name contains no underscores
            if !field_name.contains('_') {
                return Some(GetterSetter {
                    class_name,
                    field_name,
                    is_getter: false,
                    func_name,
                });
            }
        }
    }
    
    None
}

/// Information about a class method call
#[derive(Debug, Clone)]
pub struct ClassMethod {
    /// The class name (e.g., "Point", "Container")
    pub class_name: String,
    /// The method name (e.g., "sum_coords", "distance_to")
    pub method_name: String,
    /// The full function name
    pub func_name: String,
}

/// Identifies if a function is a class method (not a constructor, getter, or setter)
/// 
/// Examples:
/// - "simple_method_call::_classes::_Point::{impl#0}::sum_coords" -> Some(ClassMethod { class_name: "Point", method_name: "sum_coords" })
/// - "simple_method_call::_classes::_Container::{impl#0}::distance_to" -> Some(ClassMethod { class_name: "Container", method_name: "distance_to" })
/// - "simple_method_call::_classes::_Container::{impl#0}::get_internal_point" -> Some(ClassMethod { class_name: "Container", method_name: "get_internal_point" })
/// - "simple_method_call::_classes::_Container::{impl#0}::get_point_sum" -> Some(ClassMethod { class_name: "Container", method_name: "get_point_sum" })
/// - "simple_method_call::_classes::_Point::{impl#0}::new" -> None (constructor)
/// - "simple_method_call::_classes::_Container::{impl#0}::get_point" -> None (getter)
/// - "simple_method_call::_classes::_Container::{impl#0}::set_point" -> None (setter)
pub fn identify_class_method(func_ref: &Rc<FunctionReference>) -> Option<ClassMethod> {
    let func_name = func_ref.to_string();
    
    // Must be in _classes::_ namespace
    if let Some(class_name) = extract_class_name_from_func(&func_name) {
        // Must be a wrapper method (not data::)
        if func_name.contains("::data::") {
            return None;
        }
        
        // Must not be a constructor
        if func_name.contains("::new") {
            return None;
        }
        
        // Must not be a getter or setter
        // Use identify_getter_setter to accurately identify getters/setters
        // This avoids false positives for methods like get_internal_point or get_point_sum
        if identify_getter_setter(func_ref).is_some() {
            return None;
        }
        
        // Extract method name: _classes::_ClassName::{impl#N}::method_name
        // Pattern: find the last "::" before the end
        if let Some(impl_start) = func_name.find("::{impl#") {
            let after_impl = &func_name[impl_start + 7..]; // "::{impl#" is 7 chars
            if let Some(impl_end) = after_impl.find("}::") {
                let after_impl_end = &after_impl[impl_end + 3..]; // "}::" is 3 chars
                // The method name is everything after the last "}::"
                let method_name = after_impl_end.to_string();
                
                // Skip if method_name is empty or contains "::" (shouldn't happen for wrapper methods)
                if !method_name.is_empty() && !method_name.contains("::") {
                    return Some(ClassMethod {
                        class_name,
                        method_name,
                        func_name,
                    });
                }
            }
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
/// If `field_index` is provided, it will directly access that field after unwrapping
/// to the struct type, instead of traversing all fields.
/// 
/// Returns the inner DSL class type if found, or None if no DSL class type
/// is found after unwrapping all wrappers.
pub fn extract_dsl_class_from_wrapper<'tcx>(
    tcx: TyCtxt<'tcx>, 
    ty: Ty<'tcx>,
    field_index: Option<usize>,
) -> Option<Ty<'tcx>> {
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
                    if let Some(field_idx) = field_index {
                        // If field_index is provided, directly access that specific field
                        if field_idx < variant.fields.len() {
                            let field_ty = type_util::get_field_type(tcx, current_ty, field_idx);
                            debug!("extract_dsl_class_from_wrapper: checking field {} of {:?}", 
                                   field_idx, current_ty);
                            // Recursively check this specific field type
                            if let Some(class_ty) = extract_dsl_class_from_wrapper(tcx, field_ty, None) {
                                debug!("extract_dsl_class_from_wrapper: found DSL class type {:?} in field {} of {:?}", 
                                       class_ty, field_idx, current_ty);
                                return Some(class_ty);
                            }
                        } else {
                            debug!("extract_dsl_class_from_wrapper: field index {} out of bounds for {:?}", 
                                   field_idx, current_ty);
                        }
                    } else {
                        // Fallback: check a limited number of fields if no field_index provided
                        let max_fields_to_check = 5;
                        for (field_idx, _field) in variant.fields.iter().enumerate().take(max_fields_to_check) {
                            let field_ty = type_util::get_field_type(tcx, current_ty, field_idx);
                            // Recursively check this field type
                            if let Some(class_ty) = extract_dsl_class_from_wrapper(tcx, field_ty, None) {
                                debug!("extract_dsl_class_from_wrapper: found DSL class type {:?} in field {} of {:?}", 
                                       class_ty, field_idx, current_ty);
                                return Some(class_ty);
                            }
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
                // Rc<T> -> T (including classes::Rc which is a re-export of alloc::rc::Rc)
                else if path_str == "alloc::rc::Rc" || path_str == "std::rc::Rc" || path_str == "classes::Rc" {
                    if let Some(GenericArgKind::Type(inner_ty)) = args.get(0).map(|arg| arg.unpack()) {
                        debug!("extract_dsl_class_from_wrapper: unwrapping Rc<{:?}>", inner_ty);
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
                // RcDyn<T> (from classes::ptr::RcDyn) - unwrap to field 0 type (ManuallyDrop<Rc<Dyn<T::Data>>>)
                // Note: We use field type instead of generic arg because RcDyn's internal structure
                // is { data: ManuallyDrop<Rc<Dyn<C::Data>>>, vtable: ... }
                else if path_str.contains("classes::ptr::RcDyn") {
                    // Get field 0's type (the actual data field)
                    let field_ty = type_util::get_field_type(tcx, current_ty, 0);
                    debug!("extract_dsl_class_from_wrapper: unwrapping RcDyn, field 0 type = {:?}", field_ty);
                    current_ty = field_ty;
                    depth += 1;
                    continue;
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
