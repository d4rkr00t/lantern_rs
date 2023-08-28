use std::path::PathBuf;

use color_eyre::eyre::Result;

use lantern_parse_ts::parse_ts;

use swc_atoms::JsWord;
use swc_common::Span;
use swc_ecma_ast::ExportAll;
use swc_ecma_visit::Visit;

#[derive(Debug)]
struct TSSymbols {
    pub path: PathBuf,
    pub exports: Vec<Export>,
    pub imports: Vec<()>,
}

impl TSSymbols {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            exports: vec![],
            imports: vec![],
        }
    }
}

impl Visit for TSSymbols {
    // export * from "./path";
    fn visit_export_all(&mut self, export_all: &ExportAll) {
        self.exports.push(Export::All(FileReference::new(
            &export_all.src.value,
            &export_all.span,
        )))
    }

    // export default <expr>;
    fn visit_export_default_expr(&mut self, export_default_expr: &swc_ecma_ast::ExportDefaultExpr) {
        self.exports
            .push(Export::DefaultExpr(export_default_expr.span.clone()));
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
            swc_ecma_ast::Decl::Class(class_decl) => self.exports.push(Export::ClassDecl(
                class_decl.ident.sym.to_string(),
                class_decl.class.span.clone(),
            )),
            swc_ecma_ast::Decl::Fn(fn_decl) => self.exports.push(Export::FnDecl(
                fn_decl.ident.sym.to_string(),
                fn_decl.function.span.clone(),
            )),
            swc_ecma_ast::Decl::Var(var_decl) => {
                for decl in &var_decl.decls {
                    if let swc_ecma_ast::Pat::Ident(ident) = &decl.name {
                        self.exports
                            .push(Export::Decl(ident.id.sym.to_string(), decl.span.clone()));
                    }
                }
            }
            swc_ecma_ast::Decl::TsEnum(ts_enum_decl) => self.exports.push(Export::EnumDecl(
                ts_enum_decl.id.sym.to_string(),
                ts_enum_decl.span.clone(),
            )),
            swc_ecma_ast::Decl::TsInterface(ts_interface_decl) => {
                self.exports.push(Export::InterfaceDecl(
                    ts_interface_decl.id.sym.to_string(),
                    ts_interface_decl.span.clone(),
                ))
            }
            swc_ecma_ast::Decl::TsTypeAlias(ts_type_alias_decl) => {
                self.exports.push(Export::TypeAliasDecl(
                    ts_type_alias_decl.id.sym.to_string(),
                    ts_type_alias_decl.span.clone(),
                ))
            }
            _ => {}
        }
    }

    // export { default } from "./exports_default_decl";
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

                    self.exports.push(Export::Named(
                        orig,
                        exported,
                        named_export_specifier.span.clone(),
                        None,
                    ));
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
                self.exports.push(Export::DefaultClassDecl(
                    name,
                    class_decl.class.span.clone(),
                ))
            }
            swc_ecma_ast::DefaultDecl::Fn(fn_decl) => {
                let name = if let Some(ident) = &fn_decl.ident {
                    Some(ident.sym.to_string())
                } else {
                    None
                };
                self.exports
                    .push(Export::DefaultFnDecl(name, fn_decl.function.span.clone()))
            }
            swc_ecma_ast::DefaultDecl::TsInterfaceDecl(ts_interface_decl) => {
                self.exports.push(Export::DefaultInterfaceDecl(
                    ts_interface_decl.id.sym.to_string(),
                    ts_interface_decl.span.clone(),
                ))
            }
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Export {
    All(FileReference),
    ClassDecl(String, Span),
    Decl(String, Span),
    DefaultClassDecl(Option<String>, Span),
    DefaultDecl(String, Span),
    DefaultExpr(Span),
    DefaultFnDecl(Option<String>, Span),
    DefaultInterfaceDecl(String, Span),
    EnumDecl(String, Span),
    FnDecl(String, Span),
    InterfaceDecl(String, Span),
    TypeAliasDecl(String, Span),
    Named(String, Option<String>, Span, Option<FileReference>),
}

pub fn analyze(path: &PathBuf) -> Result<()> {
    let mut ts_s = TSSymbols::new(path.clone());
    let program = parse_ts(path)?;

    ts_s.visit_program(&program);

    println!("exports: {:#?}", ts_s.exports);

    return Ok(());
}
