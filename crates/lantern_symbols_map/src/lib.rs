use std::path::PathBuf;

use color_eyre::eyre::Result;

use lantern_parse_ts::parse_ts;

use swc_atoms::JsWord;
use swc_common::Span;
use swc_ecma_ast::ExportAll;
use swc_ecma_visit::Visit;

pub fn build(path: &PathBuf) -> Result<TSSymbolsMap> {
    let mut ts_s = TSSymbolsMap::new(vec![path.clone()]);
    ts_s.build()?;
    return Ok(ts_s);
}

#[derive(Debug)]
pub struct TSSymbolsMap {
    pub files: Vec<PathBuf>,
    pub symbols: Vec<TSSymbol>,
}

impl TSSymbolsMap {
    pub fn new(entry_points: Vec<PathBuf>) -> Self {
        Self {
            files: entry_points,
            symbols: vec![],
        }
    }

    pub fn build(&mut self) -> Result<()> {
        for (i, file) in self.files.iter().enumerate() {
            let program = parse_ts(file)?;
            let mut visitor = TSVisitor::new(i);
            visitor.visit_program(&program);

            for symbol in visitor.symbols {
                self.symbols.push(symbol);
            }
            return Ok(());
        }

        return Ok(());
    }
}

#[derive(Debug)]
pub struct TSSymbol {
    file_id: usize,
    symbol: TSSymbolData,
}

#[derive(Debug)]
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
    pub raw: JsWord,
    pub span: Span,
}

impl FileReference {
    pub fn new(raw: &JsWord, span: &Span) -> Self {
        Self {
            raw: raw.clone(),
            span: span.clone(),
        }
    }
}

struct TSVisitor {
    file_id: usize,
    pub symbols: Vec<TSSymbol>,
}

impl TSVisitor {
    pub fn new(file_id: usize) -> Self {
        return Self {
            file_id,
            symbols: vec![],
        };
    }
}

impl Visit for TSVisitor {
    // export * from "./path";
    fn visit_export_all(&mut self, export_all: &ExportAll) {
        self.symbols.push(TSSymbol {
            file_id: self.file_id,
            symbol: TSSymbolData::ExportAll(FileReference::new(
                &export_all.src.value,
                &export_all.span,
            )),
        })
    }

    // export default <expr>;
    fn visit_export_default_expr(&mut self, export_default_expr: &swc_ecma_ast::ExportDefaultExpr) {
        self.symbols.push(TSSymbol {
            file_id: self.file_id,
            symbol: TSSymbolData::ExportDefaultExpr(export_default_expr.span.clone()),
        })
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
            swc_ecma_ast::Decl::Class(class_decl) => self.symbols.push(TSSymbol {
                file_id: self.file_id,
                symbol: TSSymbolData::ExportClassDecl(
                    class_decl.ident.sym.to_string(),
                    class_decl.class.span.clone(),
                ),
            }),
            swc_ecma_ast::Decl::Fn(fn_decl) => self.symbols.push(TSSymbol {
                file_id: self.file_id,
                symbol: TSSymbolData::ExportFnDecl(
                    fn_decl.ident.sym.to_string(),
                    fn_decl.function.span.clone(),
                ),
            }),
            swc_ecma_ast::Decl::Var(var_decl) => {
                for decl in &var_decl.decls {
                    if let swc_ecma_ast::Pat::Ident(ident) = &decl.name {
                        self.symbols.push(TSSymbol {
                            file_id: self.file_id,
                            symbol: TSSymbolData::ExportDecl(
                                ident.id.sym.to_string(),
                                decl.span.clone(),
                            ),
                        });
                    }
                }
            }
            swc_ecma_ast::Decl::TsEnum(ts_enum_decl) => self.symbols.push(TSSymbol {
                file_id: self.file_id,
                symbol: TSSymbolData::ExportEnumDecl(
                    ts_enum_decl.id.sym.to_string(),
                    ts_enum_decl.span.clone(),
                ),
            }),
            swc_ecma_ast::Decl::TsInterface(ts_interface_decl) => self.symbols.push(TSSymbol {
                file_id: self.file_id,
                symbol: TSSymbolData::ExportInterfaceDecl(
                    ts_interface_decl.id.sym.to_string(),
                    ts_interface_decl.span.clone(),
                ),
            }),
            swc_ecma_ast::Decl::TsTypeAlias(ts_type_alias_decl) => self.symbols.push(TSSymbol {
                file_id: self.file_id,
                symbol: TSSymbolData::ExportTypeAliasDecl(
                    ts_type_alias_decl.id.sym.to_string(),
                    ts_type_alias_decl.span.clone(),
                ),
            }),
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
                        Some(FileReference::new(&src.value, &src.span))
                    } else {
                        None
                    };

                    self.symbols.push(TSSymbol {
                        file_id: self.file_id,
                        symbol: TSSymbolData::ExportNamed(
                            orig,
                            exported,
                            named_export_specifier.span.clone(),
                            src,
                        ),
                    });
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
                self.symbols.push(TSSymbol {
                    file_id: self.file_id,
                    symbol: TSSymbolData::ExportDefaultClassDecl(
                        name,
                        class_decl.class.span.clone(),
                    ),
                })
            }
            swc_ecma_ast::DefaultDecl::Fn(fn_decl) => {
                let name = if let Some(ident) = &fn_decl.ident {
                    Some(ident.sym.to_string())
                } else {
                    None
                };
                self.symbols.push(TSSymbol {
                    file_id: self.file_id,
                    symbol: TSSymbolData::ExportDefaultFnDecl(name, fn_decl.function.span.clone()),
                })
            }
            swc_ecma_ast::DefaultDecl::TsInterfaceDecl(ts_interface_decl) => {
                self.symbols.push(TSSymbol {
                    file_id: self.file_id,
                    symbol: TSSymbolData::ExportDefaultInterfaceDecl(
                        ts_interface_decl.id.sym.to_string(),
                        ts_interface_decl.span.clone(),
                    ),
                })
            }
        }
    }

    fn visit_import_decl(&mut self, import_decl: &swc_ecma_ast::ImportDecl) {
        let src = FileReference::new(&import_decl.src.value, &import_decl.src.span);
        let type_only = import_decl.type_only;
        for spec in &import_decl.specifiers {
            match &spec {
                // import React from "react";
                &swc_ecma_ast::ImportSpecifier::Default(spec) => {
                    self.symbols.push(TSSymbol {
                        file_id: self.file_id,
                        symbol: TSSymbolData::ImportDefault(
                            spec.local.sym.to_string(),
                            spec.span,
                            src.clone(),
                            type_only,
                        ),
                    });
                }
                // import * as React from "react";
                &swc_ecma_ast::ImportSpecifier::Namespace(spec) => {
                    self.symbols.push(TSSymbol {
                        file_id: self.file_id,
                        symbol: TSSymbolData::ImportStar(
                            spec.local.sym.to_string(),
                            spec.span,
                            src.clone(),
                            type_only,
                        ),
                    });
                }
                // import { useState } from "react";
                &swc_ecma_ast::ImportSpecifier::Named(spec) => {
                    self.symbols.push(TSSymbol {
                        file_id: self.file_id,
                        symbol: TSSymbolData::ImportNamed(
                            spec.local.sym.to_string(),
                            spec.span,
                            src.clone(),
                            type_only || spec.is_type_only,
                        ),
                    });
                }
            }
        }
    }
}
