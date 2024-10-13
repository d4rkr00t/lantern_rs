use std::{collections::HashMap, path::PathBuf};

use color_eyre::eyre::Result;

use lantern_code_annotation::CodeAnnotation;

use lantern_symbols_map::symbol::LNSymbol;
use lantern_symbols_map::symbol::LNSymbolData;
use lantern_symbols_map::symbols_map::LNSymbolsMap;

pub fn run(entry_points: &Vec<PathBuf>) -> Result<()> {
    let mut ln_map = lantern_symbols_map::build_symbols_map(&entry_points)?;
    let mut annotations: HashMap<usize, CodeAnnotation> = HashMap::new();
    let re_exports = find_files_with_reexports(&ln_map)?;
    let total = re_exports.len();

    for symbol in re_exports {
        let span = symbol.get_span();
        let span_line = ln_map.get_line_number_from_span(symbol.module_id, span);
        if !annotations.contains_key(&symbol.module_id) {
            annotations.insert(
                symbol.module_id,
                CodeAnnotation::new(
                    ln_map.get_module_path(symbol.module_id).clone(),
                    ln_map.get_module_source(symbol.module_id).to_string(),
                ),
            );
        }
        let annotation = annotations.get_mut(&symbol.module_id).unwrap();
        let annotation_message = if let Some(symbol_name) = symbol.get_name() {
            format!("re-export: {}", symbol_name)
        } else {
            "re-export".to_owned()
        };

        annotation.annotate(annotation_message, span_line, span.clone());
    }

    for (_, value) in &annotations {
        println!("{}", value.print());
    }

    println!("Total re-exports found: {}", total);

    return Ok(());
}

fn find_files_with_reexports(ln_map: &LNSymbolsMap) -> Result<Vec<LNSymbol>> {
    let mut re_exports: Vec<LNSymbol> = Vec::new();

    for module in &ln_map.modules {
        for symbol_id in &module.symbols {
            let symbol = &ln_map.symbols[*symbol_id];
            match symbol.symbol {
                LNSymbolData::ExportAll(_) => {
                    re_exports.push(symbol.clone());
                }
                LNSymbolData::ExportNamed(_, _, _, Some(_)) => {
                    re_exports.push(symbol.clone());
                }
                _ => {}
            }
        }
    }

    let re_exports = re_exports
        .iter()
        .filter(|x| {
            return ln_map.get_module(x.module_id).is_some_and(|m| {
                return !m.is_entry && !m.file_path.to_str().unwrap().contains("node_modules");
            });
        })
        .cloned()
        .collect::<Vec<LNSymbol>>();

    return Ok(re_exports);
}
