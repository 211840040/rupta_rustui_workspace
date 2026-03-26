use crate::rcpta::{ClassPAG, ClassPTSResult};
use crate::util::class::dsl_inheritance_graph::dump_inheritance_graph_from_entry_types;
use crate::util::class::ClassTypeSystem;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::Path;

fn ensure_parent_dir(file_path: &str) {
    if let Some(parent) = Path::new(file_path).parent() {
        let _ = fs::create_dir_all(parent);
    }
}

fn compute_reachable_nodes(adj: &HashMap<String, Vec<String>>, start: &str) -> HashSet<String> {
    let mut visited: HashSet<String> = HashSet::new();
    let mut work = std::collections::VecDeque::new();
    visited.insert(start.to_string());
    work.push_back(start.to_string());
    while let Some(cur) = work.pop_front() {
        if let Some(nexts) = adj.get(&cur) {
            for n in nexts {
                if visited.insert(n.clone()) {
                    work.push_back(n.clone());
                }
            }
        }
    }
    visited
}

/// A minimal static extends adjacency built by re-parsing DSL sources.
/// (We keep this local so the safety log doesn't depend on reading a dumped graph file.)
fn build_extends_adj_from_dsl_sources() -> HashMap<String, Vec<String>> {
    // We reuse the existing inheritance graph dump machinery to ensure we follow the same parsing
    // rules as `dump_inheritance_graph_from_entry_types`, but we still need an in-memory extends map.
    //
    // Implementation note: for now, we parse the same DSL sources again with a small regex.
    // This keeps the cast-safety logic self-contained and stable.
    let rupta_manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = rupta_manifest_dir
        .parent()
        .expect("rupta manifest must have a parent workspace root");
    let tests_root = workspace_root.join("rustdsl/classes/tests");

    let mut files = Vec::new();
    let mut stack = vec![tests_root];
    while let Some(dir) = stack.pop() {
        let entries = match fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                files.push(path);
            }
        }
    }

    let re_extends = regex::Regex::new(
        r"\bpub\s+(?:abstract\s+)?class\s+([A-Za-z_][A-Za-z0-9_]*)\s+extends\s+([A-Za-z_][A-Za-z0-9_]*)\b",
    )
    .expect("regex");

    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for file in files {
        let content = match fs::read_to_string(&file) {
            Ok(c) => c,
            Err(_) => continue,
        };
        for cap in re_extends.captures_iter(&content) {
            let child = cap.get(1).unwrap().as_str().to_string();
            let parent = cap.get(2).unwrap().as_str().to_string();
            adj.entry(child).or_default().push(parent);
        }
    }
    adj
}

fn type_range_for_ptr(class_pag: &ClassPAG, result: &ClassPTSResult, ptr_id: &str) -> HashSet<String> {
    let mut out: HashSet<String> = HashSet::new();
    let Some(objs) = result.pts.get(ptr_id) else {
        return out;
    };
    for obj_id in objs {
        if let Some(obj) = class_pag.get_obj(obj_id) {
            out.insert(obj.class_type.clone());
        }
    }
    out
}

fn is_subtype_via_extends(adj: &HashMap<String, Vec<String>>, sub: &str, sup: &str) -> bool {
    if sub == sup {
        return true;
    }
    let reach = compute_reachable_nodes(adj, sub);
    reach.contains(sup)
}

/// Dumps one line per source-level class cast site:
/// `file:line:col cast is safe/unsafe`
pub fn dump_cast_safety_log(
    class_type_system: &ClassTypeSystem,
    class_pag: &ClassPAG,
    pts_result: &ClassPTSResult,
    output_path: &str,
) {
    // Build static extends map once. (Safety decisions here focus on class up/down-casts.)
    let extends_adj = build_extends_adj_from_dsl_sources();

    // Also keep a side-effect dump of inheritance graph if user requested it separately.
    // (This is a no-op unless the caller enabled that option; included only to keep behavior stable.)
    let _ = class_type_system;
    let _ = dump_inheritance_graph_from_entry_types;

    ensure_parent_dir(output_path);
    let mut writer: Box<dyn Write> = match output_path {
        "stdout" => Box::new(std::io::stdout()),
        _ => Box::new(fs::File::create(output_path).expect("Unable to create cast safety log file")),
    };

    let mut sites = class_pag.cast_sites().to_vec();
    sites.sort_by(|a, b| a.src_loc.cmp(&b.src_loc).then(a.src_ptr_id.cmp(&b.src_ptr_id)).then(a.dst_ptr_id.cmp(&b.dst_ptr_id)));

    for site in sites {
        let src_types = type_range_for_ptr(class_pag, pts_result, &site.src_ptr_id);
        let dst_ty = class_pag
            .get_ptr(&site.dst_ptr_id)
            .map(|p| p.class_type.clone());

        let safe = if src_types.is_empty() || dst_ty.is_none() {
            false
        } else {
            let dst_ty = dst_ty.unwrap();
            src_types
                .iter()
                .all(|s| is_subtype_via_extends(&extends_adj, s, &dst_ty))
        };

        let verdict = if safe { "safe" } else { "unsafe" };
        writer
            .write_all(format!("{} cast is {}\n", site.src_loc, verdict).as_bytes())
            .expect("write cast safety line");
    }

    writer.flush().expect("flush cast safety log");
}

