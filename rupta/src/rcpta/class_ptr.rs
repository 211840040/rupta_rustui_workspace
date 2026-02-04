// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.
//
// rcpta: Author: Yan Wang, Date: 2026-02-02

//! Class-level pointer (ClassPtr): models a class reference in the program.
//!
//! A ClassPtr represents one of: local variable, parameter, return value,
//! instance field, or static field — each holding a reference to a class instance.

use std::fmt;

/// Placeholder for context-sensitive analysis. Replace with real context type (e.g. k-CFA stack) later.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Context(());

impl Context {
    pub fn new() -> Self {
        Context(())
    }
}

/// Kind of class pointer: where the reference lives in the program.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClassPtrKind {
    /// Local variable (e.g. `let p: CRc<Point> = ...` → `main::p`)
    Local,
    /// Method parameter (e.g. `fn foo(&self, other: &CRc<Point>)` → `foo::self`, `foo::other`)
    Param,
    /// Method return value (e.g. `fn get_point(&self) -> CRc<Point>` → `get_point::ret`)
    Return,
    /// Instance field (e.g. `class Container { point: CRc<Point> }` → `Container.point`)
    InstanceField {
        /// Base pointer id (e.g. `main::_c1`)
        base: String,
        /// Field name (e.g. `point`)
        field_name: String,
    },
    /// Static field (e.g. `class Singleton { static instance: CRc<Singleton> }` → `Singleton::instance`)
    StaticField {
        /// Class name
        class_name: String,
        /// Field name
        field_name: String,
    },
    /// Temporary / internal (e.g. intermediate in getter/setter expansion)
    Temp,
}

/// Class-level pointer: models one class reference (variable, parameter, return, field).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassPtr {
    /// Unique identifier (e.g. `main::p`, `foo::param_1`, `Container.point`)
    pub id: String,
    /// Class type this pointer points to (e.g. `Point`, `Container`)
    pub class_type: String,
    /// Kind of pointer
    pub kind: ClassPtrKind,
    /// Context for context-sensitive analysis (optional; unused when context-insensitive)
    pub context: Option<Context>,
}

impl ClassPtr {
    pub fn new_local(id: String, class_type: String) -> Self {
        Self {
            id: id.clone(),
            class_type,
            kind: ClassPtrKind::Local,
            context: None,
        }
    }

    pub fn new_param(func_name: &str, param_index: usize, class_type: String) -> Self {
        Self {
            id: format!("{}::param_{}", func_name, param_index),
            class_type,
            kind: ClassPtrKind::Param,
            context: None,
        }
    }

    pub fn new_return(func_name: &str, class_type: String) -> Self {
        Self {
            id: format!("{}::ret", func_name),
            class_type,
            kind: ClassPtrKind::Return,
            context: None,
        }
    }

    pub fn new_instance_field(base_id: &str, field_name: &str, class_type: String) -> Self {
        Self {
            id: format!("{}.{}", base_id, field_name),
            class_type,
            kind: ClassPtrKind::InstanceField {
                base: base_id.to_string(),
                field_name: field_name.to_string(),
            },
            context: None,
        }
    }

    pub fn new_static_field(class_name: &str, field_name: &str, class_type: String) -> Self {
        Self {
            id: format!("{}::{}", class_name, field_name),
            class_type,
            kind: ClassPtrKind::StaticField {
                class_name: class_name.to_string(),
                field_name: field_name.to_string(),
            },
            context: None,
        }
    }

    pub fn new_temp(id: String, class_type: String) -> Self {
        Self {
            id: id.clone(),
            class_type,
            kind: ClassPtrKind::Temp,
            context: None,
        }
    }

    /// Attach context (for context-sensitive analysis).
    pub fn with_context(mut self, ctx: Context) -> Self {
        self.context = Some(ctx);
        self
    }
}

impl fmt::Display for ClassPtr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.id, self.class_type)
    }
}
