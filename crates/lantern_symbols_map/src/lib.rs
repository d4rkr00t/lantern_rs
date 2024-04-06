use std::{collections::HashMap, path::PathBuf};

use color_eyre::eyre::Result;

use oxc_allocator::Allocator;
use oxc_ast::{
    ast::{
        BindingPatternKind, Declaration, ExportDefaultDeclarationKind, ImportDeclarationSpecifier,
    },
    Visit,
};

use lantern_parse_ts::parse_ts;
use lantern_resolver::LanternResolver;
use oxc_span::Span;

pub fn build(entry_points: &Vec<&PathBuf>) -> Result<TSSymbolsMap> {
    let resolver = LanternResolver::new();
    let mut ts_s = TSSymbolsMap::new(resolver);
    let allocator = Allocator::default();

    for entry_point in entry_points {
        let path = entry_point.canonicalize()?;
        ts_s.add_module(TSModule {
            file_path: path,
            symbols: vec![],
            is_entry: true,
        });
    }

    let mut id = 0;

    loop {
        let module = if let Some(module) = ts_s.get_module(id) {
            module
        } else {
            break;
        };
        let source = std::fs::read_to_string(&module.file_path).unwrap();
        let path = module.file_path.clone();
        let program = parse_ts(&allocator, &source, &path)?;
        let parent = module.file_path.parent().unwrap().to_path_buf();
        let mut visitor = TSVisitor::new(id, parent, &mut ts_s);
        visitor.visit_program(&program);
        id += 1;
    }

    return Ok(ts_s);
}

pub struct TSSymbolsMap {
    pub modules: Vec<TSModule>,
    pub symbols: Vec<TSSymbol>,
    path_to_module_id: HashMap<String, usize>,
    sources: HashMap<usize, String>,
    resolver: LanternResolver,
}

impl TSSymbolsMap {
    pub fn new(resolver: LanternResolver) -> Self {
        Self {
            modules: Vec::new(),
            symbols: Vec::new(),
            path_to_module_id: HashMap::new(),
            sources: HashMap::new(),
            resolver,
        }
    }

