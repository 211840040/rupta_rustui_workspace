// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.

use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;
use std::time::Duration;

use itertools::Itertools;
use log::*;
use rustc_middle::ty::TyCtxt;

use super::propagator::propagator::Propagator;
use super::strategies::context_strategy::{ContextStrategy, KObjectSensitive};
use super::strategies::stack_filtering::StackFilter;
use super::PointerAnalysis;
use super::*;
use crate::graph::call_graph::CSCallGraph;
use crate::graph::func_pag::FuncPAG;
use crate::mir::analysis_context::AnalysisContext;
use crate::mir::call_site::{AssocCallGroup, CSCallSite, CallSite, CallType};
use crate::mir::context::{Context, ContextId};
use crate::mir::function::{CSFuncId, FuncId};
use crate::mir::path::{CSPath, Path, PathEnum};
use crate::rta::rta::RapidTypeAnalysis;
use crate::util::class::analysis;
use crate::util::pta_statistics::ContextSensitiveStat;
use crate::util::{self, chunked_queue, results_dumper};

pub type CallSiteSensitivePTA<'pta, 'tcx, 'compilation> =
    ContextSensitivePTA<'pta, 'tcx, 'compilation, KCallSiteSensitive>;
/// The object-sensitive pointer analysis for Rust has not been throughly evaluated so far.
pub type ObjectSensitivePTA<'pta, 'tcx, 'compilation> =
    ContextSensitivePTA<'pta, 'tcx, 'compilation, KObjectSensitive>;

pub struct ContextSensitivePTA<'pta, 'tcx, 'compilation, S: ContextStrategy> {
    /// The analysis context
    pub(crate) acx: &'pta mut AnalysisContext<'tcx, 'compilation>,
    /// Points-to data
    pub(crate) pt_data: DiffPTDataTy,
    /// Pointer Assignment Graph
    pub(crate) pag: PAG<Rc<CSPath>>,
    /// Call graph
    pub call_graph: CSCallGraph,

    /// Records the functions that have been processed
    pub(crate) processed_funcs: HashSet<CSFuncId>,

    /// Iterator for address_of edges in pag
    addr_edge_iter: chunked_queue::IterCopied<EdgeId>,

    // Inter-procedure edges created for dynamic calls, which will be iterated
    // as initial constraints in propagator
    pub(crate) inter_proc_edges_queue: chunked_queue::ChunkedQueue<EdgeId>,

    assoc_calls: AssocCallGroup<NodeId, CSFuncId, Rc<CSPath>>,

    ctx_strategy: S,

    pub stack_filter: Option<StackFilter<CSFuncId>>,
    pub pre_analysis_time: Duration,
}

impl<'pta, 'tcx, 'compilation, S: ContextStrategy> Debug
    for ContextSensitivePTA<'pta, 'tcx, 'compilation, S>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        "ContextSensitivePTA".fmt(f)
    }
}

/// Constructor
impl<'pta, 'tcx, 'compilation, S: ContextStrategy> ContextSensitivePTA<'pta, 'tcx, 'compilation, S> {
    pub fn new(acx: &'pta mut AnalysisContext<'tcx, 'compilation>, ctx_strategy: S) -> Self {
        let call_graph = CSCallGraph::new();
        let pag = PAG::new();
        let addr_edge_iter = pag.addr_edge_iter();
        ContextSensitivePTA {
            acx,
            pt_data: DiffPTDataTy::new(),
            pag,
            call_graph,
            processed_funcs: HashSet::new(),
            addr_edge_iter,
            inter_proc_edges_queue: chunked_queue::ChunkedQueue::new(),
            assoc_calls: AssocCallGroup::new(),
            ctx_strategy,
            stack_filter: None,
            pre_analysis_time: Duration::ZERO,
        }
    }

