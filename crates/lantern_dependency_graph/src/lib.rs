use std::{
    collections::{HashMap, HashSet, VecDeque},
    path::PathBuf,
};

use color_eyre::eyre::Result;

use lantern_symbols_map::symbol::LNSymbolData;
use lantern_symbols_map::symbols_map::LNSymbolsMap;

#[derive(Debug)]
pub struct LanternFileDependencyMap {
    pub symbols_map: LNSymbolsMap,
    pub dependency_map: HashMap<usize, HashSet<usize>>,
    pub inverse_dependency_map: HashMap<usize, HashSet<usize>>,
}

//
//
// File level dependency map.
//
//
impl LanternFileDependencyMap {
    pub fn new(symbols_map: LNSymbolsMap) -> Self {
        Self {
            symbols_map,
            dependency_map: HashMap::new(),
            inverse_dependency_map: HashMap::new(),
        }
    }

    pub fn build_dependency_graph(&mut self) {
        let mut dependency_map: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut inverse_dependency_map: HashMap<usize, HashSet<usize>> = HashMap::new();
        for module_id in 0..self.symbols_map.modules.len() {
            let module = self.symbols_map.get_module(module_id).unwrap();
            for symbol_id in &module.symbols {
                let symbol = &self.symbols_map.symbols[*symbol_id];

                match &symbol.symbol {
                    LNSymbolData::ExportAll(file_ref) => {
                        self.add_dependency(
                            &mut dependency_map,
                            &mut inverse_dependency_map,
                            module_id,
                            file_ref.module_id,
                        );
                    }
                    LNSymbolData::ExportNamed(_, _, _, Some(file_ref)) => {
                        self.add_dependency(
                            &mut dependency_map,
                            &mut inverse_dependency_map,
                            module_id,
                            file_ref.module_id,
                        );
                    }
                    LNSymbolData::ImportDefault(_, _, file_ref, _)
                    | LNSymbolData::ImportStar(_, _, file_ref, _)
                    | LNSymbolData::ImportNamed(_, _, _, file_ref, _) => {
                        self.add_dependency(
                            &mut dependency_map,
                            &mut inverse_dependency_map,
                            module_id,
                            file_ref.module_id,
                        );
                    }
                    _ => {}
                }
            }
        }
        self.dependency_map = dependency_map;
        self.inverse_dependency_map = inverse_dependency_map;
    }

    pub fn add_dependency(
        &self,
        dependency_map: &mut HashMap<usize, HashSet<usize>>,
        inverse_dependency_map: &mut HashMap<usize, HashSet<usize>>,
        from: usize,
        to: usize,
    ) {
        if let Some(dependencies) = dependency_map.get_mut(&from) {
            dependencies.insert(to);
        } else {
            let mut dependencies = HashSet::new();
            dependencies.insert(to);
            dependency_map.insert(from, dependencies);
        }

        if let Some(dependencies) = inverse_dependency_map.get_mut(&to) {
            dependencies.insert(from);
        } else {
            let mut dependencies = HashSet::new();
            dependencies.insert(from);
            inverse_dependency_map.insert(to, dependencies);
        }
    }

    pub fn graphviz(&self) -> String {
        let mut res = Vec::new();
        res.push("digraph {".to_string());
        for module_id in 0..self.symbols_map.modules.len() {
            let module = self.symbols_map.get_module(module_id).unwrap();
            res.push(format!(
                "  {} [label=\"{}\"]",
                module_id,
                module.file_path.display()
            ));

            for to in self
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

    pub fn get_affected(
        &self,
        changed_file_path: &PathBuf,
        entries_only: bool,
    ) -> Result<Vec<PathBuf>> {
        let mut affected = HashSet::new();
        let mut queue = VecDeque::from([changed_file_path.clone()]);
        while queue.len() > 0 {
            let cur = queue.pop_front().unwrap();
            let maybe_module_id = self.symbols_map.get_module_id(cur.to_str().unwrap());
            if let Some(module_id) = maybe_module_id {
                for from in self
                    .inverse_dependency_map
                    .get(&module_id)
                    .unwrap_or(&HashSet::new())
                {
                    let module = self.symbols_map.get_module(*from).unwrap();
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
}