    pub fn add_module(&mut self, module: TSModule) -> Option<usize> {
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

    pub fn get_module(&self, id: usize) -> Option<&TSModule> {
        return self.modules.get(id);
    }

    pub fn has_module(&self, path: &str) -> bool {
        return self.path_to_module_id.contains_key(path);
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

    pub fn add_symbol(&mut self, module_id: usize, symbol: TSSymbol) -> usize {
        let id = self.symbols.len();
        self.symbols.push(symbol);
        self.modules[module_id].symbols.push(id);
        return id;
    }

    pub fn resolve(&self, parent_path: &PathBuf, path: String) -> Result<PathBuf, ()> {
        return self.resolver.resolve(parent_path, &path);
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

impl<'a> Visit<'a> for TSVisitor<'a> {
    // export * from "./path";
    fn visit_export_all_declaration(&mut self, decl: &oxc_ast::ast::ExportAllDeclaration<'a>) {
        let path = self
            .symbols_map
            .resolve(&self.parent_path, decl.source.value.to_string())
            .unwrap();

        let module_id = self.symbols_map.add_module(TSModule {
            file_path: path,
            symbols: vec![],
            is_entry: false,
        });

        if module_id.is_none() {
            return;
        }

        self.symbols_map.add_symbol(
            self.module_id,
            TSSymbol {
                module_id: self.module_id,
                symbol: TSSymbolData::ExportAll(FileReference::new(module_id.unwrap(), decl.span)),
            },
        );
    }

    // export default <expr>;
    //
    // export default function a() {}
    // export default function() {}
    // export default class A {}
    // export default class {}
    // export default interface II {}
    fn visit_export_default_declaration(
        &mut self,
        decl: &oxc_ast::ast::ExportDefaultDeclaration<'a>,
    ) {
        match &decl.declaration {
            ExportDefaultDeclarationKind::Expression(_) => {
                self.symbols_map.add_symbol(
                    self.module_id,
                    TSSymbol {
                        module_id: self.module_id,
                        symbol: TSSymbolData::ExportDefaultExpr(decl.span.clone()),
                    },
                );
            }
            ExportDefaultDeclarationKind::ClassDeclaration(class_decl) => {
                let name = if let Some(ident) = &class_decl.id {
                    Some(ident.name.to_string())
                } else {
                    None
                };
                self.symbols_map.add_symbol(
                    self.module_id,
                    TSSymbol {
                        module_id: self.module_id,
                        symbol: TSSymbolData::ExportDefaultClassDecl(name, class_decl.span.clone()),
                    },
                );
            }
            ExportDefaultDeclarationKind::FunctionDeclaration(fn_decl) => {
                let name = if let Some(ident) = &fn_decl.id {
                    Some(ident.name.to_string())
                } else {
                    None
                };
                self.symbols_map.add_symbol(
                    self.module_id,
                    TSSymbol {
                        module_id: self.module_id,
                        symbol: TSSymbolData::ExportDefaultFnDecl(name, fn_decl.span.clone()),
                    },
                );
            }
            ExportDefaultDeclarationKind::TSInterfaceDeclaration(ts_interface_decl) => {
                self.symbols_map.add_symbol(
                    self.module_id,
                    TSSymbol {
                        module_id: self.module_id,
                        symbol: TSSymbolData::ExportDefaultInterfaceDecl(
                            ts_interface_decl.id.name.to_string(),
                            ts_interface_decl.span.clone(),
                        ),
                    },
                );
            }
            ExportDefaultDeclarationKind::TSEnumDeclaration(_) => {}
        }
    }

    // export function a() {}
    // export const a = 1;
    // export let a = 1;
    // export var a = 1;
    // export class A {}
    // export interface A {}
    // export enum A {}
    //
    // export { default } from "@atlaskit/editor-plugin-block-type";
    // export { a, b } from "./exports_decl";
    // export { a as a2, b as b2 } from "./exports_decl";
    // export { c };
    fn visit_export_named_declaration(&mut self, decl: &oxc_ast::ast::ExportNamedDeclaration<'a>) {
        if let Some(decl) = &decl.declaration {
            match decl {
                Declaration::VariableDeclaration(decl) => {
                    for var_decl in &decl.declarations {
                        match &var_decl.id.kind {
                            BindingPatternKind::BindingIdentifier(ident) => {
                                self.symbols_map.add_symbol(
                                    self.module_id,
                                    TSSymbol {
                                        module_id: self.module_id,
                                        symbol: TSSymbolData::ExportDecl(
                                            ident.name.to_string(),
                                            ident.span.clone(),
                                        ),
                                    },
                                );
                            }
                            _ => {}
                        }
                    }
                }
                Declaration::FunctionDeclaration(fn_decl) => {
                    let name = fn_decl.id.clone().unwrap().name.to_string();
                    self.symbols_map.add_symbol(
                        self.module_id,
                        TSSymbol {
                            module_id: self.module_id,
                            symbol: TSSymbolData::ExportFnDecl(name, fn_decl.span.clone()),
                        },
                    );
                }
                Declaration::ClassDeclaration(class_decl) => {
                    self.symbols_map.add_symbol(
                        self.module_id,
                        TSSymbol {
                            module_id: self.module_id,
                            symbol: TSSymbolData::ExportClassDecl(
                                class_decl.id.clone().unwrap().name.to_string(),
                                class_decl.span.clone(),
                            ),
                        },
                    );
                }
                Declaration::TSEnumDeclaration(ts_enum_decl) => {
                    self.symbols_map.add_symbol(
                        self.module_id,
                        TSSymbol {
                            module_id: self.module_id,
                            symbol: TSSymbolData::ExportEnumDecl(
                                ts_enum_decl.id.name.to_string(),
                                ts_enum_decl.span.clone(),
                            ),
                        },
                    );
                }
                Declaration::TSInterfaceDeclaration(ts_interface_decl) => {
                    self.symbols_map.add_symbol(
                        self.module_id,
                        TSSymbol {
                            module_id: self.module_id,
                            symbol: TSSymbolData::ExportInterfaceDecl(
                                ts_interface_decl.id.name.to_string(),
                                ts_interface_decl.span.clone(),
                            ),
                        },
                    );
                }
                Declaration::TSTypeAliasDeclaration(ts_type_alias_decl) => {
                    self.symbols_map.add_symbol(
                        self.module_id,
                        TSSymbol {
                            module_id: self.module_id,
                            symbol: TSSymbolData::ExportTypeAliasDecl(
                                ts_type_alias_decl.id.name.to_string(),
                                ts_type_alias_decl.span.clone(),
                            ),
                        },
                    );
                }
                _ => {}
            }
        } else {
            let src = if let Some(src) = &decl.source {
                let path = self
                    .symbols_map
                    .resolve(&self.parent_path, src.value.to_string())
                    .unwrap();
                let module_id = self.symbols_map.add_module(TSModule {
                    file_path: path,
                    symbols: vec![],
                    is_entry: false,
                });
                if module_id.is_none() {
                    None
                } else {
                    Some(FileReference::new(module_id.unwrap(), src.span))
                }
            } else {
                None
            };

            for spec in &decl.specifiers {
                let local = spec.local.name().to_string();
                let exported = spec.exported.name().to_string();

                self.symbols_map.add_symbol(
                    self.module_id,
                    TSSymbol {
                        module_id: self.module_id,
                        symbol: TSSymbolData::ExportNamed(
                            local,
                            exported,
                            spec.span.clone(),
                            src.clone(),
                        ),
                    },
                );
            }
        }
    }

    fn visit_import_declaration(&mut self, import_decl: &oxc_ast::ast::ImportDeclaration<'a>) {
        let path = self
            .symbols_map
            .resolve(&self.parent_path, import_decl.source.value.to_string())
            .unwrap();

        let module_id = self.symbols_map.add_module(TSModule {
            file_path: path,
            symbols: vec![],
            is_entry: false,
        });

        if module_id.is_none() {
            return;
        }

        let src = FileReference::new(module_id.unwrap(), import_decl.source.span);
        let type_only = import_decl.import_kind.is_type();
        if let Some(specifiers) = &import_decl.specifiers {
            for spec in specifiers {
                match spec {
                    // import React from "react";
                    ImportDeclarationSpecifier::ImportDefaultSpecifier(spec) => {
                        self.symbols_map.add_symbol(
                            self.module_id,
                            TSSymbol {
                                module_id: self.module_id,
                                symbol: TSSymbolData::ImportDefault(
                                    spec.local.name.to_string(),
                                    spec.span,
                                    src.clone(),
                                    type_only,
                                ),
                            },
                        );
                    }
                    // import * as React from "react";
                    ImportDeclarationSpecifier::ImportNamespaceSpecifier(spec) => {
                        self.symbols_map.add_symbol(
                            self.module_id,
                            TSSymbol {
                                module_id: self.module_id,
                                symbol: TSSymbolData::ImportStar(
                                    spec.local.name.to_string(),
                                    spec.span,
                                    src.clone(),
                                    type_only,
                                ),
                            },
                        );
                    }
                    // import { useState } from "react";
                    ImportDeclarationSpecifier::ImportSpecifier(spec) => {
                        self.symbols_map.add_symbol(
                            self.module_id,
                            TSSymbol {
                                module_id: self.module_id,
                                symbol: TSSymbolData::ImportNamed(
                                    spec.local.name.to_string(),
                                    spec.span,
                                    src.clone(),
                                    type_only || spec.import_kind.is_type(),
                                ),
                            },
                        );
                    }
                }
            }
        }
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

#[derive(Debug, Clone)]
pub struct FileReference {
    pub module_id: usize,
    pub span: Span,
}

impl FileReference {
    pub fn new(module_id: usize, span: Span) -> Self {
        Self { module_id, span }
    }
}

#[derive(Debug, Clone)]
pub enum TSSymbolData {
    ExportAll(FileReference),

    ExportDefaultExpr(Span),
    ExportNamed(String, String, Span, Option<FileReference>),

    ExportDecl(String, Span),
    ExportFnDecl(String, Span),
    ExportClassDecl(String, Span),

    ExportEnumDecl(String, Span),
    ExportInterfaceDecl(String, Span),
    ExportTypeAliasDecl(String, Span),

    ExportDefaultClassDecl(Option<String>, Span),
    ExportDefaultFnDecl(Option<String>, Span),
    ExportDefaultInterfaceDecl(String, Span),

    ImportDefault(String, Span, FileReference, bool),
    ImportStar(String, Span, FileReference, bool),
    ImportNamed(String, Span, FileReference, bool),
}
