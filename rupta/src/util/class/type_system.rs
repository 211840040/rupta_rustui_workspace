// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.

//! Class Type System for DSL Classes
//! 
//! This module provides a simplified type system specifically for DSL classes,
//! independent of rustc's complex type representations. It tracks:
//! - Class definitions (name, fields, inheritance)
//! - Class instances (heap objects)
//! - Class references (pointers to class instances)
//! - Field types (which fields hold references to which classes)

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use log::*;

use crate::mir::path::Path;

/// Represents a field in a class
#[derive(Debug, Clone)]
pub struct ClassField {
    /// Field name (e.g., "point", "data")
    pub name: String,
    /// Field index in the class (0 = super_, 1+ = user fields)
    pub index: usize,
    /// If this field holds a class reference, the class name
    pub class_type: Option<String>,
}

/// Represents a class type definition
#[derive(Debug, Clone)]
pub struct ClassTypeInfo {
    /// Class name (e.g., "Point", "Container")
    pub name: String,
    /// Fields: field_name -> ClassField
    pub fields: HashMap<String, ClassField>,
    /// Method names (for quick lookup)
    pub methods: HashSet<String>,
    /// Method implementation: method_name -> func_name (full name for dispatch)
    /// Used for polymorphic resolution: given concrete type + method_name, get the callee func.
    pub method_impls: HashMap<String, String>,
    /// Parent class name (for inheritance)
    pub parent: Option<String>,
    /// All known subclasses
    pub subclasses: HashSet<String>,
}

impl ClassTypeInfo {
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: HashMap::new(),
            methods: HashSet::new(),
            method_impls: HashMap::new(),
            parent: None,
            subclasses: HashSet::new(),
        }
    }

    /// Adds or updates a field in this class
    pub fn add_field(&mut self, name: String, index: usize, class_type: Option<String>) {
        self.fields.insert(name.clone(), ClassField {
            name,
            index,
            class_type,
        });
    }

    /// Gets the field index by name
    pub fn get_field_index(&self, field_name: &str) -> Option<usize> {
        self.fields.get(field_name).map(|f| f.index)
    }

    /// Gets the field name by index (for rcpta: map cell path projection to field name)
    pub fn get_field_name_by_index(&self, field_index: usize) -> Option<String> {
        self.fields
            .iter()
            .find(|(_, f)| f.index == field_index)
            .map(|(name, _)| name.clone())
    }

    /// Gets the field's class type (if it's a class reference)
    pub fn get_field_class_type(&self, field_name: &str) -> Option<&String> {
        self.fields.get(field_name).and_then(|f| f.class_type.as_ref())
    }

    /// Adds a method to this class
    pub fn add_method(&mut self, method_name: String) {
        self.methods.insert(method_name);
    }

    /// Checks if this class has a method
    pub fn has_method(&self, method_name: &str) -> bool {
        self.methods.contains(method_name)
    }

    /// Registers the implementing function for a method (for polymorphic dispatch).
    pub fn add_method_impl(&mut self, method_name: String, func_name: String) {
        self.methods.insert(method_name.clone());
        self.method_impls.insert(method_name, func_name);
    }
}

/// The class type system that manages all class type information
#[derive(Debug, Default)]
pub struct ClassTypeSystem {
    /// All registered class types: class_name -> ClassTypeInfo
    classes: HashMap<String, ClassTypeInfo>,
    
    /// Path to class type mapping: path -> class_name
    /// This tracks what class type each path (variable, heap object) has
    path_class_types: HashMap<Rc<Path>, String>,
    
    /// Class instances: heap_object_path -> class_name
    /// Specifically tracks heap objects that are class instances
    class_instances: HashMap<Rc<Path>, String>,
    
    /// Class references: path -> (class_name, is_direct_ref)
    /// Tracks pointers that reference class instances
    /// is_direct_ref: true if directly points to instance, false if through field
    class_references: HashMap<Rc<Path>, (String, bool)>,
    
    /// Field index counter per class for sequential assignment
    field_counters: HashMap<String, usize>,
}

impl ClassTypeSystem {
    pub fn new() -> Self {
        Self::default()
    }

    // ==================== Class Registration ====================

    /// Registers a new class type (called when analyzing class constructor)
    pub fn register_class(&mut self, class_name: &str) -> &mut ClassTypeInfo {
        if !self.classes.contains_key(class_name) {
            debug!("ClassTypeSystem: Registering new class '{}'", class_name);
            self.classes.insert(class_name.to_string(), ClassTypeInfo::new(class_name.to_string()));
            self.field_counters.insert(class_name.to_string(), 0);
        }
        self.classes.get_mut(class_name).unwrap()
    }

    /// Gets class type info (immutable)
    pub fn get_class(&self, class_name: &str) -> Option<&ClassTypeInfo> {
        self.classes.get(class_name)
    }