    #[inline]
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.acx.tcx
    }

    #[inline]
    pub fn get_context_id(&mut self, context: &Rc<Context<S::E>>) -> ContextId {
        self.ctx_strategy.get_context_id(context)
    }

    #[inline]
    pub fn get_context_by_id(&self, context_id: ContextId) -> Rc<Context<S::E>> {
        self.ctx_strategy.get_context_by_id(context_id)
    }
    #[inline]
    pub fn get_empty_context_id(&mut self) -> ContextId {
        self.ctx_strategy.get_empty_context_id()
    }

    /// Process statements in reachable functions.
    /// Iteratively process until no new callee is discovered (Tai-e style).
    fn process_reach_funcs(&mut self) {
        loop {
            let mut any_processed = false;
            let funcs: Vec<CSFuncId> = self.call_graph.reach_funcs_iter().collect();
            for func in funcs {
                if self.processed_funcs.contains(&func) {
                    continue;
                }
                let func_ref = self.acx.get_function_reference(func.func_id);
                info!(
                    "Processing function {:?} {}, context: {:?}",
                    func.func_id,
                    func_ref.to_string(),
                    self.get_context_by_id(func.cid),
                );
                if self.pag.build_func_pag(self.acx, func.func_id) {
                    self.add_fpag_edges(func);
                    self.process_calls_in_fpag(func);
                    any_processed = true;
                }
            }
            if !any_processed {
                break;
            }
        }
    }

    /// Adds internal edges of a function pag to the whole program's pag.
    /// The function pag for the given def_id should be built before calling this function.
    /// rcpta: we only add the class pag edges here(with context). The internal edges in the function pag will be added when processing the function body.
    pub fn add_fpag_edges(&mut self, func: CSFuncId) {
        if self.processed_funcs.contains(&func) {
            return;
        }

        let fpag = unsafe { &*(self.pag.func_pags.get(&func.func_id).unwrap() as *const FuncPAG) };
        let class_fpag = &fpag.class_fpag;
        let func_ref = self.acx.get_function_reference(func.func_id);

        for ptr_id in class_fpag.ptr_ids() {
            debug!(
                "Adding class pointer {:?} in function {:?} to PAG",
                ptr_id,
                func_ref.to_string()
            );
        }

        let ctx = self
            .ctx_strategy
            .get_dsl_ctx(&self.get_context_by_id(func.cid), self.acx);
        debug!(
            "Context for function {} is {}",
            func_ref.to_string(),
            ctx.to_string()
        );
        let func_tag = format!("{}{}", ctx, func_ref.to_string());

        for (cptr_id, cobj_id) in class_fpag.iter_alloc_edges() {
            let cptr = class_fpag.get_ptr(&cptr_id).unwrap();
            let cobj = class_fpag.get_obj(&cobj_id).unwrap();
            let cs_cptr = cptr.clone().with_context(ctx.clone());
            let cs_cobj = cobj.clone().with_context(ctx.clone());
            let cs_cptr_id = self.acx.class_pag.get_or_create_ptr(cs_cptr);
            let cs_cobj_id = self.acx.class_pag.get_or_create_obj(cs_cobj);
            self.acx.class_pag.set_ptr_func(&cs_cptr_id, func_tag.clone());
            debug!(
                "Adding class alloc edge to pag {} -> {} in function {} ",
                cs_cptr_id,
                cs_cobj_id,
                func_ref.to_string(),
            );
            self.acx.class_pag.add_alloc(cs_cptr_id, cs_cobj_id);
        }

        for (src_cptr_id, dst_cptr_id) in class_fpag.iter_assign_edges() {
            let src_cptr = class_fpag.get_ptr(&src_cptr_id).unwrap();
            let dst_cptr = class_fpag.get_ptr(&dst_cptr_id).unwrap();

            let cs_src_cptr = src_cptr.clone().with_context(ctx.clone());
            let cs_dst_cptr = dst_cptr.clone().with_context(ctx.clone());
            let cs_src_cptr_id = self.acx.class_pag.get_or_create_ptr(cs_src_cptr);
            let cs_dst_cptr_id = self.acx.class_pag.get_or_create_ptr(cs_dst_cptr);
            self.acx.class_pag.set_ptr_func(&cs_src_cptr_id, func_tag.clone());
            self.acx.class_pag.set_ptr_func(&cs_dst_cptr_id, func_tag.clone());
            debug!(
                "Adding class assign edge to pag {} -> {} in function {} ",
                cs_src_cptr_id,
                cs_dst_cptr_id,
                func_ref.to_string(),
            );
            self.acx.class_pag.add_assign(cs_src_cptr_id, cs_dst_cptr_id);
        }

        for (src_cptr_id, dst_cptr_id) in class_fpag.iter_cast_edges() {
            let src_cptr = class_fpag.get_ptr(&src_cptr_id).unwrap();
            let dst_cptr = class_fpag.get_ptr(&dst_cptr_id).unwrap();

            let cs_src_cptr = src_cptr.clone().with_context(ctx.clone());
            let cs_dst_cptr = dst_cptr.clone().with_context(ctx.clone());
            let cs_src_cptr_id = self.acx.class_pag.get_or_create_ptr(cs_src_cptr);
            let cs_dst_cptr_id = self.acx.class_pag.get_or_create_ptr(cs_dst_cptr);
            self.acx.class_pag.set_ptr_func(&cs_src_cptr_id, func_tag.clone());
            self.acx.class_pag.set_ptr_func(&cs_dst_cptr_id, func_tag.clone());
            debug!(
                "Adding class cast edge to pag {} -> {} in function {} ",
                cs_src_cptr_id,
                cs_dst_cptr_id,
                func_ref.to_string(),
            );
            self.acx.class_pag.add_cast(cs_src_cptr_id, cs_dst_cptr_id);
        }

        for load_edge in class_fpag.iter_load_edges() {
            let base_cptr = class_fpag.get_ptr(&load_edge.base_ptr_id).unwrap();
            let dst_cptr = class_fpag.get_ptr(&load_edge.dst_ptr_id).unwrap();

            let cs_base_cptr = base_cptr.clone().with_context(ctx.clone());
            let cs_dst_cptr = dst_cptr.clone().with_context(ctx.clone());
            let cs_base_cptr_id = self.acx.class_pag.get_or_create_ptr(cs_base_cptr);
            let cs_dst_cptr_id = self.acx.class_pag.get_or_create_ptr(cs_dst_cptr);
            self.acx
                .class_pag
                .set_ptr_func(&cs_base_cptr_id, func_tag.clone());
            self.acx.class_pag.set_ptr_func(&cs_dst_cptr_id, func_tag.clone());
            self.acx
                .class_pag
                .set_ptr_func(&cs_base_cptr_id, func_tag.clone());
            debug!(
                "Adding class load edge to pag {} -> {} in function {} ",
                cs_base_cptr_id,
                cs_dst_cptr_id,
                func_ref.to_string(),
            );
            self.acx.class_pag.add_load(
                cs_base_cptr_id.clone(),
                load_edge.field.clone(),
                cs_dst_cptr_id.clone(),
            );
        }

        for store_edge in class_fpag.iter_store_edges() {
            let base_cptr = class_fpag.get_ptr(&store_edge.base_ptr_id).unwrap();
            let src_cptr = class_fpag.get_ptr(&store_edge.src_ptr_id).unwrap();

            let cs_base_cptr = base_cptr.clone().with_context(ctx.clone());
            let cs_src_cptr = src_cptr.clone().with_context(ctx.clone());
            let cs_base_cptr_id = self.acx.class_pag.get_or_create_ptr(cs_base_cptr);
            let cs_src_cptr_id = self.acx.class_pag.get_or_create_ptr(cs_src_cptr);
            self.acx
                .class_pag
                .set_ptr_func(&cs_base_cptr_id, func_tag.clone());
            self.acx.class_pag.set_ptr_func(&cs_src_cptr_id, func_tag.clone());
            self.acx
                .class_pag
                .set_ptr_func(&cs_base_cptr_id, func_tag.clone());
            debug!(
                "Adding class store edge to pag {} -> {} in function {} ",
                cs_src_cptr_id,
                cs_base_cptr_id,
                func_ref.to_string(),
            );
            self.acx.class_pag.add_store(
                cs_base_cptr_id.clone(),
                store_edge.field.clone(),
                cs_src_cptr_id.clone(),
            );
        }

        for call_arg_edge in class_fpag.call_arg_edges() {
            let fml_cptr = class_fpag.get_ptr(&call_arg_edge.formal_ptr_id).unwrap();
            let act_cptr = class_fpag.get_ptr(&call_arg_edge.actual_ptr_id).unwrap();

            let ctx_t = crate::rcpta::Context::new_k_limited_context(
                &ctx,
                call_arg_edge.call_site.clone(),
                self.acx.analysis_options.context_depth as usize,
            );

            let cs_fml_cptr = fml_cptr.clone().with_context(ctx_t);
            let cs_act_cptr = act_cptr.clone().with_context(ctx.clone());
            let cs_fml_cptr_id = self.acx.class_pag.get_or_create_ptr(cs_fml_cptr);
            let cs_act_cptr_id = self.acx.class_pag.get_or_create_ptr(cs_act_cptr);
            self.acx.class_pag.set_ptr_func(&cs_act_cptr_id, func_tag.clone());
            debug!(
                "Adding class call arg edge to pag {} -> {} in function {:?} at callsite {} ",
                cs_act_cptr_id,
                cs_fml_cptr_id,
                func_ref.to_string(),
                call_arg_edge.call_site
            );
            self.acx.class_pag.add_call_arg(
                call_arg_edge.call_site.clone(),
                call_arg_edge.arg_idx,
                cs_act_cptr_id,
                cs_fml_cptr_id,
            );
        }

        for call_ret_edge in class_fpag.call_ret_edges() {
            let fml_cptr = class_fpag.get_ptr(&call_ret_edge.formal_ret_ptr_id).unwrap();
            let act_cptr = class_fpag.get_ptr(&call_ret_edge.actual_ret_ptr_id).unwrap();

            let ctx_t = crate::rcpta::Context::new_k_limited_context(
                &ctx,
                call_ret_edge.call_site.clone(),
                self.acx.analysis_options.context_depth as usize,
            );

            let cs_fml_cptr = fml_cptr.clone().with_context(ctx_t);
            let cs_act_cptr = act_cptr.clone().with_context(ctx.clone());
            let cs_fml_cptr_id = self.acx.class_pag.get_or_create_ptr(cs_fml_cptr);
            let cs_act_cptr_id = self.acx.class_pag.get_or_create_ptr(cs_act_cptr);
            self.acx.class_pag.set_ptr_func(&cs_act_cptr_id, func_tag.clone());
            debug!(
                "Adding class call ret edge to pag {} -> {} in function {:?} at callsite {} ",
                cs_fml_cptr_id,
                cs_act_cptr_id,
                func_ref.to_string(),
                call_ret_edge.call_site
            );
            self.acx
                .class_pag
                .add_call_ret(call_ret_edge.call_site.clone(), cs_fml_cptr_id, cs_act_cptr_id);
        }

        let edges_iter = fpag.internal_edges_iter();
        for (src, dst, kind) in edges_iter {
            let cs_src = self.mk_cs_path(src, func.cid);
            let cs_dst = self.mk_cs_path(dst, func.cid);
            if let Some(edge_id) = self.pag.add_edge(&cs_src, &cs_dst, kind.clone()) {
                if cs_src.path.is_promoted_constant() || cs_src.path.is_static_variable() {
                    self.inter_proc_edges_queue.push(edge_id);
                }
            }
        }

        // add edges in the promoted functions
        // We do not analyze the promoted functions context sensitively
        if let Some(promoted_funcs) = self.pag.promoted_funcs_map.get(&func.func_id) {
            let promoted_funcs = unsafe { &*(promoted_funcs as *const HashSet<FuncId>) };
            for promoted_func in promoted_funcs {
                let cs_promtoted_func = CSFuncId::new(self.get_empty_context_id(), *promoted_func);
                self.add_fpag_edges(cs_promtoted_func);
            }
        }
        // add edges in the related static functions
        // We do not analyze the static functions context sensitively
        if let Some(static_funcs) = self.pag.involved_static_funcs_map.get(&func.func_id) {
            let static_funcs = unsafe { &*(static_funcs as *const HashSet<FuncId>) };
            for static_func in static_funcs {
                let cs_static_func = CSFuncId::new(self.get_empty_context_id(), *static_func);
                self.add_fpag_edges(cs_static_func);
            }
        }

        self.processed_funcs.insert(func);
    }

    fn process_calls_in_fpag(&mut self, func: CSFuncId) {
        let fpag = unsafe { &*(self.pag.get_func_pag(&func.func_id).unwrap() as *const FuncPAG) };
        // For static dispatch callsites, the call target can be resolved directly.
        for (callsite, callee) in &fpag.static_dispatch_callsites {
            let cs_callsite = self.mk_cs_callsite(callsite, func.cid);
            self.process_new_call(&cs_callsite, callee);
            self.call_graph
                .set_callsite_type(callsite.into(), CallType::StaticDispatch);
        }

        // For special callsites, we have summary the effects. Therefore we only add call edge
        // for the callsite without adding arg --> param and ret --> dst edges.
        for (callsite, callee) in &fpag.special_callsites {
            let cs_callsite = self.mk_cs_callsite(callsite, func.cid);
            // Do not add contexts for the special callees
            let empty_cid = self.special_callsite_context(&cs_callsite, callee);
            let cs_callee = self.mk_cs_func(*callee, empty_cid);
            self.call_graph.add_edge(cs_callsite.into(), func, cs_callee);
            // This may classify some special dynamic calls into static calls
            self.call_graph
                .set_callsite_type(callsite.into(), CallType::StaticDispatch);
        }

        // For std::ops::call, dynamic and fnptr callsites, add them to the dynamic_calls and fnptr_calls maps.
        for (dyn_fn_obj, callsite) in &fpag.dynamic_fntrait_callsites {
            let cs_dyn_fn_obj = self.mk_cs_path(dyn_fn_obj, func.cid);
            let cs_callsite = self.mk_cs_callsite(callsite, func.cid);
            let dyn_node_id = self.dyn_node_id(&cs_dyn_fn_obj);
            self.assoc_calls
                .add_dynamic_fntrait_call(dyn_node_id, cs_callsite);
            self.call_graph
                .set_callsite_type(callsite.into(), CallType::DynamicFnTrait);
        }
        for (dyn_var, callsite) in &fpag.dynamic_dispatch_callsites {
            let cs_dyn_var = self.mk_cs_path(dyn_var, func.cid);
            let cs_callsite = self.mk_cs_callsite(callsite, func.cid);
            let dyn_node_id = self.dyn_node_id(&cs_dyn_var);
            self.assoc_calls
                .add_dynamic_dispatch_call(dyn_node_id, cs_callsite);
            self.call_graph
                .set_callsite_type(callsite.into(), CallType::DynamicDispatch);
        }
        for (fn_ptr, callsite) in &fpag.fnptr_callsites {
            let cs_fn_ptr = self.mk_cs_path(fn_ptr, func.cid);
            let cs_callsite = self.mk_cs_callsite(callsite, func.cid);
            self.assoc_calls
                .add_fnptr_call(self.pag.get_or_insert_node(&cs_fn_ptr), cs_callsite);
            self.call_graph
                .set_callsite_type(callsite.into(), CallType::FnPtr);
        }
    }

    fn dyn_node_id(&mut self, dyn_obj: &Rc<CSPath>) -> NodeId {
        self.pag.get_or_insert_node(dyn_obj)
    }

    /// Process a resolved call according to the call type
    fn process_new_call(&mut self, callsite: &Rc<CSCallSite>, callee: &FuncId) {
        let callee_def_id = self.acx.get_function_reference(*callee).def_id;
        // an instance call
        if util::has_self_parameter(self.tcx(), callee_def_id) {
            // borrow self (&self or &mut self)
            if util::has_self_ref_parameter(self.tcx(), callee_def_id) {
                // the instance should be the pointed-to object of the self pointer
                let callee_cid = if analysis::is_ctx_should_be_compressed(callsite, callee, self.acx) {
                    callsite.func.cid
                } else {
                    self.ctx_strategy
                        .new_instance_call_context(callsite, None)
                        .unwrap_or_else(|| {
                            // rcpta: ensure callee is always in call graph so its body is built (Load/Cast edges).
                            // When strategy returns None (e.g. object-sensitive with no receiver), use static context.
                            self.ctx_strategy.new_static_call_context(callsite)
                        })
                };
                let cs_callee = CSFuncId::new(callee_cid, *callee);
                self.add_call_edge(callsite, &cs_callee);
                let self_ref: &Rc<CSPath> = callsite.args.get(0).expect("invalid arguments");
                let self_ref_id = self.pag.get_or_insert_node(self_ref);
                self.assoc_calls
                    .add_static_dispatch_instance_call(self_ref_id, callsite.clone(), *callee);
            } else {
                // move self
                let instance = callsite.args.get(0).expect("invalid arguments");
                let callee_cid = if analysis::is_ctx_should_be_compressed(callsite, callee, self.acx) {
                    callsite.func.cid
                } else {
                    self.ctx_strategy
                        .new_instance_call_context(callsite, Some(instance))
                        .unwrap_or_else(|| self.ctx_strategy.new_static_call_context(callsite))
                };
                let cs_callee = CSFuncId::new(callee_cid, *callee);
                self.add_call_edge(callsite, &cs_callee);
            }
        } else {
            let callee_cid = if analysis::is_ctx_should_be_compressed(callsite, callee, self.acx) {
                callsite.func.cid
            } else {
                self.ctx_strategy.new_static_call_context(callsite)
            };
            let cs_callee = CSFuncId::new(callee_cid, *callee);
            self.add_call_edge(callsite, &cs_callee);
        }
    }

    fn special_callsite_context(&mut self, callsite: &Rc<CSCallSite>, _callee: &FuncId) -> ContextId {
        // Currently we treat all special callsites as statical callsites
        self.ctx_strategy.new_static_call_context(callsite)
    }

    // Add new call edges to pag
    fn process_new_calls(&mut self, new_calls: &Vec<(Rc<CSCallSite>, FuncId)>) {
        for (callsite, callee_id) in new_calls {
            self.process_new_call(callsite, callee_id);
        }
        self.process_reach_funcs();
    }

    fn process_new_call_instances(&mut self, new_call_instances: &Vec<(Rc<CSCallSite>, Rc<CSPath>, FuncId)>) {
        for (callsite, instance, callee_id) in new_call_instances {
            if let Some(callee_cid) = if analysis::is_ctx_should_be_compressed(callsite, callee_id, self.acx)
            {
                Some(callsite.func.cid)
            } else {
                self.ctx_strategy
                    .new_instance_call_context(callsite, Some(instance))
            } {
                let cs_callee = CSFuncId::new(callee_cid, *callee_id);
                self.add_call_edge(callsite, &cs_callee);
            }
        }
        self.process_reach_funcs();
    }

    fn add_call_edge(&mut self, callsite: &Rc<CSCallSite>, callee: &CSFuncId) {
        let caller = callsite.func;
        if !self.call_graph.add_edge(callsite.into(), caller, *callee) {
            return;
        }
        let new_inter_proc_edges = self.pag.add_inter_procedural_edges(self.acx, callsite, *callee);
        for edge in new_inter_proc_edges {
            self.inter_proc_edges_queue.push(edge);
            self.add_page_edge_func(edge, callsite.func);
        }
    }

    fn mk_cs_path(&mut self, path: &Rc<Path>, cid: ContextId) -> Rc<CSPath> {
        match path.value() {
            PathEnum::Parameter { .. }
            | PathEnum::LocalVariable { .. }
            | PathEnum::ReturnValue { .. }
            | PathEnum::Auxiliary { .. }
            | PathEnum::QualifiedPath { .. }
            | PathEnum::OffsetPath { .. } => CSPath::new_cs_path(cid, path.clone()),
            PathEnum::HeapObj { .. } => {
                // Directly use the context of the method for the heap objects
                CSPath::new_cs_path(cid, path.clone())
            }
            PathEnum::Constant
            | PathEnum::StaticVariable { .. }
            | PathEnum::PromotedConstant { .. }
            | PathEnum::Function(..)
            | PathEnum::PromotedStrRefArray
            | PathEnum::PromotedArgumentV1Array
            | PathEnum::Type(..) => {
                // Context insensitive for these kinds of path
                let empty_cid = self.get_empty_context_id();
                CSPath::new_cs_path(empty_cid, path.clone())
            }
        }
    }

    fn mk_cs_func(&mut self, func_id: FuncId, cid: ContextId) -> CSFuncId {
        CSFuncId { cid, func_id }
    }

    fn mk_cs_callsite(&mut self, callsite: &Rc<CallSite>, cid: ContextId) -> Rc<CSCallSite> {
        Rc::new(CSCallSite::new(
            CSFuncId {
                cid,
                func_id: callsite.func,
            },
            callsite.location,
            callsite
                .args
                .iter()
                .map(|arg| self.mk_cs_path(arg, cid))
                .collect_vec(),
            self.mk_cs_path(&callsite.destination, cid),
        ))
    }

    fn add_page_edge_func(&mut self, edge: EdgeId, func: CSFuncId) {
        if let Some(sf) = &mut self.stack_filter {
            sf.add_pag_edge_in_func(edge, func);
        }
    }

    #[inline]
    pub fn get_pt_data(&self) -> &DiffPTDataTy {
        &self.pt_data
    }
}

