#[cfg(test)]
mod tests {
    use lantern_symbols_map::build;
    use lantern_testing::load_fixture;
    use std::path::PathBuf;

    #[test]
    fn exports_default_expr() {
        let path_buf = load_fixture!("exports_default_expr.ts");
        build(&path_buf).unwrap();
    }

    #[test]
    fn exports_all() {
        let path_buf = load_fixture!("exports_all.ts");
        build(&path_buf).unwrap();
    }

    #[test]
    fn exports_decl() {
        let path_buf = load_fixture!("exports_decl.ts");
        build(&path_buf).unwrap();
    }

    #[test]
    fn exports_default_decl() {
        let path_buf = load_fixture!("exports_default_decl.ts");
        build(&path_buf).unwrap();
    }

    #[test]
    fn exports_named() {
        let path_buf = load_fixture!("exports_named.ts");
        build(&path_buf).unwrap();
    }

    #[test]
    fn exports_type_alias() {
        let path_buf = load_fixture!("exports_type_alias.ts");
        build(&path_buf).unwrap();
    }
}
