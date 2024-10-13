use std::collections::HashSet;

use lantern_dependency_graph::LanternFileDependencyMap;

pub fn file_dependency_map_to_graphviz(depgraph: &LanternFileDependencyMap) -> String {
    let mut res = Vec::new();
    res.push("digraph {".to_string());
    for module_id in 0..depgraph.symbols_map.modules.len() {
        let module = depgraph.symbols_map.get_module(module_id).unwrap();
        res.push(format!(
            "  {} [label=\"{}\"]",
            module_id,
            module.file_path.display()
        ));

        for to in depgraph
            .dependency_map
            .get(&module_id)
            .unwrap_or(&HashSet::new())
        {
            res.push(format!("  {} -> {}", module_id, to));
        }
    }
    res.push("}".to_string());
    return res.join("\n");
}
