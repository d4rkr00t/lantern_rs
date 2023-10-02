use std::collections::{HashMap, HashSet};

use lantern_symbols_map::{TSSymbolData, TSSymbolsMap};

#[derive(Debug)]
pub struct LanternFileDependencyMap {
    pub symbols_map: TSSymbolsMap,
    pub dependency_map: HashMap<usize, HashSet<usize>>,
    pub inverse_dependency_map: HashMap<usize, HashSet<usize>>,
}

impl LanternFileDependencyMap {
    pub fn new(symbols_map: TSSymbolsMap) -> Self {
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
                    TSSymbolData::ExportAll(file_ref) => {
                        self.add_dependency(
                            &mut dependency_map,
                            &mut inverse_dependency_map,
                            module_id,
                            file_ref.module_id,
                        );
                    }
                    TSSymbolData::ExportNamed(_, _, _, Some(file_ref)) => {
                        self.add_dependency(
                            &mut dependency_map,
                            &mut inverse_dependency_map,
                            module_id,
                            file_ref.module_id,
                        );
                    }
                    TSSymbolData::ImportDefault(_, _, file_ref, _)
                    | TSSymbolData::ImportStar(_, _, file_ref, _)
                    | TSSymbolData::ImportNamed(_, _, file_ref, _) => {
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
}
