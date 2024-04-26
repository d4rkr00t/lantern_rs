use oxc_span::Span;

#[derive(Debug, Clone)]
pub struct LNSymbol {
    pub module_id: usize,
    pub symbol: LNSymbolData,
}

impl LNSymbol {
    pub fn get_span(&self) -> &Span {
        match &self.symbol {
            LNSymbolData::ExportAll(file_ref) => &file_ref.span,
            LNSymbolData::ExportClassDecl(_, span) => span,
            LNSymbolData::ExportDecl(_, span) => span,
            LNSymbolData::ExportDefaultClassDecl(_, span) => span,
            LNSymbolData::ExportDefaultExpr(span) => span,
            LNSymbolData::ExportDefaultFnDecl(_, span) => span,
            LNSymbolData::ExportDefaultInterfaceDecl(_, span) => span,
            LNSymbolData::ExportEnumDecl(_, span) => span,
            LNSymbolData::ExportFnDecl(_, span) => span,
            LNSymbolData::ExportInterfaceDecl(_, span) => span,
            LNSymbolData::ExportTypeAliasDecl(_, span) => span,
            LNSymbolData::ExportNamed(_, _, span, _) => span,
            LNSymbolData::ImportDefault(_, span, _, _) => span,
            LNSymbolData::ImportStar(_, span, _, _) => span,
            LNSymbolData::ImportNamed(_, _, span, _, _) => span,
        }
    }

    pub fn get_name(&self) -> Option<&str> {
        match &self.symbol {
            LNSymbolData::ExportAll(_) => None,
            LNSymbolData::ExportClassDecl(name, _) => Some(name),
            LNSymbolData::ExportDecl(name, _) => Some(name),
            LNSymbolData::ExportDefaultClassDecl(name, _) => name.as_deref(),
            LNSymbolData::ExportDefaultExpr(_) => None,
            LNSymbolData::ExportDefaultFnDecl(name, _) => name.as_deref(),
            LNSymbolData::ExportDefaultInterfaceDecl(name, _) => Some(name),
            LNSymbolData::ExportEnumDecl(name, _) => Some(name),
            LNSymbolData::ExportFnDecl(name, _) => Some(name),
            LNSymbolData::ExportInterfaceDecl(name, _) => Some(name),
            LNSymbolData::ExportTypeAliasDecl(name, _) => Some(name),
            LNSymbolData::ExportNamed(_, name, _, _) => Some(name),
            LNSymbolData::ImportDefault(name, _, _, _) => Some(name),
            LNSymbolData::ImportStar(name, _, _, _) => Some(name),
            LNSymbolData::ImportNamed(name, _, _, _, _) => Some(name),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LNSymbolData {
    ExportAll(LNFileReference),

    ExportDefaultExpr(Span),
    ExportNamed(String, String, Span, Option<LNFileReference>),

    ExportDecl(String, Span),
    ExportFnDecl(String, Span),
    ExportClassDecl(String, Span),

    ExportEnumDecl(String, Span),
    ExportInterfaceDecl(String, Span),
    ExportTypeAliasDecl(String, Span),

    ExportDefaultClassDecl(Option<String>, Span),
    ExportDefaultFnDecl(Option<String>, Span),
    ExportDefaultInterfaceDecl(String, Span),

    ImportDefault(String, Span, LNFileReference, bool),
    ImportStar(String, Span, LNFileReference, bool),
    ImportNamed(String, String, Span, LNFileReference, bool),
}

#[derive(Debug, Clone)]
pub struct LNFileReference {
    pub module_id: usize,
    pub span: Span,
}

impl LNFileReference {
    pub fn new(module_id: usize, span: Span) -> Self {
        Self { module_id, span }
    }
}