    /// Gets class type info (mutable)
    pub fn get_class_mut(&mut self, class_name: &str) -> Option<&mut ClassTypeInfo> {
        self.classes.get_mut(class_name)
    }

    /// Sets inheritance relationship
    pub fn set_parent(&mut self, class_name: &str, parent_name: &str) {
        // Register both classes if not already registered
        self.register_class(class_name);
        self.register_class(parent_name);
        
        // Set parent
        if let Some(class_info) = self.classes.get_mut(class_name) {
            class_info.parent = Some(parent_name.to_string());
        }
        
        // Add to parent's subclasses
        if let Some(parent_info) = self.classes.get_mut(parent_name) {
            parent_info.subclasses.insert(class_name.to_string());
        }
        
        debug!("ClassTypeSystem: Set {} extends {}", class_name, parent_name);
    }

    // ==================== Field Management ====================

    /// Registers a field for a class (called when analyzing getter/setter)
    /// Returns the assigned field index
    pub fn register_field(&mut self, class_name: &str, field_name: &str, field_class_type: Option<&str>) -> usize {
        self.register_class(class_name);
        
        // Check if field already exists
        if let Some(class_info) = self.classes.get(class_name) {
            if let Some(index) = class_info.get_field_index(field_name) {
                // Update class type if newly discovered
                if field_class_type.is_some() {
                    if let Some(class_info) = self.classes.get_mut(class_name) {
                        if let Some(field) = class_info.fields.get_mut(field_name) {
                            if field.class_type.is_none() {
                                field.class_type = field_class_type.map(|s| s.to_string());
                                debug!("ClassTypeSystem: Updated field {}.{} type to {:?}", 
                                       class_name, field_name, field_class_type);
                            }
                        }
                    }
                }
                return index;
            }
        }
        
        // Assign new field index
        let counter = self.field_counters.entry(class_name.to_string()).or_insert(0);
        let index = *counter;
        *counter += 1;
        
        // Add field to class
        if let Some(class_info) = self.classes.get_mut(class_name) {
            class_info.add_field(
                field_name.to_string(), 
                index, 
                field_class_type.map(|s| s.to_string())
            );
        }
        
        debug!("ClassTypeSystem: Registered field {}.{} at index {} (type: {:?})", 
               class_name, field_name, index, field_class_type);
        
        index
    }

    /// Registers a field with a specific index (used when we see cell_option_set/cell_option_get in a
    /// getter/setter body and know the MIR field index). Ensures get_field_name_by_index(class_name, index)
    /// returns field_name. Does nothing if the field is already registered for this class at this index.
    pub fn register_field_with_index(
        &mut self,
        class_name: &str,
        field_name: &str,
        index: usize,
        field_class_type: Option<&str>,
    ) {
        self.register_class(class_name);
        if let Some(class_info) = self.classes.get(class_name) {
            if class_info.get_field_name_by_index(index).as_deref() == Some(field_name) {
                return;
            }
        }
        if let Some(class_info) = self.classes.get(class_name) {
            if class_info.get_field_index(field_name).is_some() {
                return;
            }
        }
        if let Some(class_info) = self.classes.get_mut(class_name) {
            class_info.add_field(
                field_name.to_string(),
                index,
                field_class_type.map(|s| s.to_string()),
            );
        }
        let counter = self.field_counters.entry(class_name.to_string()).or_insert(0);
        if *counter <= index {
            *counter = index + 1;
        }
        debug!("ClassTypeSystem: Registered field {}.{} at index {} (type: {:?})",
               class_name, field_name, index, field_class_type);
    }

    /// Gets the field index for a class field
    pub fn get_field_index(&self, class_name: &str, field_name: &str) -> Option<usize> {
        self.classes.get(class_name).and_then(|c| c.get_field_index(field_name))
    }

    /// Gets the field's class type (if it's a class reference field)
    pub fn get_field_class_type(&self, class_name: &str, field_name: &str) -> Option<String> {
        self.classes
            .get(class_name)
            .and_then(|c| c.get_field_class_type(field_name).cloned())
    }

    /// Gets the field name for a class field by index (for rcpta: cell_option_set/get base path -> class + field index -> name)
    pub fn get_field_name_by_index(&self, class_name: &str, field_index: usize) -> Option<String> {
        self.classes
            .get(class_name)
            .and_then(|c| c.get_field_name_by_index(field_index))
    }

