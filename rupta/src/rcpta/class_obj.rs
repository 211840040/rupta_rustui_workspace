// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.
//
// rcpta: Author: Yan Wang, Date: 2026-02-02

//! Class-level object (ClassObj): models a heap-allocated class instance.
//!
//! A ClassObj is created at a class constructor call (e.g. `Point::new(1, 2)`).

use std::fmt;

use super::class_ptr::Context;

/// Allocation site: where in the program this object was allocated (function + location).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AllocSite {
    /// Function name (e.g. `main`, `Point::new`)
    pub func: String,
    /// MIR location (e.g. `bb0[6]`, `bb1[0]`)
    pub location: String,
}

impl AllocSite {
    pub fn new(func: impl Into<String>, location: impl Into<String>) -> Self {
        Self {
            func: func.into(),
            location: location.into(),
        }
    }

    /// Single string form for display (e.g. `main:bb0[6]`).
    pub fn to_string(&self) -> String {
        format!("{}:{}", self.func, self.location)
    }
}

impl fmt::Display for AllocSite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.func, self.location)
    }
}

/// Class-level object: one heap-allocated class instance.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassObj {
    /// Unique identifier (e.g. `obj_0`, `obj_1`)
    pub id: String,
    /// Class type (e.g. `Point`, `Container`)
    pub class_type: String,
    /// Where this object was allocated
    pub alloc_site: AllocSite,
    /// Context for context-sensitive analysis (optional).
    pub context: Context,
}

impl ClassObj {
    pub fn new(id: impl Into<String>, class_type: impl Into<String>, alloc_site: AllocSite) -> Self {
        Self {
            id: id.into(),
            class_type: class_type.into(),
            alloc_site,
            context: Context::new_empty(),
        }
    }

    pub fn with_context(mut self, ctx: Context) -> Self {
        self.context = ctx;
        self
    }
}

impl fmt::Display for ClassObj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}@{}",
            self.context, self.id, self.class_type, self.alloc_site
        )
    }
}
