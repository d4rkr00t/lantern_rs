use std::{collections::HashMap, path::PathBuf};

use color_eyre::eyre::Result;

use lantern_parse_ts::parse_ts;

use swc_common::{FileName, Span};
use swc_ecma_ast::ExportAll;
use swc_ecma_loader::{resolve::Resolve, resolvers::node::NodeModulesResolver};
use swc_ecma_visit::Visit;

pub fn build(path: &PathBuf) -> Result<TSSymbolsMap> {
    let mut ts_s = TSSymbolsMap::new();
    ts_s.add_module(TSModule {
        file_path: path.clone(),
        symbols: vec![],
        is_entry: true,
    });

    let mut id = 0;

    loop {
        let module = if let Some(module) = ts_s.get_module(id) {
            module
        } else {
            break;
        };
        let program = parse_ts(&module.file_path)?;
        let parent = module.file_path.parent().unwrap().to_path_buf();
        let mut visitor = TSVisitor::new(id, parent, &mut ts_s);
        visitor.visit_program(&program);
        id += 1;
    }

    return Ok(ts_s);
}

#[derive(Debug)]
pub struct TSSymbolsMap {
    pub modules: Vec<TSModule>,
    pub symbols: Vec<TSSymbol>,
    path_to_module_id: HashMap<String, usize>,
    resolver: NodeModulesResolver,
    sources: HashMap<usize, String>,
}

impl TSSymbolsMap {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            symbols: Vec::new(),
            path_to_module_id: HashMap::new(),
            resolver: NodeModulesResolver::new(
                swc_ecma_loader::TargetEnv::Node,
                Default::default(),
                false,
            ),
            sources: HashMap::new(),
        }
    }

    pub fn resolve(&self, from: &PathBuf, to: &str) -> Result<PathBuf> {
        let path = self.resolver.resolve(&FileName::Real(from.clone()), to);
        match path {
            Ok(FileName::Real(p)) => Ok(p),
            _ => Err(color_eyre::eyre::eyre!(
                "Couldn't resolve path {} from {}",
                to,
                from.display()
            )),
        }
    }

    pub fn add_module(&mut self, module: TSModule) -> usize {
        if self.has_module(module.file_path.to_str().unwrap()) {
            return self.path_to_module_id[module.file_path.to_str().unwrap()];
        }

        let id = self.modules.len();
        self.modules.push(module);
        self.path_to_module_id
            .insert(self.modules[id].file_path.to_str().unwrap().to_string(), id);
        return id;
    }

    pub fn get_module(&self, id: usize) -> Option<&TSModule> {
        return self.modules.get(id);
    }

    pub fn get_module_id(&self, path: &str) -> Option<usize> {
        return self.path_to_module_id.get(path).copied();
    }

    pub fn has_module(&self, path: &str) -> bool {
        return self.path_to_module_id.contains_key(path);
    }

    pub fn add_symbol(&mut self, module_id: usize, symbol: TSSymbol) -> usize {
        let id = self.symbols.len();
        self.symbols.push(symbol);
        self.modules[module_id].symbols.push(id);
        return id;
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

    pub fn get_module_path(&self, module_id: usize) -> &PathBuf {
        return &self.modules[module_id].file_path;
    }

    pub fn get_source_from_span(&mut self, module_id: usize, span: &Span) -> String {
        let source = self.get_module_source(module_id);
        return source[span.lo.0 as usize..span.hi.0 as usize].to_string();
    }

    pub fn get_line_number_from_span(&mut self, module_id: usize, span: &Span) -> usize {
        if span.lo.0 == 0 {
            return 1;
        }
        let source = self.get_module_source(module_id);
        let source = source[0..span.lo.0 as usize].to_string();
        return source.lines().count();
    }
}

#[derive(Debug)]
pub struct TSModule {
    pub file_path: PathBuf,
    pub symbols: Vec<usize>,
    pub is_entry: bool,
}

#[derive(Debug, Clone)]
pub struct TSSymbol {
    pub module_id: usize,
    pub symbol: TSSymbolData,
}