    /// Builds a field path for getter/setter operations.
    /// 
    /// Given a base path (e.g., `&Container`) and field name (e.g., "point"),
    /// this method:
    /// 1. Registers the field if not already registered
    /// 2. Constructs the path: (*base).field_index
    /// 3. Returns the constructed path
    /// 
    /// This centralizes the field index management in the type system.
    /// 
    /// # Arguments
    /// * `class_name` - The class containing the field (e.g., "Container")
    /// * `field_name` - The field being accessed (e.g., "point")
    /// * `base_path` - The self reference path (e.g., `&self`)
    /// * `field_class_type` - Optional: the class type stored in this field
    /// 
    /// # Returns
    /// A tuple of (field_path, field_index)
    pub fn build_field_path(
        &mut self, 
        class_name: &str, 
        field_name: &str, 
        base_path: Rc<Path>,
        field_class_type: Option<&str>
    ) -> (Rc<Path>, usize) {
        use crate::mir::path::PathSelector;
        
        // Register field and get index
        let field_index = self.register_field(class_name, field_name, field_class_type);
        
        // Build path: (*base).field_index
        let deref_base = Path::new_qualified(base_path, vec![PathSelector::Deref]);
        let field_path = Path::new_field(deref_base, field_index);
        
        debug!("ClassTypeSystem: Built field path {}.{} -> {:?} (index={})", 
               class_name, field_name, field_path, field_index);
        
        (field_path, field_index)
    }

    /// Updates the class type of a field (called when type is inferred from setter value)
    pub fn update_field_class_type(&mut self, class_name: &str, field_name: &str, field_class_type: &str) {
        if let Some(class_info) = self.classes.get_mut(class_name) {
            if let Some(field) = class_info.fields.get_mut(field_name) {
                if field.class_type.is_none() {
                    field.class_type = Some(field_class_type.to_string());
                    debug!("ClassTypeSystem: Updated field {}.{} type to '{}'", 
                           class_name, field_name, field_class_type);
                }
            }
        }
    }

    // ==================== Instance Tracking ====================

    /// Marks a heap object as a class instance (called when analyzing constructor allocation)
    pub fn mark_class_instance(&mut self, heap_path: Rc<Path>, class_name: &str) {
        self.register_class(class_name);
        self.class_instances.insert(heap_path.clone(), class_name.to_string());
        self.path_class_types.insert(heap_path.clone(), class_name.to_string());
        debug!("ClassTypeSystem: Marked {:?} as instance of '{}'", heap_path, class_name);
    }

    /// Checks if a path is a class instance
    pub fn is_class_instance(&self, path: &Rc<Path>) -> bool {
        self.class_instances.contains_key(path)
    }

    /// Gets the class name of a class instance
    pub fn get_instance_class(&self, path: &Rc<Path>) -> Option<&String> {
        self.class_instances.get(path)
    }

    // ==================== Reference Tracking ====================

    /// Marks a path as a class reference (called when analyzing assignments)
    pub fn mark_class_reference(&mut self, path: Rc<Path>, class_name: &str, is_direct: bool) {
        self.class_references.insert(path.clone(), (class_name.to_string(), is_direct));
        self.path_class_types.insert(path.clone(), class_name.to_string());
        debug!("ClassTypeSystem: Marked {:?} as reference to '{}' (direct={})", 
               path, class_name, is_direct);
    }

    /// Checks if a path is a class reference
    pub fn is_class_reference(&self, path: &Rc<Path>) -> bool {
        self.class_references.contains_key(path)
    }

    /// Gets the class name of a class reference
    pub fn get_reference_class(&self, path: &Rc<Path>) -> Option<&String> {
        self.class_references.get(path).map(|(name, _)| name)
    }

    // ==================== Type Propagation ====================

    /// Propagates class type from source to destination (called during assign analysis)
    pub fn propagate_type(&mut self, src: &Rc<Path>, dst: Rc<Path>) {
        if let Some(class_name) = self.path_class_types.get(src).cloned() {
            self.path_class_types.insert(dst.clone(), class_name.clone());
            
            // Also mark as reference if source was a reference
            if self.class_references.contains_key(src) {
                self.class_references.insert(dst, (class_name, false));
            }
        }
    }

    /// Gets the class type of any path
    pub fn get_path_class_type(&self, path: &Rc<Path>) -> Option<&String> {
        self.path_class_types.get(path)
    }

    /// Sets the class type of a path
    pub fn set_path_class_type(&mut self, path: Rc<Path>, class_name: &str) {
        self.path_class_types.insert(path, class_name.to_string());
    }

    // ==================== Type Checking ====================

    /// Checks if two paths have the same class type
    pub fn same_class_type(&self, path1: &Rc<Path>, path2: &Rc<Path>) -> bool {
        match (self.get_path_class_type(path1), self.get_path_class_type(path2)) {
            (Some(t1), Some(t2)) => t1 == t2,
            _ => false,
        }
    }

    /// Checks if path1's class type is a subtype of path2's class type
    pub fn is_subtype(&self, subtype: &str, supertype: &str) -> bool {
        if subtype == supertype {
            return true;
        }
        
        // Check inheritance chain
        if let Some(class_info) = self.classes.get(subtype) {
            if let Some(parent) = &class_info.parent {
                return self.is_subtype(parent, supertype);
            }
        }
        
        false
    }

