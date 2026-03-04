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

use rustc_middle::mir::Location;

/// Placeholder for context-sensitive analysis. Replace with real context type (e.g. k-CFA stack) later.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
// k-callsites
pub struct DSLContextElement {
    /// Function name (e.g. `main`, `Point::new`)
    pub func: String,
    /// MIR location (e.g. `bb0[6]`, `bb1[0]`)
    pub location: Location,
}

impl DSLContextElement {
    pub fn new(func: String, location: Location) -> Self {
        Self { func, location }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Context {
    pub context_elems: Vec<DSLContextElement>,
}

impl Context {
    pub fn new_empty() -> Self {
        Context {
            context_elems: Vec::new(),
        }
    }

    pub fn new(context_elems: Vec<DSLContextElement>) -> Self {
        Context { context_elems }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.context_elems.len()
    }

    pub fn empty(&self) -> bool {
        self.context_elems.is_empty()
    }

    pub fn last_elem(&self) -> Option<&DSLContextElement> {
        self.context_elems.last()
    }

    /// Composes a new context from a given context and a new context element.
    /// Discard the last old context element if the length of context exceeds the depth limit.
    pub fn new_k_limited_context(old_ctx: Context, elem: DSLContextElement, k: usize) -> Self {
        let mut elems = Vec::with_capacity(k);
        if k > 0 {
            elems.push(elem);
            if old_ctx.len() < k {
                elems.extend_from_slice(&old_ctx.context_elems[..])
            } else {
                elems.extend_from_slice(&old_ctx.context_elems[..k - 1])
            }
        }
        Context { context_elems: elems }
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.empty() {
            write!(
                f,
                "[{}]",
                self.context_elems
                    .iter()
                    .map(|elem| {
                        format!(
                            "{}:bb{}[{}]",
                            elem.func,
                            elem.location.block.index(),
                            elem.location.statement_index
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("->")
            )
        } else {
            write!(f, "")
        }
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
    /// Context for context-sensitive analysis (empty when context-insensitive)
    pub context: Context,
}

impl ClassPtr {
    pub fn new_local(id: String, class_type: String, ctx: Context) -> Self {
        Self {
            id: id.clone(),
            class_type,
            kind: ClassPtrKind::Local,
            context: ctx,
        }
    }

    pub fn make_local_id(ordinal: &usize, func_name: Option<&str>, ctx: Context) -> String {
        match func_name {
            Some(name) => format!("{}{}::local_{}", ctx, name, ordinal),
            None => format!("{}local_{}", ctx, ordinal),
        }
    }

    pub fn new_param(func_name: &str, param_index: usize, class_type: String, ctx: Context) -> Self {
        Self {
            id: Self::make_param_id(&param_index, Some(func_name), ctx.clone()),
            class_type,
            kind: ClassPtrKind::Param,
            context: ctx,
        }
    }

    pub fn make_param_id(param_index: &usize, func_name: Option<&str>, ctx: Context) -> String {
        match func_name {
            Some(name) => format!("{}{}::param_{}", ctx, name, param_index),
            None => format!("{}param_{}", ctx, param_index),
        }
    }

    pub fn new_return(func_name: &str, class_type: String, ctx: Context) -> Self {
        Self {
            id: Self::make_return_id(Some(func_name), ctx.clone()),
            class_type,
            kind: ClassPtrKind::Return,
            context: ctx,
        }
    }

    pub fn make_return_id(func_name: Option<&str>, ctx: Context) -> String {
        match func_name {
            Some(name) => format!("{}{}::ret", ctx, name),
            None => format!("{}ret", ctx),
        }
    }

    pub fn new_instance_field(base_id: &str, field_name: &str, class_type: String, ctx: Context) -> Self {
        Self {
            id: Self::make_instance_field_id(base_id, field_name, ctx.clone()),
            class_type,
            kind: ClassPtrKind::InstanceField {
                base: base_id.to_string(),
                field_name: field_name.to_string(),
            },
            context: ctx,
        }
    }

    pub fn make_instance_field_id(base_id: &str, field_name: &str, ctx: Context) -> String {
        format!("{}{}.{}", ctx, base_id, field_name)
    }

    pub fn new_static_field(class_name: &str, field_name: &str, class_type: String, ctx: Context) -> Self {
        Self {
            id: format!("{}::{}", class_name, field_name),
            class_type,
            kind: ClassPtrKind::StaticField {
                class_name: class_name.to_string(),
                field_name: field_name.to_string(),
            },
            context: ctx,
        }
    }

    pub fn make_static_field_id(class_name: &str, field_name: &str, ctx: Context) -> String {
        format!("{}{}::{}", ctx, class_name, field_name)
    }

    pub fn new_temp(id: String, class_type: String, ctx: Context) -> Self {
        Self {
            id: id.clone(),
            class_type,
            kind: ClassPtrKind::Temp,
            context: ctx,
        }
    }

    // Attach context (for context-sensitive analysis).
    // pub fn with_context(mut self, ctx: Context) -> Self {
    //     self.context = ctx;
    //     self
    // }
}

impl fmt::Display for ClassPtr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.context, self.id, self.class_type)
    }
}
