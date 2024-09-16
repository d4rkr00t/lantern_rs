#[cfg(test)]
mod tests {
    use lantern_symbols_map::build_symbols_map;
    use lantern_testing::load_fixture;
    use lantern_unused_exports::find_unused_exports;

    #[test]
    fn simple_import_as() {
        let path_buf = load_fixture!("used_import_as/index.ts");
        let ep = vec![path_buf];
        let sm = build_symbols_map(&ep).unwrap();
        let unused_exports = find_unused_exports(&sm).unwrap();
        assert_eq!(unused_exports.len(), 0);
    }

    #[test]
    fn simple_named() {
        let path_buf = load_fixture!("used_simple_named/index.ts");
        let ep = vec![path_buf];
        let sm = build_symbols_map(&ep).unwrap();
        let unused_exports = find_unused_exports(&sm).unwrap();
        assert_eq!(unused_exports.len(), 0);
    }

    #[test]
    fn simple_named_transitive() {
        let path_buf = load_fixture!("used_simple_named_transitive/index.ts");
        let ep = vec![path_buf];
        let sm = build_symbols_map(&ep).unwrap();
        let unused_exports = find_unused_exports(&sm).unwrap();
        assert_eq!(unused_exports.len(), 0);
    }

    #[test]
    fn named_as_transitive() {
        let path_buf = load_fixture!("used_named_as_transitive/index.ts");
        let ep = vec![path_buf];
        let sm = build_symbols_map(&ep).unwrap();
        let unused_exports = find_unused_exports(&sm).unwrap();
        assert_eq!(unused_exports.len(), 0);
    }

    #[test]
    fn default_export() {
        let path_buf = load_fixture!("used_default_export/index.ts");
        let ep = vec![path_buf];
        let sm = build_symbols_map(&ep).unwrap();
        println!("{:#?}", sm);
        let unused_exports = find_unused_exports(&sm).unwrap();
        assert_eq!(unused_exports.len(), 0);
    }

    #[test]
    fn deep_chain() {
        let path_buf = load_fixture!("used_deep_chain/index.ts");
        let ep = vec![path_buf];
        let sm = build_symbols_map(&ep).unwrap();
        let unused_exports = find_unused_exports(&sm).unwrap();
        assert_eq!(unused_exports.len(), 0);
    }

    // #[test]
    // fn export_star() {
    //     let path_buf = load_fixture!("used_export_star/index.ts");
    //     let ep = vec![path_buf];
    //     let sm = build_symbols_map(&ep).unwrap();
    //     let unused_exports = find_unused_exports(&sm).unwrap();
    //     assert_eq!(unused_exports.len(), 0);
    // }
}