impl<'pta, 'tcx, 'compilation, S: ContextStrategy> PointerAnalysis<'tcx, 'compilation>
    for ContextSensitivePTA<'pta, 'tcx, 'compilation, S>
{
    fn pre_analysis(&mut self) {
        if !self.acx.analysis_options.stack_filtering {
            return;
        }
        info!("Start pre-analysis");
        let mut rta = RapidTypeAnalysis::new(&mut self.acx);
        rta.analyze();
        self.pre_analysis_time += rta.analysis_time;
        self.stack_filter = Some(StackFilter::new(rta.call_graph));
        self.ctx_strategy
            .with_stack_filter(self.stack_filter.as_mut().unwrap());
        self.pre_analysis_time += self.stack_filter.as_ref().unwrap().fra_time();
        println!(
            "Pre-analysis time {}",
            humantime::format_duration(self.pre_analysis_time).to_string()
        );
    }

    /// Initialize the analysis.
    fn initialize(&mut self) {
        // add the entry point to the call graph
        let entry_point = self.acx.entry_point;
        let empty_context_id = self.get_empty_context_id();
        let entry_func_id = self.acx.get_func_id(entry_point, self.tcx().mk_args(&[]));
        self.call_graph
            .add_node(CSFuncId::new(empty_context_id, entry_func_id));

        // process statements of reachable functions
        self.process_reach_funcs();

        // rcpta: ensure every callee in static_dispatch_callsites has its PAG (and ClassPAG edges) built,
        // in case any were not added to reach_funcs (e.g. context strategy or order).
        self.pag.build_all_callee_pags(self.acx);
        self.acx.flush_pending_class_cg_edges();
    }

    /// Solve the worklist problem using Propagator.
    fn propagate(&mut self) {
        let mut iter_proc_edge_iter = self.inter_proc_edges_queue.iter_copied();
        // Solve until no new call relationship is found.
        loop {
            let mut new_calls: Vec<(Rc<CSCallSite>, FuncId)> = Vec::new();
            let mut new_call_instances: Vec<(Rc<CSCallSite>, Rc<CSPath>, FuncId)> = Vec::new();
            let mut propagator = Propagator::new(
                self.acx,
                &mut self.pt_data,
                &mut self.pag,
                &mut new_calls,
                &mut new_call_instances,
                &mut self.addr_edge_iter,
                &mut iter_proc_edge_iter,
                &mut self.assoc_calls,
                self.stack_filter.as_mut(),
            );
            propagator.solve_worklist();

            if new_calls.is_empty() && new_call_instances.is_empty() {
                break;
            } else {
                self.process_new_calls(&new_calls);
                self.process_new_call_instances(&new_call_instances);
            }
        }
    }

    /// Finalize the analysis.
    fn finalize(&self) {
        // dump call graph, points-to results
        results_dumper::dump_results(self.acx, &self.call_graph, &self.pt_data, &self.pag);

        // dump pta statistics
        let pta_stat = ContextSensitiveStat::new(self);
        pta_stat.dump_stats();
    }
}
