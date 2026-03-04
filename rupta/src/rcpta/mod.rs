// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.
//
// rcpta increments: Author: Yan Wang, Date: 2026-02-02

//! rcpta: Rust Class Pointer Analysis.
//!
//! Class-level pointer analysis built on rupta's MIR analysis.
//! Core structures: ClassPtr, ClassObj, ClassPAG.

pub mod class_cg;
pub mod class_obj;
pub mod class_pag;
pub mod class_ptr;
pub mod class_pts;

pub use class_cg::ClassCallGraph;
pub use class_obj::{AllocSite, ClassObj};
pub use class_pag::{
    AllocEdge, AssignEdge, CallArgEdge, CallRetEdge, CallSiteId, ClassPAG, FieldId, LoadEdge, StoreEdge,
};
pub use class_ptr::{ClassPtr, ClassPtrKind, Context};
pub use class_pts::{solve_class_pts, ClassPTS, ClassPTSResult, MaterializedLoadEdge, MaterializedStoreEdge};
