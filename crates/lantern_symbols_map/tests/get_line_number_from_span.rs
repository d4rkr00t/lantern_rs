#[cfg(test)]
mod tests {
    use lantern_symbols_map::build_symbols_map;
    use lantern_testing::load_fixture;

    #[test]
    fn end_of_file() {
        let path_buf = load_fixture!("exports_decl.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        let symbol = sm.symbols[9].clone();
        let span = symbol.get_span();
        let line = sm.get_line_number_from_span(symbol.module_id, span);
        assert_eq!(line, 16);
    }

    #[test]
    fn multiple_var_decl() {
        let path_buf = load_fixture!("exports_decl.ts");
        let ep = vec![path_buf];
        let mut sm = build_symbols_map(&ep).unwrap();
        let symbol = sm.symbols[5].clone();
        let span = symbol.get_span();
        let line = sm.get_line_number_from_span(symbol.module_id, span);
        assert_eq!(line, 8);
    }
}
