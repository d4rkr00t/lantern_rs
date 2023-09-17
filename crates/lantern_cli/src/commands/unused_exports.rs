use std::path::PathBuf;

use color_eyre::eyre::Result;

use lantern_symbols_map::{self, TSSymbol, TSSymbolData};

pub fn analyze(project_path: PathBuf) -> Result<()> {
    let abs_path = project_path.canonicalize()?;
    let mut ln_map = lantern_symbols_map::build(&abs_path)?;
    let mut exports: Vec<TSSymbol> = Vec::new();
    for symbol in &ln_map.symbols {
        match &symbol.symbol {
            TSSymbolData::ExportAll(_)
            | TSSymbolData::ExportClassDecl(_, _)
            | TSSymbolData::ExportDecl(_, _)
            | TSSymbolData::ExportDefaultClassDecl(_, _)
            | TSSymbolData::ExportDefaultDecl(_, _)
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
            _ => {}
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
                        | TSSymbolData::ExportDefaultDecl(_, _)
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
            TSSymbolData::ImportNamed(i_name, _, file_ref, _)
            | TSSymbolData::ExportNamed(i_name, _, _, Some(file_ref)) => {
                let idx = exports.iter().position(|x| {
                    if x.module_id != file_ref.module_id {
                        return false;
                    }

                    match &x.symbol {
                        TSSymbolData::ExportClassDecl(e_name, _)
                        | TSSymbolData::ExportDecl(e_name, _)
                        | TSSymbolData::ExportEnumDecl(e_name, _)
                        | TSSymbolData::ExportFnDecl(e_name, _)
                        | TSSymbolData::ExportInterfaceDecl(e_name, _)
                        | TSSymbolData::ExportTypeAliasDecl(e_name, _)
                        | TSSymbolData::ExportNamed(e_name, _, _, _) => {
                            return e_name == i_name;
                        }
                        _ => return false,
                    }
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
        let span_source = ln_map.get_source_from_span(symbol.module_id, span);
        let span_line = ln_map.get_line_number_from_span(symbol.module_id, span);
        let code = printable_code_snippet(&span_source, span_line);
        let module = ln_map.get_module(symbol.module_id).unwrap();
        println!(
            "unused export in {:?} on line {}: \n\x1B[40m{}",
            &module.file_path, span_line, code,
        );
        println!();
        println!("\x1B[0m\x1B[39m---------------------");
        println!();
    }
    // println!("unused exports: {:#?}", exports);

    return Ok(());
}

fn printable_code_snippet(source: &str, start_line_num: usize) -> String {
    let mut lines = source.lines();
    let mut output: Vec<String> = Vec::new();

    while let Some(line) = lines.next() {
        let line_num = start_line_num + output.len();
        output.push(format!("{} | {}", line_num, line));
    }

    return output.join("\n").to_string();
}
