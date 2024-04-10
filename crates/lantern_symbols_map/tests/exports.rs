#[cfg(test)]
mod tests {
    use lantern_symbols_map::build_symbols_map;
    use lantern_testing::load_fixture;

    #[test]
    fn exports_default_expr() {
        let path_buf = load_fixture!("exports_default_expr.ts");
        let ep = vec![path_buf];
        build_symbols_map(&ep).unwrap();
    }

    #[test]
    fn exports_all() {
        let path_buf = load_fixture!("exports_all.ts");
        let ep = vec![path_buf];
        build_symbols_map(&ep).unwrap();
    }

    #[test]
    fn exports_decl() {
        let path_buf = load_fixture!("exports_decl.ts");
        let ep = vec![path_buf];
        build_symbols_map(&ep).unwrap();
    }

    #[test]
    fn exports_default_decl() {
        let path_buf = load_fixture!("exports_default_decl.ts");
        let ep = vec![path_buf];
        build_symbols_map(&ep).unwrap();
    }

    #[test]
    fn exports_named() {
        let path_buf = load_fixture!("exports_named.ts");
        let ep = vec![path_buf];
        build_symbols_map(&ep).unwrap();
    }

    #[test]
    fn exports_type_alias() {
        let path_buf = load_fixture!("exports_type_alias.ts");
        let ep = vec![path_buf];
        build_symbols_map(&ep).unwrap();
    }
}
