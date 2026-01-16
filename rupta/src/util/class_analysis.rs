// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.

//! Class-level analysis utilities for DSL programs.
//! 
//! This module provides utilities to identify and analyze class-related operations
//! in programs that use the classes DSL macro.

use std::rc::Rc;
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

/// Checks if a heap object path represents a class instance
pub fn is_class_instance_heap_obj(
    acx: &crate::mir::analysis_context::AnalysisContext,
    path: &crate::mir::path::PathEnum,
) -> bool {
    use crate::mir::path::{Path, PathEnum};
    
    if let PathEnum::HeapObj { .. } = path {
        // Reconstruct the Path to check in the set
        // This is a bit inefficient, but necessary for the check
        let path_rc = match path {
            PathEnum::HeapObj { func_id, location } => {
                Path::new_heap_obj(*func_id, *location)
            }
            _ => return false,
        };
        acx.class_instance_heap_objs.contains(&path_rc)
    } else {
        false
    }
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
