// Copyright (c) 2024 <Wei Li>.
//
// This source code is licensed under the GNU license found in the
// LICENSE file in the root directory of this source tree.

//! Class Call Graph for DSL Classes
//! 
//! This module provides a simplified call graph that only tracks class method calls,
//! filtering out DSL internal implementation details (like vtable access, RcDyn operations, etc.)

use std::collections::{HashMap, HashSet};
use std::fmt;
use log::*;

/// Represents a class method in the call graph
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassMethodId {
    /// Class name (e.g., "Point", "Container")
    pub class_name: String,
    /// Method name (e.g., "distance_squared", "get_point_sum")
    pub method_name: String,
}

impl ClassMethodId {
    pub fn new(class_name: String, method_name: String) -> Self {
        Self {
            class_name,
            method_name,
        }
    }

    /// Format as "ClassName::method_name"
    pub fn to_string(&self) -> String {
        format!("{}::{}", self.class_name, self.method_name)
    }
}

impl fmt::Display for ClassMethodId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.class_name, self.method_name)
    }
}

/// A simplified call graph that only tracks class method calls
#[derive(Debug, Default)]
pub struct ClassCallGraph {
    /// Call edges: caller -> set of callees
    /// Key: caller method, Value: set of callee methods
    edges: HashMap<ClassMethodId, HashSet<ClassMethodId>>,
    
    /// All methods that appear in the call graph (as callers or callees)
    methods: HashSet<ClassMethodId>,
}

impl ClassCallGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a call edge from caller to callee
    /// 
    /// # Arguments
    /// * `caller_class` - The class name of the caller method
    /// * `caller_method` - The method name of the caller method
    /// * `callee_class` - The class name of the callee method
    /// * `callee_method` - The method name of the callee method
    pub fn add_call_edge(
        &mut self,
        caller_class: &str,
        caller_method: &str,
        callee_class: &str,
        callee_method: &str,
    ) {
        let caller = ClassMethodId::new(caller_class.to_string(), caller_method.to_string());
        let callee = ClassMethodId::new(callee_class.to_string(), callee_method.to_string());
        
        self.methods.insert(caller.clone());
        self.methods.insert(callee.clone());
        
        self.edges.entry(caller).or_insert_with(HashSet::new).insert(callee);
        
        debug!("ClassCallGraph: Added edge {} -> {}", 
               ClassMethodId::new(caller_class.to_string(), caller_method.to_string()),
               ClassMethodId::new(callee_class.to_string(), callee_method.to_string()));
    }

    /// Get all callees for a given caller method
    pub fn get_callees(&self, caller_class: &str, caller_method: &str) -> Vec<&ClassMethodId> {
        let caller = ClassMethodId::new(caller_class.to_string(), caller_method.to_string());
        self.edges.get(&caller)
            .map(|callees| callees.iter().collect())
            .unwrap_or_default()
    }

    /// Get all methods in the call graph
    pub fn get_all_methods(&self) -> Vec<&ClassMethodId> {
        self.methods.iter().collect()
    }

    /// Get all call edges
    pub fn get_all_edges(&self) -> Vec<(&ClassMethodId, &ClassMethodId)> {
        let mut result = Vec::new();
        for (caller, callees) in &self.edges {
            for callee in callees {
                result.push((caller, callee));
            }
        }
        result
    }

    /// Dump the call graph to a string in DOT format
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph ClassCallGraph {\n");
        dot.push_str("  rankdir=LR;\n");
        dot.push_str("  node [shape=box];\n\n");
        
        // Add all nodes
        for method in &self.methods {
            let label = format!("{}\\n{}", method.class_name, method.method_name);
            dot.push_str(&format!("  \"{}\" [label=\"{}\"];\n", method.to_string(), label));
        }
        
        dot.push_str("\n");
        
        // Add all edges
        for (caller, callees) in &self.edges {
            for callee in callees {
                dot.push_str(&format!("  \"{}\" -> \"{}\";\n", caller.to_string(), callee.to_string()));
            }
        }
        
        dot.push_str("}\n");
        dot
    }

    /// Dump the call graph to a string in text format
    pub fn to_text(&self) -> String {
        let mut text = String::from("Class Call Graph:\n");
        text.push_str("================\n\n");
        
        if self.edges.is_empty() {
            text.push_str("(No class method calls found)\n");
            return text;
        }
        
        // Group by caller class
        let mut by_class: HashMap<String, Vec<(&ClassMethodId, &HashSet<ClassMethodId>)>> = HashMap::new();
        for (caller, callees) in &self.edges {
            by_class.entry(caller.class_name.clone())
                .or_insert_with(Vec::new)
                .push((caller, callees));
        }
        
        let mut classes: Vec<_> = by_class.keys().collect();
        classes.sort();
        
        for class_name in classes {
            text.push_str(&format!("Class: {}\n", class_name));
            let methods = &by_class[class_name];
            let mut sorted_methods: Vec<_> = methods.iter().collect();
            sorted_methods.sort_by_key(|(m, _)| &m.method_name);
            
            for (caller, callees) in sorted_methods {
                text.push_str(&format!("  {}::{}\n", caller.class_name, caller.method_name));
                let mut sorted_callees: Vec<_> = callees.iter().collect();
                sorted_callees.sort();
                for callee in sorted_callees {
                    text.push_str(&format!("    -> {}::{}\n", callee.class_name, callee.method_name));
                }
            }
            text.push_str("\n");
        }
        
        text
    }

    /// Get statistics about the call graph
    pub fn stats(&self) -> ClassCallGraphStats {
        ClassCallGraphStats {
            num_methods: self.methods.len(),
            num_edges: self.edges.values().map(|callees| callees.len()).sum(),
            num_callers: self.edges.len(),
        }
    }
}

/// Statistics about the class call graph
#[derive(Debug)]
pub struct ClassCallGraphStats {
    pub num_methods: usize,
    pub num_edges: usize,
    pub num_callers: usize,
}

impl fmt::Display for ClassCallGraphStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ClassCallGraph Stats: {} methods, {} callers, {} edges",
               self.num_methods, self.num_callers, self.num_edges)
    }
}
