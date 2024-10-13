use color_eyre::eyre::Result;

use lantern_symbols_map::symbol::LNSymbol;
use lantern_symbols_map::symbol::LNSymbolData;
use lantern_symbols_map::symbols_map::LNSymbolsMap;

pub fn find_unused_exports(ln_map: &LNSymbolsMap) -> Result<Vec<LNSymbol>> {
    let mut exports: Vec<LNSymbol> = Vec::new();

    for symbol in &ln_map.symbols {
        match &symbol.symbol {
            LNSymbolData::ExportAll(_)
            | LNSymbolData::ExportClassDecl(_, _)
            | LNSymbolData::ExportDecl(_, _)
            | LNSymbolData::ExportDefaultClassDecl(_, _)
            | LNSymbolData::ExportDefaultExpr(_)
            | LNSymbolData::ExportDefaultFnDecl(_, _)
            | LNSymbolData::ExportDefaultIdentifier(_, _)
            | LNSymbolData::ExportDefaultCallExpression(_, _)
            | LNSymbolData::ExportDefaultInterfaceDecl(_, _)
            | LNSymbolData::ExportEnumDecl(_, _)
            | LNSymbolData::ExportFnDecl(_, _)
            | LNSymbolData::ExportInterfaceDecl(_, _)
            | LNSymbolData::ExportNamed(_, _, _, _)
            | LNSymbolData::ExportTypeAliasDecl(_, _) => {
                exports.push(symbol.clone());
            }
            LNSymbolData::ImportStar(_, _, _, _)
            | LNSymbolData::ExportDefaultConditionalExpression(_, _, _)
            | LNSymbolData::ImportNamed(_, _, _, _, _)
            | LNSymbolData::ImportDefault(_, _, _, _) => {}
        }
    }

    for symbol in &ln_map.symbols {
        match &symbol.symbol {
            LNSymbolData::ImportDefault(_, _, file_ref, _) => {
                let idx = exports.iter().position(|x| {
                    if x.module_id != file_ref.module_id {
                        return false;
                    }

                    match &x.symbol {
                        LNSymbolData::ExportDefaultClassDecl(_, _)
                        | LNSymbolData::ExportDefaultExpr(_)
                        | LNSymbolData::ExportDefaultFnDecl(_, _)
                        | LNSymbolData::ExportDefaultIdentifier(_, _)
                        | LNSymbolData::ExportDefaultCallExpression(_, _)
                        | LNSymbolData::ExportDefaultConditionalExpression(_, _, _)
                        | LNSymbolData::ExportDefaultInterfaceDecl(_, _) => {
                            return true;
                        }
                        _ => return false,
                    }
                });

                if let Some(idx) = idx {
                    exports.remove(idx);
                }
            }
            LNSymbolData::ImportNamed(_, o_name, _, file_ref, _) => {
                let idx = exports.iter().position(|x| {
                    if x.module_id != file_ref.module_id {
                        return false;
                    }

                    match &x.symbol {
                        LNSymbolData::ExportAll(_) => {
                            return true;
                        }
                        _ => {}
                    }

                    if let Some(e_name) = x.get_name() {
                        return e_name == o_name;
                    }

                    return false;
                });

                if let Some(idx) = idx {
                    exports.remove(idx);
                }
            }
            LNSymbolData::ExportNamed(i_name, _, _, Some(file_ref)) => {
                let idx = exports.iter().position(|x| {
                    if x.module_id != file_ref.module_id {
                        return false;
                    }

                    if let Some(e_name) = x.get_name() {
                        return e_name == i_name;
                    }

                    if let LNSymbolData::ExportDefaultExpr(_) = x.symbol {
                        return i_name == "default";
                    }

                    return false;
                });

                if let Some(idx) = idx {
                    exports.remove(idx);
                }
            }
            LNSymbolData::ExportAll(file_ref) => {
                exports = exports
                    .iter()
                    .filter(|x| {
                        return x.module_id != file_ref.module_id;
                    })
                    .cloned()
                    .collect();
            }
            LNSymbolData::ImportStar(_, _, file_ref, _) => {
                exports = exports
                    .iter()
                    .filter(|x| {
                        return x.module_id != file_ref.module_id;
                    })
                    .cloned()
                    .collect();
            }
            _ => {}
        }
    }

    let exports = exports
        .iter()
        .filter(|x| {
            return ln_map.get_module(x.module_id).is_some_and(|m| {
                return !m.is_entry && !m.file_path.to_str().unwrap().contains("node_modules");
            });
        })
        .cloned()
        .collect::<Vec<LNSymbol>>();

    return Ok(exports);
}
