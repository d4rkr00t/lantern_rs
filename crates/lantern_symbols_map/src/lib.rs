use std::path::PathBuf;

mod module;
pub mod symbol;
pub mod symbols_map;

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

use module::LNModule;
use symbol::{LNFileReference, LNSymbol, LNSymbolData};
use symbols_map::LNSymbolsMap;

pub struct LNVisitor<'a> {
    module_id: usize,
    parent_path: PathBuf,
    symbols_map: &'a mut LNSymbolsMap,
}

pub fn build_symbols_map(entry_points: &Vec<PathBuf>) -> Result<LNSymbolsMap> {
    let resolver = LanternResolver::new();
    let mut ln_symbols_map = LNSymbolsMap::new(resolver);
    let allocator = Allocator::default();

    for entry_point in entry_points {
        let path = entry_point.canonicalize()?;
        ln_symbols_map.add_module(LNModule {
            file_path: path,
            symbols: vec![],
            is_entry: true,
        });
    }

    let mut id = 0;

    loop {
        let module = if let Some(module) = ln_symbols_map.get_module(id) {
            module
        } else {
            break;
        };
        let source = std::fs::read_to_string(&module.file_path).unwrap();
        let path = module.file_path.clone();
        let program = parse_ts(&allocator, &source, &path)?;
        let parent = module.file_path.parent().unwrap().to_path_buf();
        let mut visitor = LNVisitor::new(id, parent, &mut ln_symbols_map);
        visitor.visit_program(&program);
        id += 1;
    }

    return Ok(ln_symbols_map);
}

impl<'a> LNVisitor<'a> {
    pub fn new(file_id: usize, parent_path: PathBuf, ts_s: &'a mut LNSymbolsMap) -> Self {
        return Self {
            parent_path,
            module_id: file_id,
            symbols_map: ts_s,
        };
    }
}

