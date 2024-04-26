#[cfg(test)]
mod tests {
    use lantern_code_annotation::CodeAnnotation;
    use lantern_symbols_map::{build_symbols_map, symbol::LNSymbol, symbols_map::LNSymbolsMap};
    use lantern_testing::load_fixture;

    pub fn debug_symbol_span(symbol: &LNSymbol, sm: &mut LNSymbolsMap) {
        let path = sm.get_module_path(symbol.module_id).clone();
        let src = sm.get_module_source(symbol.module_id).to_string();
        let mut annotation = CodeAnnotation::new(path, src);
        let span = symbol.get_span();
        let span_line = sm.get_line_number_from_span(symbol.module_id, span);
        annotation.annotate("debug symbol".to_string(), span_line, span.clone());
        println!("{}", annotation.print());
    }

    #[test]
    fn imports_default_specifier() {
        let path_buf = load_fixture!("imports_default.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 1);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 7);
        assert_eq!(span.end, 12);

        assert_eq!(symbol.get_name().unwrap(), "React");
    }

    #[test]
    fn imports_namespace_specifier() {
        let path_buf = load_fixture!("imports_namespace_specifier.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 1);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 7);
        assert_eq!(span.end, 20);

        assert_eq!(symbol.get_name().unwrap(), "ReactDOM");
    }

    #[test]
    fn imports_named() {
        let path_buf = load_fixture!("imports_named.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 1);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 9);
        assert_eq!(span.end, 26);

        assert_eq!(symbol.get_name().unwrap(), "hello");
    }

    #[test]
    fn imports_declaration_specifier() {
        let path_buf = load_fixture!("imports_declaration_specifier.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 1);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 9);
        assert_eq!(span.end, 17);

        assert_eq!(symbol.get_name().unwrap(), "useState");
    }

    #[test]
    fn imports_declaration_specifier_as() {
        let path_buf = load_fixture!("imports_declaration_specifier_as.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 1);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 9);
        assert_eq!(span.end, 33);

        assert_eq!(symbol.get_name().unwrap(), "useSomething");
    }

    #[test]
    fn imports_type() {
        let path_buf = load_fixture!("imports_type.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 1);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 14);
        assert_eq!(span.end, 15);

        assert_eq!(symbol.get_name().unwrap(), "A");
    }

    #[test]
    fn imports_mixed_declaration_specifier_as_and_type() {
        let path_buf = load_fixture!("imports_mixed_declaration_specifier_as_and_type.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 2);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 9);
        assert_eq!(span.end, 16);

        assert_eq!(symbol.get_name().unwrap(), "Aa");

        let symbol = sm.symbols[1].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 18);
        assert_eq!(span.end, 24);

        assert_eq!(symbol.get_name().unwrap(), "C");
    }

    #[test]
    fn imports_type_external_package() {
        let path_buf = load_fixture!("imports_type_external_package.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 1);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 12);
        assert_eq!(span.end, 17);

        assert_eq!(symbol.get_name().unwrap(), "Hello");
    }

    #[test]
    fn imports_namespace_type_specifier() {
        let path_buf = load_fixture!("imports_namespace_type_specifier.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 1);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 12);
        assert_eq!(span.end, 23);

        assert_eq!(symbol.get_name().unwrap(), "Hello2");
    }

    #[test]
    fn imports_type_inside_declaration() {
        let path_buf = load_fixture!("imports_type_inside_declaration.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 1);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 9);
        assert_eq!(span.end, 16);

        assert_eq!(symbol.get_name().unwrap(), "Ab");
    }
}
