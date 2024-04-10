use std::{collections::HashMap, path::PathBuf};

use color_eyre::eyre::Result;

use lantern_code_annotation::CodeAnnotation;
use lantern_symbols_map::symbol::LNSymbol;
use lantern_symbols_map::symbol::LNSymbolData;

pub fn analyze(entry_points: &Vec<PathBuf>) -> Result<()> {
    let mut ln_map = lantern_symbols_map::build_symbols_map(&entry_points)?;
    let mut exports: Vec<LNSymbol> = Vec::new();
    let mut annotations: HashMap<usize, CodeAnnotation> = HashMap::new();

    for symbol in &ln_map.symbols {
        match &symbol.symbol {
            LNSymbolData::ExportAll(_)
            | LNSymbolData::ExportClassDecl(_, _)
            | LNSymbolData::ExportDecl(_, _)
            | LNSymbolData::ExportDefaultClassDecl(_, _)
            | LNSymbolData::ExportDefaultExpr(_)
            | LNSymbolData::ExportDefaultFnDecl(_, _)
            | LNSymbolData::ExportDefaultInterfaceDecl(_, _)
            | LNSymbolData::ExportEnumDecl(_, _)
            | LNSymbolData::ExportFnDecl(_, _)
            | LNSymbolData::ExportInterfaceDecl(_, _)
            | LNSymbolData::ExportNamed(_, _, _, _)
            | LNSymbolData::ExportTypeAliasDecl(_, _) => {
                exports.push(symbol.clone());
            }
            LNSymbolData::ImportStar(_, _, _, _)
            | LNSymbolData::ImportNamed(_, _, _, _)
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
            LNSymbolData::ImportNamed(i_name, _, file_ref, _) => {
                let idx = exports.iter().position(|x| {
                    if x.module_id != file_ref.module_id {
                        return false;
                    }

                    if let Some(e_name) = x.get_name() {
                        return e_name == i_name;
                    }

                    return false;
                });

                if let Some(idx) = idx {
                    exports.remove(idx);
                }
            }
            LNSymbolData::ExportNamed(i_name, o_name, _, Some(file_ref)) => {
                let idx = exports.iter().position(|x| {
                    if x.module_id != file_ref.module_id {
                        return false;
                    }

                    if let Some(e_name) = x.get_name() {
                        return e_name == o_name;
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
        .collect::<Vec<_>>();

    for symbol in exports {
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
            format!("unused export: {}", symbol_name)
        } else {
            "unused export".to_owned()
        };

        annotation.annotate(annotation_message, span_line, span.clone());
    }

    for (_, value) in &annotations {
        println!("{}", value.print());
        println!();
    }

    return Ok(());
}
