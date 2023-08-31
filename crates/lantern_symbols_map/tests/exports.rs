#[cfg(test)]
mod tests {
    use lantern_symbols_map::analyze;
    use lantern_testing::load_fixture;
    use std::path::PathBuf;

    #[test]
    fn exports_default_expr() {
        let path_buf = load_fixture!("exports_default_expr.ts");
        analyze(&path_buf).unwrap();
    }

    #[test]
    fn exports_all() {
        let path_buf = load_fixture!("exports_all.ts");
        analyze(&path_buf).unwrap();
    }

    #[test]
    fn exports_decl() {
        let path_buf = load_fixture!("exports_decl.ts");
        analyze(&path_buf).unwrap();
    }

    #[test]
    fn exports_default_decl() {
        let path_buf = load_fixture!("exports_default_decl.ts");
        analyze(&path_buf).unwrap();
    }

    #[test]
    fn exports_named() {
        let path_buf = load_fixture!("exports_named.ts");
        analyze(&path_buf).unwrap();
    }
}