impl TSSymbol {
    pub fn get_span(&self) -> &Span {
        match &self.symbol {
            TSSymbolData::ExportAll(file_ref) => &file_ref.span,
            TSSymbolData::ExportClassDecl(_, span) => span,
            TSSymbolData::ExportDecl(_, span) => span,
            TSSymbolData::ExportDefaultClassDecl(_, span) => span,
            TSSymbolData::ExportDefaultDecl(_, span) => span,
            TSSymbolData::ExportDefaultExpr(span) => span,
            TSSymbolData::ExportDefaultFnDecl(_, span) => span,
            TSSymbolData::ExportDefaultInterfaceDecl(_, span) => span,
            TSSymbolData::ExportEnumDecl(_, span) => span,
            TSSymbolData::ExportFnDecl(_, span) => span,
            TSSymbolData::ExportInterfaceDecl(_, span) => span,
            TSSymbolData::ExportTypeAliasDecl(_, span) => span,
            TSSymbolData::ExportNamed(_, _, span, _) => span,
            TSSymbolData::ImportDefault(_, span, _, _) => span,
            TSSymbolData::ImportStar(_, span, _, _) => span,
            TSSymbolData::ImportNamed(_, span, _, _) => span,
        }
    }

    pub fn get_name(&self) -> Option<&str> {
        match &self.symbol {
            TSSymbolData::ExportAll(_) => None,
            TSSymbolData::ExportClassDecl(name, _) => Some(name),
            TSSymbolData::ExportDecl(name, _) => Some(name),
            TSSymbolData::ExportDefaultClassDecl(name, _) => name.as_deref(),
            TSSymbolData::ExportDefaultDecl(name, _) => Some(name),
            TSSymbolData::ExportDefaultExpr(_) => None,
            TSSymbolData::ExportDefaultFnDecl(name, _) => name.as_deref(),
            TSSymbolData::ExportDefaultInterfaceDecl(name, _) => Some(name),
            TSSymbolData::ExportEnumDecl(name, _) => Some(name),
            TSSymbolData::ExportFnDecl(name, _) => Some(name),
            TSSymbolData::ExportInterfaceDecl(name, _) => Some(name),
            TSSymbolData::ExportTypeAliasDecl(name, _) => Some(name),
            TSSymbolData::ExportNamed(name, None, _, _) => Some(name),
            TSSymbolData::ExportNamed(_, Some(name), _, _) => Some(name),
            TSSymbolData::ImportDefault(name, _, _, _) => Some(name),
            TSSymbolData::ImportStar(name, _, _, _) => Some(name),
            TSSymbolData::ImportNamed(name, _, _, _) => Some(name),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TSSymbolData {
    ExportAll(FileReference),
    ExportClassDecl(String, Span),
    ExportDecl(String, Span),
    ExportDefaultClassDecl(Option<String>, Span),
    ExportDefaultDecl(String, Span),
    ExportDefaultExpr(Span),
    ExportDefaultFnDecl(Option<String>, Span),
    ExportDefaultInterfaceDecl(String, Span),
    ExportEnumDecl(String, Span),
    ExportFnDecl(String, Span),
    ExportInterfaceDecl(String, Span),
    ExportTypeAliasDecl(String, Span),
    ExportNamed(String, Option<String>, Span, Option<FileReference>),

    ImportDefault(String, Span, FileReference, bool),
    ImportStar(String, Span, FileReference, bool),
    ImportNamed(String, Span, FileReference, bool),
}

#[derive(Debug, Clone)]
pub struct FileReference {
    pub module_id: usize,
    pub span: Span,
}

impl FileReference {
    pub fn new(module_id: usize, span: &Span) -> Self {
        Self {
            module_id,
            span: span.clone(),
        }
    }
}

struct TSVisitor<'a> {
    module_id: usize,
    parent_path: PathBuf,
    symbols_map: &'a mut TSSymbolsMap,
}

impl<'a> TSVisitor<'a> {
    pub fn new(file_id: usize, parent_path: PathBuf, ts_s: &'a mut TSSymbolsMap) -> Self {
        return Self {
            parent_path,
            module_id: file_id,
            symbols_map: ts_s,
        };
    }
}

impl<'a> Visit for TSVisitor<'a> {
    // export * from "./path";
    fn visit_export_all(&mut self, export_all: &ExportAll) {
        let path = self
            .symbols_map
            .resolve(&self.parent_path, &export_all.src.value)
            .unwrap();
        let module_id = self.symbols_map.add_module(TSModule {
            file_path: path,
            symbols: vec![],
            is_entry: false,
        });
        self.symbols_map.add_symbol(
            self.module_id,
            TSSymbol {
                module_id: self.module_id,
                symbol: TSSymbolData::ExportAll(FileReference::new(module_id, &export_all.span)),
            },
        );
    }

    // export default <expr>;
    fn visit_export_default_expr(&mut self, export_default_expr: &swc_ecma_ast::ExportDefaultExpr) {
        self.symbols_map.add_symbol(
            self.module_id,
            TSSymbol {
                module_id: self.module_id,
                symbol: TSSymbolData::ExportDefaultExpr(export_default_expr.span.clone()),
            },
        );
    }

    // export function a() {}
    // export const a = 1;
    // export let a = 1;
    // export var a = 1;
    // export class A {}
    // export interface A {}
    // export enum A {}
    fn visit_export_decl(&mut self, export_decl: &swc_ecma_ast::ExportDecl) {
        match &export_decl.decl {
            swc_ecma_ast::Decl::Class(class_decl) => {
                self.symbols_map.add_symbol(
                    self.module_id,
                    TSSymbol {
                        module_id: self.module_id,
                        symbol: TSSymbolData::ExportClassDecl(
                            class_decl.ident.sym.to_string(),
                            class_decl.class.span.clone(),
                        ),
                    },
                );
            }
            swc_ecma_ast::Decl::Fn(fn_decl) => {
                self.symbols_map.add_symbol(
                    self.module_id,
                    TSSymbol {
                        module_id: self.module_id,
                        symbol: TSSymbolData::ExportFnDecl(
                            fn_decl.ident.sym.to_string(),
                            fn_decl.function.span.clone(),
                        ),
                    },
                );
            }
            swc_ecma_ast::Decl::Var(var_decl) => {
                for decl in &var_decl.decls {
                    if let swc_ecma_ast::Pat::Ident(ident) = &decl.name {
                        self.symbols_map.add_symbol(
                            self.module_id,
                            TSSymbol {
                                module_id: self.module_id,
                                symbol: TSSymbolData::ExportDecl(
                                    ident.id.sym.to_string(),
                                    decl.span.clone(),
                                ),
                            },
                        );
                    }
                }
            }
            swc_ecma_ast::Decl::TsEnum(ts_enum_decl) => {
                self.symbols_map.add_symbol(
                    self.module_id,
                    TSSymbol {
                        module_id: self.module_id,
                        symbol: TSSymbolData::ExportEnumDecl(
                            ts_enum_decl.id.sym.to_string(),
                            ts_enum_decl.span.clone(),
                        ),
                    },
                );
            }
            swc_ecma_ast::Decl::TsInterface(ts_interface_decl) => {
                self.symbols_map.add_symbol(
                    self.module_id,
                    TSSymbol {
                        module_id: self.module_id,
                        symbol: TSSymbolData::ExportInterfaceDecl(
                            ts_interface_decl.id.sym.to_string(),
                            ts_interface_decl.span.clone(),
                        ),
                    },
                );
            }
            swc_ecma_ast::Decl::TsTypeAlias(ts_type_alias_decl) => {
                self.symbols_map.add_symbol(
                    self.module_id,
                    TSSymbol {
                        module_id: self.module_id,
                        symbol: TSSymbolData::ExportTypeAliasDecl(
                            ts_type_alias_decl.id.sym.to_string(),
                            ts_type_alias_decl.span.clone(),
                        ),
                    },
                );
            }
            _ => {}
        }
    }

    // export { default } from "@atlaskit/editor-plugin-block-type";
    // export { a, b } from "./exports_decl";
    // export { a as a2, b as b2 } from "./exports_decl";
    // export { c };
    fn visit_named_export(&mut self, n: &swc_ecma_ast::NamedExport) {
        for spec in &n.specifiers {
            match &spec {
                &swc_ecma_ast::ExportSpecifier::Named(named_export_specifier) => {
                    let orig = &named_export_specifier.orig;
                    let orig = match &orig {
                        &swc_ecma_ast::ModuleExportName::Ident(ident) => ident.sym.to_string(),
                        &swc_ecma_ast::ModuleExportName::Str(str_) => str_.value.to_string(),
                    };

                    let exported = if let Some(exported) = &named_export_specifier.exported {
                        match &exported {
                            &swc_ecma_ast::ModuleExportName::Ident(ident) => {
                                Some(ident.sym.to_string())
                            }
                            &swc_ecma_ast::ModuleExportName::Str(str_) => {
                                Some(str_.value.to_string())
                            }
                        }
                    } else {
                        None
                    };

                    let src = if let Some(src) = &n.src {
                        let path = self
                            .symbols_map
                            .resolve(&self.parent_path, &src.value)
                            .unwrap();
                        let module_id = self.symbols_map.add_module(TSModule {
                            file_path: path,
                            symbols: vec![],
                            is_entry: false,
                        });
                        Some(FileReference::new(module_id, &src.span))
                    } else {
                        None
                    };

                    self.symbols_map.add_symbol(
                        self.module_id,
                        TSSymbol {
                            module_id: self.module_id,
                            symbol: TSSymbolData::ExportNamed(
                                orig,
                                exported,
                                named_export_specifier.span.clone(),
                                src,
                            ),
                        },
                    );
                }
                _ => {}
            }
        }
    }

    // export default function a() {}
    // export default function() {}
    // export default class A {}
    // export default class {}
    // export default interface II {}
    fn visit_export_default_decl(&mut self, export_default_decl: &swc_ecma_ast::ExportDefaultDecl) {
        match &export_default_decl.decl {
            swc_ecma_ast::DefaultDecl::Class(class_decl) => {
                let name = if let Some(ident) = &class_decl.ident {
                    Some(ident.sym.to_string())
                } else {
                    None
                };
                self.symbols_map.add_symbol(
                    self.module_id,
                    TSSymbol {
                        module_id: self.module_id,
                        symbol: TSSymbolData::ExportDefaultClassDecl(
                            name,
                            class_decl.class.span.clone(),
                        ),
                    },
                );
            }
            swc_ecma_ast::DefaultDecl::Fn(fn_decl) => {
                let name = if let Some(ident) = &fn_decl.ident {
                    Some(ident.sym.to_string())
                } else {
                    None
                };
                self.symbols_map.add_symbol(
                    self.module_id,
                    TSSymbol {
                        module_id: self.module_id,
                        symbol: TSSymbolData::ExportDefaultFnDecl(
                            name,
                            fn_decl.function.span.clone(),
                        ),
                    },
                );
            }
            swc_ecma_ast::DefaultDecl::TsInterfaceDecl(ts_interface_decl) => {
                self.symbols_map.add_symbol(
                    self.module_id,
                    TSSymbol {
                        module_id: self.module_id,
                        symbol: TSSymbolData::ExportDefaultInterfaceDecl(
                            ts_interface_decl.id.sym.to_string(),
                            ts_interface_decl.span.clone(),
                        ),
                    },
                );
            }
        }
    }

    fn visit_import_decl(&mut self, import_decl: &swc_ecma_ast::ImportDecl) {
        let path = self
            .symbols_map
            .resolve(&self.parent_path, &import_decl.src.value)
            .unwrap();
        let module_id = self.symbols_map.add_module(TSModule {
            file_path: path,
            symbols: vec![],
            is_entry: false,
        });
        let src = FileReference::new(module_id, &import_decl.src.span);
        let type_only = import_decl.type_only;
        for spec in &import_decl.specifiers {
            match &spec {
                // import React from "react";
                &swc_ecma_ast::ImportSpecifier::Default(spec) => {
                    self.symbols_map.add_symbol(
                        self.module_id,
                        TSSymbol {
                            module_id: self.module_id,
                            symbol: TSSymbolData::ImportDefault(
                                spec.local.sym.to_string(),
                                spec.span,
                                src.clone(),
                                type_only,
                            ),
                        },
                    );
                }
                // import * as React from "react";
                &swc_ecma_ast::ImportSpecifier::Namespace(spec) => {
                    self.symbols_map.add_symbol(
                        self.module_id,
                        TSSymbol {
                            module_id: self.module_id,
                            symbol: TSSymbolData::ImportStar(
                                spec.local.sym.to_string(),
                                spec.span,
                                src.clone(),
                                type_only,
                            ),
                        },
                    );
                }
                // import { useState } from "react";
                &swc_ecma_ast::ImportSpecifier::Named(spec) => {
                    self.symbols_map.add_symbol(
                        self.module_id,
                        TSSymbol {
                            module_id: self.module_id,
                            symbol: TSSymbolData::ImportNamed(
                                spec.local.sym.to_string(),
                                spec.span,
                                src.clone(),
                                type_only || spec.is_type_only,
                            ),
                        },
                    );
                }
            }
        }
    }
}
