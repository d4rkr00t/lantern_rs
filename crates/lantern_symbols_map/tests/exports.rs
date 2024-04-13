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
    fn exports_default_expr() {
        let path_buf = load_fixture!("exports_default_expr.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 1);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 0);
        assert_eq!(span.end, 18);

        assert_eq!(symbol.get_name().is_none(), true);
    }

    #[test]
    fn exports_all() {
        let path_buf = load_fixture!("exports_all.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 2);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 0);
        assert_eq!(span.end, 39);

        assert_eq!(symbol.get_name().is_none(), true);
    }

    #[test]
    fn exports_decl_function() {
        let path_buf = load_fixture!("exports_decl_function.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 1);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 7);
        assert_eq!(span.end, 26);

        assert_eq!(symbol.get_name().unwrap(), "hello");
    }

    #[test]
    fn exports_decl_arrow_function() {
        let path_buf = load_fixture!("exports_decl_arrow_function.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 1);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 13);
        assert_eq!(span.end, 19);

        assert_eq!(symbol.get_name().unwrap(), "hello2");
    }

    #[test]
    fn exports_decl_const() {
        let path_buf = load_fixture!("exports_decl_const.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 1);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 13);
        assert_eq!(span.end, 14);

        assert_eq!(symbol.get_name().unwrap(), "a");
    }

    #[test]
    fn exports_decl_let() {
        let path_buf = load_fixture!("exports_decl_let.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 1);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 11);
        assert_eq!(span.end, 12);

        assert_eq!(symbol.get_name().unwrap(), "b");
    }

    #[test]
    fn exports_decl_var() {
        let path_buf = load_fixture!("exports_decl_var.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 2);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 11);
        assert_eq!(span.end, 12);

        assert_eq!(symbol.get_name().unwrap(), "c");

        let symbol = sm.symbols[1].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 20);
        assert_eq!(span.end, 21);

        assert_eq!(symbol.get_name().unwrap(), "d");
    }

    #[test]
    fn exports_decl_class() {
        let path_buf = load_fixture!("exports_decl_class.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 1);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 7);
        assert_eq!(span.end, 21);

        assert_eq!(symbol.get_name().unwrap(), "Hello");
    }

    #[test]
    fn exports_decl_type() {
        let path_buf = load_fixture!("exports_decl_type.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 1);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 7);
        assert_eq!(span.end, 18);

        assert_eq!(symbol.get_name().unwrap(), "A");
    }

    #[test]
    fn exports_decl_enum() {
        let path_buf = load_fixture!("exports_decl_enum.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 1);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 7);
        assert_eq!(span.end, 16);

        assert_eq!(symbol.get_name().unwrap(), "B");
    }

    #[test]
    fn exports_decl_interface() {
        let path_buf = load_fixture!("exports_decl_interface.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 1);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 7);
        assert_eq!(span.end, 22);

        assert_eq!(symbol.get_name().unwrap(), "IC");
    }

    #[test]
    fn exports_decl() {
        let path_buf = load_fixture!("exports_decl.ts");
        let ep = vec![path_buf];
        let sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 10);
    }

    #[test]
    fn exports_default_decl() {
        let path_buf = load_fixture!("exports_default_decl.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 1);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 15);
        assert_eq!(span.end, 23);

        assert_eq!(symbol.get_name().is_none(), true);
    }

    #[test]
    fn exports_named_default() {
        let path_buf = load_fixture!("exports_named_default.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 2);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 9);
        assert_eq!(span.end, 16);

        assert_eq!(symbol.get_name().unwrap(), "default");
    }

    #[test]
    fn exports_named_multiple() {
        let path_buf = load_fixture!("exports_named_multiple.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 12);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 9);
        assert_eq!(span.end, 10);

        assert_eq!(symbol.get_name().unwrap(), "a");

        let symbol = sm.symbols[1].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 12);
        assert_eq!(span.end, 13);

        assert_eq!(symbol.get_name().unwrap(), "b");
    }

    #[test]
    fn exports_named_aliased() {
        let path_buf = load_fixture!("exports_named_aliased.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 12);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 9);
        assert_eq!(span.end, 16);

        assert_eq!(symbol.get_name().unwrap(), "a2");

        let symbol = sm.symbols[1].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 18);
        assert_eq!(span.end, 25);

        assert_eq!(symbol.get_name().unwrap(), "b2");
    }

    #[test]
    fn exports_named() {
        let path_buf = load_fixture!("exports_named.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        assert_eq!(sm.symbols.len(), 1);

        let symbol = sm.symbols[0].clone();
        let span = symbol.get_span();
        debug_symbol_span(&symbol, &mut sm);

        assert_eq!(span.start, 23);
        assert_eq!(span.end, 24);

        assert_eq!(symbol.get_name().unwrap(), "c");
    }
}