impl<'a> Visit<'a> for LNVisitor<'a> {
    // export * from "./path";
    fn visit_export_all_declaration(&mut self, decl: &oxc_ast::ast::ExportAllDeclaration<'a>) {
        let maybe_path = self
            .symbols_map
            .resolve(&self.parent_path, decl.source.value.to_string());

        if maybe_path.is_err() {
            println!("{}", maybe_path.err().unwrap());
            return;
        }

        let path = maybe_path.unwrap();

        let module_id = self.symbols_map.add_module(LNModule {
            file_path: path,
            symbols: vec![],
            is_entry: false,
        });

        if module_id.is_none() {
            return;
        }

        self.symbols_map.add_symbol(
            self.module_id,
            LNSymbol {
                module_id: self.module_id,
                symbol: LNSymbolData::ExportAll(LNFileReference::new(
                    module_id.unwrap(),
                    decl.span,
                )),
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
                    LNSymbol {
                        module_id: self.module_id,
                        symbol: LNSymbolData::ExportDefaultExpr(decl.span.clone()),
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
                    LNSymbol {
                        module_id: self.module_id,
                        symbol: LNSymbolData::ExportDefaultClassDecl(name, class_decl.span.clone()),
                    },
                );
            }
            ExportDefaultDeclarationKind::FunctionDeclaration(fn_decl) => {
                let name = if let Some(ident) = &fn_decl.id {
                    Some(ident.name.to_string())
                } else {
                    None
                };
                let span = if let Some(ident) = &fn_decl.id {
                    ident.span.clone()
                } else {
                    fn_decl.span.clone()
                };
                self.symbols_map.add_symbol(
                    self.module_id,
                    LNSymbol {
                        module_id: self.module_id,
                        symbol: LNSymbolData::ExportDefaultFnDecl(name, span),
                    },
                );
            }
            ExportDefaultDeclarationKind::TSInterfaceDeclaration(ts_interface_decl) => {
                self.symbols_map.add_symbol(
                    self.module_id,
                    LNSymbol {
                        module_id: self.module_id,
                        symbol: LNSymbolData::ExportDefaultInterfaceDecl(
                            ts_interface_decl.id.name.to_string(),
                            ts_interface_decl.id.span.clone(),
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
                                    LNSymbol {
                                        module_id: self.module_id,
                                        symbol: LNSymbolData::ExportDecl(
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
                        LNSymbol {
                            module_id: self.module_id,
                            symbol: LNSymbolData::ExportFnDecl(name, fn_decl.span.clone()),
                        },
                    );
                }
                Declaration::ClassDeclaration(class_decl) => {
                    self.symbols_map.add_symbol(
                        self.module_id,
                        LNSymbol {
                            module_id: self.module_id,
                            // TODO: fix unwrap
                            symbol: LNSymbolData::ExportClassDecl(
                                class_decl.id.clone().unwrap().name.to_string(),
                                class_decl.id.clone().unwrap().span.clone(),
                            ),
                        },
                    );
                }
                Declaration::TSEnumDeclaration(ts_enum_decl) => {
                    self.symbols_map.add_symbol(
                        self.module_id,
                        LNSymbol {
                            module_id: self.module_id,
                            symbol: LNSymbolData::ExportEnumDecl(
                                ts_enum_decl.id.name.to_string(),
                                ts_enum_decl.id.span.clone(),
                            ),
                        },
                    );
                }
                Declaration::TSInterfaceDeclaration(ts_interface_decl) => {
                    self.symbols_map.add_symbol(
                        self.module_id,
                        LNSymbol {
                            module_id: self.module_id,
                            symbol: LNSymbolData::ExportInterfaceDecl(
                                ts_interface_decl.id.name.to_string(),
                                ts_interface_decl.id.span.clone(),
                            ),
                        },
                    );
                }
                Declaration::TSTypeAliasDeclaration(ts_type_alias_decl) => {
                    self.symbols_map.add_symbol(
                        self.module_id,
                        LNSymbol {
                            module_id: self.module_id,
                            symbol: LNSymbolData::ExportTypeAliasDecl(
                                ts_type_alias_decl.id.name.to_string(),
                                ts_type_alias_decl.id.span.clone(),
                            ),
                        },
                    );
                }
                _ => {}
            }
        } else {
            let src = if let Some(src) = &decl.source {
                let maybe_path = self
                    .symbols_map
                    .resolve(&self.parent_path, src.value.to_string());

                if maybe_path.is_err() {
                    println!("{}", maybe_path.err().unwrap());
                    return;
                }

                let path = maybe_path.unwrap();

                let module_id = self.symbols_map.add_module(LNModule {
                    file_path: path,
                    symbols: vec![],
                    is_entry: false,
                });
                if module_id.is_none() {
                    None
                } else {
                    Some(LNFileReference::new(module_id.unwrap(), src.span))
                }
            } else {
                None
            };

            for spec in &decl.specifiers {
                let local = spec.local.name().to_string();
                let exported = spec.exported.name().to_string();

                self.symbols_map.add_symbol(
                    self.module_id,
                    LNSymbol {
                        module_id: self.module_id,
                        symbol: LNSymbolData::ExportNamed(
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
        let maybe_path = self
            .symbols_map
            .resolve(&self.parent_path, import_decl.source.value.to_string());

        if maybe_path.is_err() {
            println!("{}", maybe_path.err().unwrap());
            return;
        }

        let path = maybe_path.unwrap();

        let module_id = self.symbols_map.add_module(LNModule {
            file_path: path,
            symbols: vec![],
            is_entry: false,
        });

        if module_id.is_none() {
            return;
        }

        let src = LNFileReference::new(module_id.unwrap(), import_decl.source.span);
        let type_only = import_decl.import_kind.is_type();
        if let Some(specifiers) = &import_decl.specifiers {
            for spec in specifiers {
                match spec {
                    // import React from "react";
                    ImportDeclarationSpecifier::ImportDefaultSpecifier(spec) => {
                        self.symbols_map.add_symbol(
                            self.module_id,
                            LNSymbol {
                                module_id: self.module_id,
                                symbol: LNSymbolData::ImportDefault(
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
                            LNSymbol {
                                module_id: self.module_id,
                                symbol: LNSymbolData::ImportStar(
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
                            LNSymbol {
                                module_id: self.module_id,
                                symbol: LNSymbolData::ImportNamed(
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
