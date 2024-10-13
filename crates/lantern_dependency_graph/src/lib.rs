use std::collections::{HashMap, HashSet};

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
}
