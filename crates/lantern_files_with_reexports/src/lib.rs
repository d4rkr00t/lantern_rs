use color_eyre::eyre::Result;

use lantern_symbols_map::symbol::LNSymbol;
use lantern_symbols_map::symbol::LNSymbolData;
use lantern_symbols_map::symbols_map::LNSymbolsMap;

pub fn find_files_with_reexports(ln_map: &LNSymbolsMap) -> Result<Vec<LNSymbol>> {
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
