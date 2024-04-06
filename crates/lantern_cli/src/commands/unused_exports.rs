use std::{collections::HashMap, path::PathBuf};

use color_eyre::eyre::Result;

use lantern_code_annotation::CodeAnnotation;
use lantern_symbols_map::{self, TSSymbol, TSSymbolData};

pub fn analyze(entry_points: Vec<&PathBuf>) -> Result<()> {
    let mut ln_map = lantern_symbols_map::build(&entry_points)?;
    let mut exports: Vec<TSSymbol> = Vec::new();
    let mut annotations: HashMap<usize, CodeAnnotation> = HashMap::new();

    for symbol in &ln_map.symbols {
        match &symbol.symbol {
            TSSymbolData::ExportAll(_)
            | TSSymbolData::ExportClassDecl(_, _)
            | TSSymbolData::ExportDecl(_, _)
            | TSSymbolData::ExportDefaultClassDecl(_, _)
            | TSSymbolData::ExportDefaultExpr(_)
            | TSSymbolData::ExportDefaultFnDecl(_, _)
            | TSSymbolData::ExportDefaultInterfaceDecl(_, _)
            | TSSymbolData::ExportEnumDecl(_, _)
            | TSSymbolData::ExportFnDecl(_, _)
            | TSSymbolData::ExportInterfaceDecl(_, _)
            | TSSymbolData::ExportNamed(_, _, _, _)
            | TSSymbolData::ExportTypeAliasDecl(_, _) => {
                exports.push(symbol.clone());
            }
            TSSymbolData::ImportStar(_, _, _, _)
            | TSSymbolData::ImportNamed(_, _, _, _)
            | TSSymbolData::ImportDefault(_, _, _, _) => {}
        }
    }

    for symbol in &ln_map.symbols {
        match &symbol.symbol {
            TSSymbolData::ImportDefault(_, _, file_ref, _) => {
                let idx = exports.iter().position(|x| {
                    if x.module_id != file_ref.module_id {
                        return false;
                    }

                    match &x.symbol {
                        TSSymbolData::ExportDefaultClassDecl(_, _)
                        | TSSymbolData::ExportDefaultExpr(_)
                        | TSSymbolData::ExportDefaultFnDecl(_, _)
                        | TSSymbolData::ExportDefaultInterfaceDecl(_, _) => {
                            return true;
                        }
                        _ => return false,
                    }
                });

                if let Some(idx) = idx {
                    exports.remove(idx);
                }
            }
            TSSymbolData::ImportNamed(i_name, _, file_ref, _) => {
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
            TSSymbolData::ExportNamed(i_name, o_name, _, Some(file_ref)) => {
                let idx = exports.iter().position(|x| {
                    if x.module_id != file_ref.module_id {
                        return false;
                    }

                    if let Some(e_name) = x.get_name() {
                        return e_name == o_name;
                    }

                    if let TSSymbolData::ExportDefaultExpr(_) = x.symbol {
                        return i_name == "default";
                    }

                    return false;
                });

                if let Some(idx) = idx {
                    exports.remove(idx);
                }
            }
            TSSymbolData::ImportStar(_, _, file_ref, _) => {
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
