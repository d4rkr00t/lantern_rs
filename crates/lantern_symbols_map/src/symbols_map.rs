use std::{collections::HashMap, path::PathBuf};

use color_eyre::eyre::Result;

use oxc_span::Span;

use lantern_resolver::LanternResolver;

use crate::{symbol::LNSymbol, LNModule};

#[derive(Debug)]
pub struct LNSymbolsMap {
    pub modules: Vec<LNModule>,
    pub symbols: Vec<LNSymbol>,
    path_to_module_id: HashMap<String, usize>,
    sources: HashMap<usize, String>,
    resolver: LanternResolver,
}

impl LNSymbolsMap {
    pub fn new(resolver: LanternResolver) -> Self {
        Self {
            modules: Vec::new(),
            symbols: Vec::new(),
            path_to_module_id: HashMap::new(),
            sources: HashMap::new(),
            resolver,
        }
    }

    pub fn add_module(&mut self, module: LNModule) -> Option<usize> {
        if let Some(ext) = module.file_path.extension() {
            if ext == "json" {
                return None;
            }
        }
        if self.has_module(module.file_path.to_str().unwrap()) {
            return Some(self.path_to_module_id[module.file_path.to_str().unwrap()]);
        }

        let id = self.modules.len();
        self.modules.push(module);
        self.path_to_module_id
            .insert(self.modules[id].file_path.to_str().unwrap().to_string(), id);
        return Some(id);
    }

    pub fn get_module(&self, id: usize) -> Option<&LNModule> {
        return self.modules.get(id);
    }

    pub fn has_module(&self, path: &str) -> bool {
        return self.path_to_module_id.contains_key(path);
    }

    pub fn get_module_id(&self, path: &str) -> Option<usize> {
        return self.path_to_module_id.get(path).copied();
    }

    pub fn get_module_source(&mut self, module_id: usize) -> &str {
        if self.sources.contains_key(&module_id) {
            &self.sources[&module_id]
        } else {
            let source = std::fs::read_to_string(&self.modules[module_id].file_path).unwrap();
            self.sources.insert(module_id, source.clone());
            &self.sources[&module_id]
        }
    }

    pub fn read_span_from_module(&mut self, module_id: usize, span: &Span) -> &str {
        let source = self.get_module_source(module_id);
        return source[span.start as usize..span.end as usize].as_ref();
    }

    pub fn add_symbol(&mut self, module_id: usize, symbol: LNSymbol) -> usize {
        let id = self.symbols.len();
        self.symbols.push(symbol);
        self.modules[module_id].symbols.push(id);
        return id;
    }

    pub fn resolve(&self, parent_path: &PathBuf, path: String) -> Result<PathBuf> {
        return self.resolver.resolve(parent_path, &path);
    }

    pub fn get_module_path(&self, module_id: usize) -> &PathBuf {
        return &self.modules[module_id].file_path;
    }

    pub fn get_line_number_from_span(&mut self, module_id: usize, span: &Span) -> usize {
        if span.start == 0 {
            return 1;
        }
        let source = self.get_module_source(module_id);
        let source = source[0..span.start as usize].to_string();
        return source.lines().count();
    }
}
