use std::{collections::HashSet, path::PathBuf};

use color_eyre::eyre::Result;

use lantern_dependency_graph::LanternFileDependencyMap;

pub fn run(entry_points: &Vec<PathBuf>) -> Result<()> {
    let ln_map = lantern_symbols_map::build_symbols_map(entry_points)?;
    let mut depgraph = LanternFileDependencyMap::new(ln_map);
    depgraph.build_dependency_graph();

    let mut visited: Vec<u8> = vec![0; depgraph.symbols_map.modules.len()];
    let mut cycles: Vec<Vec<String>> = Vec::new();

    fn dfs(
        dep_map: &LanternFileDependencyMap,
        visited: &mut Vec<u8>,
        cycles: &mut Vec<Vec<String>>,
        path: &mut Vec<String>,
        module_id: usize,
    ) {
        if visited[module_id] == 2 {
            return;
        }

        let module = dep_map.symbols_map.get_module(module_id).unwrap();
        let module_path = module.file_path.to_str().unwrap().to_owned();
        path.push(module_path.clone());

        if visited[module_id] == 1 {
            let cycle_index = path.iter().position(|el| *el == module_path).unwrap();
            println!("Cycle: {:#?}", &path[cycle_index..]);
            // cycles.push(path.clone());
            path.pop();
            return;
        }

        visited[module_id] = 1;

        for to in dep_map
            .dependency_map
            .get(&module_id)
            .unwrap_or(&HashSet::new())
        {
            dfs(dep_map, visited, cycles, path, *to);
        }

        visited[module_id] = 2;
        path.pop();
    }

    for module_id in 0..depgraph.symbols_map.modules.len() {
        if visited[module_id] == 2 {
            continue;
        }

        let mut path = Vec::new();
        dfs(&depgraph, &mut visited, &mut cycles, &mut path, module_id);
    }

    return Ok(());
}