    // ==================== Statistics ====================

    /// Returns statistics about the type system
    pub fn stats(&self) -> ClassTypeSystemStats {
        ClassTypeSystemStats {
            num_classes: self.classes.len(),
            num_instances: self.class_instances.len(),
            num_references: self.class_references.len(),
            num_typed_paths: self.path_class_types.len(),
        }
    }

    /// Gets all registered class names
    pub fn get_all_class_names(&self) -> Vec<&String> {
        self.classes.keys().collect()
    }

    // ==================== Method Registration ====================

    /// Registers a method for a class (called when analyzing method calls)
    pub fn register_method(&mut self, class_name: &str, method_name: &str) {
        self.register_class(class_name);
        if let Some(class_info) = self.classes.get_mut(class_name) {
            class_info.add_method(method_name.to_string());
            debug!("ClassTypeSystem: Registered method '{}.{}'", class_name, method_name);
        }
    }

    /// Registers the implementing function for (class_name, method_name). Used to build the
    /// dispatch table for polymorphic resolution: given concrete type + method_name, resolve to callee.
    pub fn register_method_impl(&mut self, class_name: &str, method_name: &str, func_name: String) {
        self.register_class(class_name);
        if let Some(class_info) = self.classes.get_mut(class_name) {
            debug!("ClassTypeSystem: Registered method impl '{}.{}' -> {}", class_name, method_name, func_name);
            class_info.add_method_impl(method_name.to_string(), func_name);
        }
    }

    /// Resolves the callee for a (concrete_type, method_name) using the class hierarchy.
    /// Walks up the parent chain until an implementation is found (same as Tai-e's dispatch).
    /// Returns the func_name (full name) of the method that would be invoked.
    pub fn dispatch(&self, concrete_type: &str, method_name: &str) -> Option<String> {
        let class_info = self.classes.get(concrete_type)?;
        if let Some(func_name) = class_info.method_impls.get(method_name) {
            return Some(func_name.clone());
        }
        if let Some(ref parent) = class_info.parent {
            return self.dispatch(parent, method_name);
        }
        None
    }

    /// Checks if a class has a method
    pub fn has_method(&self, class_name: &str, method_name: &str) -> bool {
        self.classes.get(class_name)
            .map(|ci| ci.has_method(method_name))
            .unwrap_or(false)
    }

    // ==================== Debug/Export ====================

    /// Get all class definitions for export
    pub fn get_all_classes(&self) -> &HashMap<String, ClassTypeInfo> {
        &self.classes
    }

    /// Get all class instances for export
    pub fn get_all_instances(&self) -> &HashMap<Rc<Path>, String> {
        &self.class_instances
    }

    /// Get all class references for export
    pub fn get_all_references(&self) -> &HashMap<Rc<Path>, (String, bool)> {
        &self.class_references
    }

    /// Get all path class types for export
    pub fn get_all_path_class_types(&self) -> &HashMap<Rc<Path>, String> {
        &self.path_class_types
    }
}

/// Statistics about the class type system
#[derive(Debug)]
pub struct ClassTypeSystemStats {
    pub num_classes: usize,
    pub num_instances: usize,
    pub num_references: usize,
    pub num_typed_paths: usize,
}

impl std::fmt::Display for ClassTypeSystemStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClassTypeSystem Stats: {} classes, {} instances, {} references, {} typed paths",
               self.num_classes, self.num_instances, self.num_references, self.num_typed_paths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_registration() {
        let mut cts = ClassTypeSystem::new();
        
        cts.register_class("Point");
        cts.register_class("Container");
        
        assert!(cts.get_class("Point").is_some());
        assert!(cts.get_class("Container").is_some());
        assert!(cts.get_class("Unknown").is_none());
    }

    #[test]
    fn test_field_registration() {
        let mut cts = ClassTypeSystem::new();
        
        let idx1 = cts.register_field("Container", "point", Some("Point"));
        let idx2 = cts.register_field("Container", "data", None);
        let idx1_again = cts.register_field("Container", "point", None);
        
        assert_eq!(idx1, 0);
        assert_eq!(idx2, 1);
        assert_eq!(idx1_again, idx1); // Same field returns same index
        
        let class_info = cts.get_class("Container").unwrap();
        assert_eq!(class_info.get_field_class_type("point"), Some(&"Point".to_string()));
    }

    #[test]
    fn test_inheritance() {
        let mut cts = ClassTypeSystem::new();
        
        cts.set_parent("Container", "BaseClass");
        
        assert!(cts.is_subtype("Container", "BaseClass"));
        assert!(cts.is_subtype("Container", "Container"));
        assert!(!cts.is_subtype("BaseClass", "Container"));
    }
}
