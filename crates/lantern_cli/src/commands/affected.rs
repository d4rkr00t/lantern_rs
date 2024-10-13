use std::{
    collections::{HashSet, VecDeque},
    path::PathBuf,
};

use color_eyre::eyre::Result;

use lantern_dependency_graph::LanternFileDependencyMap;

pub fn analyze(entry_points: &Vec<PathBuf>, changed: &Vec<PathBuf>) -> Result<()> {
    let ln_map = lantern_symbols_map::build_symbols_map(entry_points)?;
    let mut depgraph = LanternFileDependencyMap::new(ln_map);
    depgraph.build_dependency_graph();

    let mut all_affected = Vec::new();

    for changed_file_path in changed {
        let affected = get_affected(&depgraph, &changed_file_path.canonicalize()?, true)?;
        all_affected.extend(affected);
    }

    println!("Affected: {:?}", all_affected);
    return Ok(());
}

fn get_affected(
    depgraph: &LanternFileDependencyMap,
    changed_file_path: &PathBuf,
    entries_only: bool,
) -> Result<Vec<PathBuf>> {
    let mut affected = HashSet::new();
    let mut queue = VecDeque::from([changed_file_path.clone()]);
    while queue.len() > 0 {
        let cur = queue.pop_front().unwrap();
        let maybe_module_id = depgraph.symbols_map.get_module_id(cur.to_str().unwrap());
        if let Some(module_id) = maybe_module_id {
            for from in depgraph
                .inverse_dependency_map
                .get(&module_id)
                .unwrap_or(&HashSet::new())
            {
                let module = depgraph.symbols_map.get_module(*from).unwrap();
                if entries_only {
                    if module.is_entry {
                        affected.insert(module.file_path.clone());
                    }
                } else {
                    affected.insert(module.file_path.clone());
                }
                queue.push_back(module.file_path.clone());
            }
        }
    }
    return Ok(affected.into_iter().collect());
}
